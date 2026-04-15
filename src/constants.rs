pub mod error_codes {
  pub const COMMON_MODULE: i32 = 0;
  pub const AUTH_MODULE: i32 = 100_000;
  pub const USER_MODULE: i32 = 200_000;
  pub const SITE_MODULE: i32 = 300_000;
  pub const COMMENT_MODULE: i32 = 400_000;
  pub const MODERATION_MODULE: i32 = 500_000;
}

pub mod rbac {
  pub mod codes {
    pub const SITE_MANAGE: &str = "site.manage";
    pub const COMMENT_MODERATE: &str = "comment.moderate";
    pub const COMMENT_DELETE_ANY: &str = "comment.delete.any";
    pub const OAUTH_PROVIDER_MANAGE: &str = "oauth_provider.manage";
    pub const MODERATION_PROVIDER_MANAGE: &str = "moderation_provider.manage";
  }

  pub mod roles {
    pub const SUPER_ADMIN: &str = "super_admin";
    pub const SITE_ADMIN: &str = "site_admin";
    pub const MODERATOR: &str = "moderator";
  }

  pub const SYSTEM_PERMISSIONS: [(&str, &str); 5] = [
    (codes::SITE_MANAGE, "Manage sites"),
    (codes::COMMENT_MODERATE, "Moderate comments"),
    (codes::COMMENT_DELETE_ANY, "Delete any comment"),
    (codes::OAUTH_PROVIDER_MANAGE, "Manage OAuth providers"),
    (
      codes::MODERATION_PROVIDER_MANAGE,
      "Manage moderation providers",
    ),
  ];

  pub const SYSTEM_ROLES: [(&str, Option<&str>); 3] = [
    (roles::SUPER_ADMIN, Some("System super administrator")),
    (roles::SITE_ADMIN, Some("Site administrator")),
    (roles::MODERATOR, Some("Comment moderator")),
  ];

  pub const SUPER_ADMIN_PERMISSION_CODES: [&str; 5] = [
    codes::SITE_MANAGE,
    codes::COMMENT_MODERATE,
    codes::COMMENT_DELETE_ANY,
    codes::OAUTH_PROVIDER_MANAGE,
    codes::MODERATION_PROVIDER_MANAGE,
  ];

  pub const SITE_ADMIN_PERMISSION_CODES: [&str; 3] = [
    codes::SITE_MANAGE,
    codes::OAUTH_PROVIDER_MANAGE,
    codes::MODERATION_PROVIDER_MANAGE,
  ];

  pub const MODERATOR_PERMISSION_CODES: [&str; 2] =
    [codes::COMMENT_MODERATE, codes::COMMENT_DELETE_ANY];
}

pub mod moderation {
  pub mod provider_kind {
    pub const LLM: &str = "llm";
    pub const AKISMET: &str = "akismet";
  }

  pub const LLM_DEFAULT_PROMPT: &str = r#"You are a comment moderation system.
Review the user comment and return only JSON.

Rules:

score is a float between 0 and 1 where higher means more risky
decision must be one of: allow, review, reject
reject for obvious spam, scams, malicious links, abusive or illegal content
review for borderline, ambiguous, or uncertain content
allow for normal comments
{{rule}}

Return exactly:
{"decision":"allow|review|reject","reason":"short reason","score":0.0}"#;
}
