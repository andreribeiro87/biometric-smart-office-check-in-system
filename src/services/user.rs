use uuid::Uuid;
use crate::repositories::user::UserRepo;
use crate::models::{User, CreateUserRequest, UpdateUserRequest};

#[derive(Clone)]
pub struct UserService {
    repo: UserRepo,
}

impl UserService {
    pub fn new(repo: UserRepo) -> Self { Self { repo } }

    pub async fn list_users(&self) -> Result<Vec<User>, anyhow::Error> {
        Ok(self.repo.list().await?)
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, anyhow::Error> {
        Ok(self.repo.get(id).await?)
    }

    pub async fn create_user(&self, req: CreateUserRequest)
                             -> Result<User, anyhow::Error> {
        // aquí pode haver regras de negócio antes de criar
        Ok(self.repo.create(req).await?)
    }

    pub async fn update_user(&self, id: Uuid, req: UpdateUserRequest)
                             -> Result<User, anyhow::Error> {
        Ok(self.repo.update(id, req).await?)
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), anyhow::Error> {
        self.repo.delete(id).await?;
        Ok(())
    }
}
