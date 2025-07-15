use crate::config::DbPool;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use crate::schema::users;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepo {
    pool: DbPool,
}

impl UserRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<User>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .map_err(|_| diesel::result::Error::NotFound)?;
        tokio::task::spawn_blocking(move || users::table.load(&mut conn))
            .await
            .unwrap()
    }

    pub async fn get(&self, id: Uuid) -> Result<User, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .map_err(|_| diesel::result::Error::NotFound)?;

        tokio::task::spawn_blocking(move || users::table.find(id).get_result(&mut conn))
            .await
            .unwrap()
    }

    pub async fn create(&self, data: CreateUserRequest) -> Result<User, diesel::result::Error> {
        let mut conn = self.pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        tokio::task::spawn_blocking(move || {
            diesel::insert_into(users::table)
                .values(&data)
                .get_result(&mut conn)
        })
        .await
        .unwrap()
    }

    pub async fn update(
        &self,
        id: Uuid,
        data: UpdateUserRequest,
    ) -> Result<User, diesel::result::Error> {
        let mut conn = self.pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        tokio::task::spawn_blocking(move || {
            diesel::update(users::table.find(id))
                .set(&data)
                .get_result(&mut conn)
        })
        .await
        .unwrap()
    }

    pub async fn delete(&self, id: Uuid) -> Result<usize, diesel::result::Error> {
        let mut conn = self.pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        tokio::task::spawn_blocking(move || {
            diesel::delete(users::table.find(id)).execute(&mut conn)
        })
        .await
        .unwrap()
    }
}
