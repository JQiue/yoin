mod comment;
mod oauth;
mod site;
mod user;

use crate::repository::Repository;

pub struct AppService {
  pub repo: Repository,
}
