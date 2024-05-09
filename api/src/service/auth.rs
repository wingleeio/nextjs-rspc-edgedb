use chrono::{DateTime, Utc};
use edgedb_protocol::model::Uuid;
use edgedb_tokio::{Client, Queryable};
use serde::Serialize;
use specta::Type;
use std::{error::Error, sync::Arc};

pub struct Auth {
    client: Arc<Client>,
}

#[derive(Debug, Queryable, Serialize, Type)]
pub struct SessionUser {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Queryable, Serialize, Type)]
pub struct Session {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub user: SessionUser,
}

impl Auth {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn create_session(&self, user_id: Uuid) -> Result<Uuid, Box<dyn Error>> {
        let args = (user_id,);
        let query = r#"
            with deleted := (
                delete Session
                filter .user = (select User filter .id = <uuid>$0) and .expires_at < datetime_of_statement()
            )
            select (insert Session {
                user := (select User filter .id = <uuid>$0)
            }) .id
        "#;
        let result: Uuid = self.client.query_required_single(query, &(args)).await?;
        Ok(result)
    }

    pub async fn invalidate_session(&self, session_id: Uuid) -> Result<(), Box<dyn Error>> {
        let args = (session_id,);
        let query = r#"
            delete Session
            filter .id = <uuid>$0
        "#;
        self.client.execute(query, &(args)).await?;
        Ok(())
    }

    pub async fn get_session(&self, session_id: Uuid) -> Result<Session, Box<dyn Error>> {
        let args = (session_id,);
        let query = r#"
            select Session {
                id,
                expires_at,
                user: {
                    id,
                    email,
                    first_name,
                    last_name
                }
            }
            filter .id = <uuid>$0
        "#;
        let result: Session = self.client.query_required_single(query, &(args)).await?;
        Ok(result)
    }

    pub async fn validate_session(&self, session_id: Uuid) -> Result<Session, Box<dyn Error>> {
        let session = self.get_session(session_id).await?;

        if session.expires_at < Utc::now() {
            self.invalidate_session(session_id).await?;
            return Err("Session expired".into());
        }
        Ok(session)
    }
}
