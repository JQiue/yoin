use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};

use crate::entity::{
  prelude::{Sites, Users},
  sites,
  users::{self},
};

pub trait UserRepo {
  async fn exists_any(conn: &DatabaseConnection) -> Result<bool, DbErr>;
  async fn exists_by_email(email: &str, conn: &DatabaseConnection) -> Result<bool, DbErr>;
  async fn find_by_id(
    user_id: i64,
    conn: &DatabaseConnection,
  ) -> Result<Option<users::Model>, DbErr>;
  async fn find_by_email(
    email: &str,
    conn: &DatabaseConnection,
  ) -> Result<Option<users::Model>, DbErr>;
}

impl UserRepo for Users {
  async fn exists_any(conn: &DatabaseConnection) -> Result<bool, DbErr> {
    Ok(Self::find().all(conn).await?.is_empty())
  }

  async fn exists_by_email(email: &str, conn: &DatabaseConnection) -> Result<bool, DbErr> {
    let user = Self::find()
      .filter(users::Column::Email.eq(email))
      .one(conn)
      .await?;
    Ok(user.is_some())
  }

  async fn find_by_id(
    user_id: i64,
    conn: &DatabaseConnection,
  ) -> Result<Option<users::Model>, DbErr> {
    Ok(
      Self::find()
        .filter(users::Column::Id.eq(user_id))
        .one(conn)
        .await?,
    )
  }

  async fn find_by_email(
    email: &str,
    conn: &DatabaseConnection,
  ) -> Result<Option<users::Model>, DbErr> {
    Ok(
      Self::find()
        .filter(users::Column::Email.eq(email))
        .one(conn)
        .await?,
    )
  }
}

pub trait SiteRepo {
  async fn find_all(conn: &DatabaseConnection) -> Result<Vec<sites::Model>, DbErr>;
}

impl SiteRepo for Sites {
  async fn find_all(conn: &DatabaseConnection) -> Result<Vec<sites::Model>, DbErr> {
    Self::find().all(conn).await
  }
}
