use helpers::hash::md5;

pub fn generate_avatar(email: &str) -> String {
  format!("https://cravatar.cn/avatar/{}", md5(email))
}
