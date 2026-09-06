use std::{collections::HashMap, hash::Hash};

use helpers::hash::md5;
use migration::prelude::{DateTime, Utc};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::error::AppError;

pub fn generate_avatar(email: &str) -> String {
  format!("https://cravatar.cn/avatar/{}", md5(email))
}

pub struct RateLimiter<T> {
  cache: Mutex<HashMap<T, DateTime<Utc>>>,
}

impl<T> Default for RateLimiter<T> {
  fn default() -> Self {
    Self {
      cache: Default::default(),
    }
  }
}

impl<T: Eq + PartialEq + Hash> RateLimiter<T> {
  pub async fn check_rate_limit(&self, key: T, limit_seconds: i64) -> bool {
    let mut cache = self.cache.lock().await;
    let now = Utc::now();
    let Some(last_comment_time) = cache.get(&key) else {
      cache.insert(key, now);
      return false;
    };

    let elapsed = now.signed_duration_since(*last_comment_time).num_seconds();
    if elapsed < limit_seconds {
      return true;
    }

    cache.insert(key, now);
    false
  }
}

pub enum OAuthProvider {
  Github,
  QQ,
}

#[derive(Debug)]
pub struct OauthProfile {
  pub id: String,
  pub nickname: String,
  pub avatar: String,
  pub email: Option<String>,
}

#[derive(Default)]
pub struct OauthConfig<'a> {
  pub client_id: &'a str,
  pub client_secret: &'a str,
  pub redirect_uri: &'a str,
  pub state: &'a str,
  pub code: &'a str,
}

pub struct OauthService<'a> {
  pub provider: OAuthProvider,
  pub config: OauthConfig<'a>,
}

impl<'a> OauthService<'a> {
  pub fn new(provider: &str) -> Result<Self, AppError> {
    let provider = match provider {
      "github" => OAuthProvider::Github,
      "qq" => OAuthProvider::QQ,
      _ => {
        return Err(AppError::invalid_oauth_provider(
          "invalid oauth provider".to_string(),
        ));
      }
    };
    Ok(Self {
      provider,
      config: OauthConfig::default(),
    })
  }

  fn authorize_endpoint(&self) -> &'static str {
    match self.provider {
      OAuthProvider::Github => "https://github.com/login/oauth/authorize",
      OAuthProvider::QQ => "https://graph.qq.com/oauth2.0/authorize",
    }
  }

  fn token_endpoint(&self) -> &'static str {
    match self.provider {
      OAuthProvider::Github => "https://github.com/login/oauth/access_token",
      OAuthProvider::QQ => "https://graph.qq.com/oauth2.0/token",
    }
  }

  fn userinfo_endpoint(&self) -> &'static str {
    match self.provider {
      OAuthProvider::Github => "https://api.github.com/user",
      OAuthProvider::QQ => "https://graph.qq.com/user/get_user_info",
    }
  }

  pub fn authorize_url(&self) -> Result<String, AppError> {
    let mut u = url::Url::parse(self.authorize_endpoint()).unwrap();
    u.query_pairs_mut()
      .append_pair("client_id", self.config.client_id)
      .append_pair("state", self.config.state)
      .append_pair("redirect_uri", self.config.redirect_uri);
    match self.provider {
      OAuthProvider::Github => {}
      OAuthProvider::QQ => {
        u.query_pairs_mut().append_pair("response_type", "code");
      }
    }
    Ok(u.to_string())
  }

  pub fn exchange_code(&self) -> String {
    #[derive(Debug, Deserialize)]
    struct AccessToken {
      access_token: String,
    }
    let mut client = ureq::post(self.token_endpoint())
      .query("client_id", self.config.client_id)
      .query("client_secret", self.config.client_secret)
      .query("code", self.config.code)
      .query("redirect_uri", self.config.redirect_uri);
    client = match self.provider {
      OAuthProvider::Github => client.header("Accept", "application/json"),
      OAuthProvider::QQ => client
        .query("grant_type", "authorization_code")
        .query("fmt", "json"),
    };
    let body = client
      .send("")
      .unwrap()
      .body_mut()
      .read_to_string()
      .unwrap();
    let token: AccessToken = serde_json::from_str(&body).unwrap();
    token.access_token
  }

  pub fn fetch_profile(&self, access_token: String) -> Result<OauthProfile, AppError> {
    #[derive(Debug, Deserialize)]
    struct GithubUser {
      id: String,
      login: String,
      avatar_url: String,
      email: String,
    }

    match self.provider {
      OAuthProvider::Github => {
        let body = ureq::get(self.userinfo_endpoint())
          .header("Authorization", format!("bearer {}", access_token))
          .call()
          .unwrap()
          .body_mut()
          .read_to_string()
          .unwrap();
        let profile = serde_json::from_str::<GithubUser>(&body).unwrap();
        Ok(OauthProfile {
          id: profile.id,
          nickname: profile.login,
          avatar: profile.avatar_url,
          email: Some(profile.email),
        })
      }
      OAuthProvider::QQ => {
        let client =
          ureq::get("https://graph.qq.com/oauth2.0/me").query("access_token", &access_token);
        let body = client.call().unwrap().body_mut().read_to_string().unwrap();
        #[derive(Debug, Deserialize)]
        struct QQOpenID {
          client_id: String,
          openid: String,
        }
        let openid: QQOpenID = serde_json::from_str(&body).unwrap();
        let client = ureq::get(self.userinfo_endpoint())
          .query("access_token", &access_token)
          .query("oauth_consumer_key", &openid.client_id)
          .query("openid", &openid.openid);
        let body = client.call().unwrap().body_mut().read_to_string().unwrap();
        #[derive(Debug, Deserialize)]
        struct QQUser {
          nickname: String,
          figureurl: String,
        }
        let profile = serde_json::from_str::<QQUser>(&body).unwrap();
        Ok(OauthProfile {
          id: openid.openid,
          nickname: profile.nickname,
          avatar: profile.figureurl,
          email: None,
        })
      }
    }
  }
}
