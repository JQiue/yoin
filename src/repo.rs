use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entity::{
  prelude::Users,
  users::{self, Model},
};

pub trait UserRepo {
  async fn is_first_user(conn: &DatabaseConnection) -> Result<bool, DbErr>;
  async fn has_user_by_email(email: &str, conn: &DatabaseConnection) -> Result<bool, DbErr>;
  async fn get_user_by_email(
    email: &str,
    conn: &DatabaseConnection,
  ) -> Result<Option<Model>, DbErr>;
}

impl UserRepo for Users {
  async fn has_user_by_email(email: &str, conn: &DatabaseConnection) -> Result<bool, DbErr> {
    let user = Users::find()
      .filter(users::Column::Email.eq(email))
      .one(conn)
      .await?;
    Ok(user.is_some())
  }

  async fn is_first_user(conn: &DatabaseConnection) -> Result<bool, DbErr> {
    Ok(Users::find().all(conn).await?.is_empty())
  }

  async fn get_user_by_email(
    email: &str,
    conn: &DatabaseConnection,
  ) -> Result<Option<Model>, DbErr> {
    Ok(
      Users::find()
        .filter(users::Column::Email.eq(email))
        .one(conn)
        .await?,
    )
  }
}
