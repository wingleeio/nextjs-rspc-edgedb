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
    pub last_accessed_at: DateTime<Utc>,
    pub user: SessionUser,
}

#[derive(Debug, Queryable, Serialize, Type)]
pub struct SessionWithMetadata {
    pub id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub last_accessed_at: DateTime<Utc>,
    pub os_name: Option<String>,
    pub os_version: Option<String>,
    pub browser_name: Option<String>,
    pub browser_version: Option<String>,
    pub is_current: bool,
}

impl Auth {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn create_session(
        &self,
        user_id: Uuid,
        os_name: Option<String>,
        os_version: Option<String>,
        browser_name: Option<String>,
        browser_version: Option<String>,
    ) -> Result<Uuid, Box<dyn Error>> {
        let args = (user_id, os_name, os_version, browser_name, browser_version);
        let query = r#"
            with 
                os_name := <optional str>$1,
                os_version := <optional str>$2,
                browser_name := <optional str>$3,
                browser_version := <optional str>$4,
                deleted := (
                    delete Session
                    filter .user = (select User filter .id = <uuid>$0) and .expires_at < datetime_of_statement()
                )
            select (insert Session {
                user := (select User filter .id = <uuid>$0),
                os_name := os_name,
                os_version := os_version,
                browser_name := browser_name,
                browser_version := browser_version
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

    pub async fn get_sessions(
        &self,
        user_id: Uuid,
        session_id: Uuid,
    ) -> Result<Vec<SessionWithMetadata>, Box<dyn Error>> {
        let args = (user_id, session_id);
        let query = r#"
            with
                current_session_id := <uuid>$1,
                deleted := (
                    delete Session
                    filter .user = (select User filter .id = <uuid>$0) and .expires_at < datetime_of_statement()
                )
            select Session {
                id,
                expires_at,
                last_accessed_at,
                os_name,
                os_version,
                browser_name,
                browser_version,
                is_current := .id = current_session_id
            }
            filter .user = <User><uuid>$0
        "#;
        let result: Vec<SessionWithMetadata> = self.client.query(query, &(args)).await?;
        Ok(result)
    }

    pub async fn get_session(&self, session_id: Uuid) -> Result<Session, Box<dyn Error>> {
        let args = (session_id,);
        let query = r#"
            with
                u := (
                    update Session
                    filter .id = <uuid>$0
                    set {
                        last_accessed_at := datetime_of_statement()
                    }
                )
            select u {
                id,
                expires_at,
                last_accessed_at,
                user: {
                    id,
                    email,
                    first_name,
                    last_name
                }
            }
            
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
