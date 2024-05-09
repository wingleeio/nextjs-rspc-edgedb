use chrono::{DateTime, Utc};
use edgedb_protocol::model::Uuid;
use edgedb_tokio::{Client, Queryable};
use std::{error::Error, sync::Arc};

pub struct Auth {
    client: Arc<Client>,
}

#[derive(Debug, Queryable)]
pub struct Session {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub user_id: Uuid,
}

impl Auth {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn create_session(&self, user_id: Uuid) -> Result<Uuid, Box<dyn Error>> {
        self.delete_expired_sessions(user_id).await?;
        let args = (user_id,);
        let query = r#"
            select (insert Session {
                user := (select User filter .id = <uuid>$0)
            }) .id
        "#;
        let result: Uuid = self.client.query_required_single(query, &(args)).await?;
        Ok(result)
    }

    pub async fn delete_expired_sessions(&self, user_id: Uuid) -> Result<(), Box<dyn Error>> {
        let args = (user_id,);
        let query = r#"
            delete Session
            filter .user = (select User filter .id = <uuid>$0)
            filter .expires < now()
        "#;
        self.client.execute(query, &(args)).await?;
        Ok(())
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
                id_str,
                expires_at,
                user_id := .user.id
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
