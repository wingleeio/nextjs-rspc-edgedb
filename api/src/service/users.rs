use std::{error::Error, sync::Arc};

use edgedb_tokio::{Client, Queryable};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

pub struct Users {
    client: Arc<Client>,
}

#[derive(Debug, Queryable, Clone, Deserialize, Serialize, Type)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hashed_password: String,
}

impl Users {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, Box<dyn Error>> {
        let args = (email,);
        let query = r#"
            select User {
                id,
                first_name,
                last_name,
                email,
                hashed_password
            }
            filter .email = <str>$0
        "#;
        let user: User = self.client.query_required_single(query, &(args)).await?;
        Ok(user)
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, Box<dyn Error>> {
        let args = (user_id,);
        let query = r#"
            select User {
                id,
                first_name,
                last_name,
                email,
                hashed_password
            }
            filter .id = <uuid>$0
        "#;
        let user: User = self.client.query_required_single(query, &(args)).await?;
        Ok(user)
    }

    pub async fn create_user(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        hashed_password: &str,
    ) -> Result<User, Box<dyn Error>> {
        let args = (first_name, last_name, email, hashed_password);
        let query = r#"
            select (insert User {
                first_name := <str>$0,
                last_name := <str>$1,
                email := <str>$2,
                hashed_password := <str>$3
            }) { 
                id,
                first_name,
                last_name,
                email,
                hashed_password
            }
        "#;
        let user: User = self.client.query_required_single(query, &(args)).await?;
        Ok(user)
    }
}
