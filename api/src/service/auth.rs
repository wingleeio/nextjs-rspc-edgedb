use edgedb_protocol::model::{Datetime, Uuid};
use edgedb_tokio::{Client, Queryable};
use std::{error::Error, sync::Arc, time::SystemTime};

struct Auth {
    client: Arc<Client>,
}

#[derive(Debug, Queryable)]
struct Session {
    id_str: String,
    expires_at: Datetime,
    user_id_str: String,
}

impl Auth {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn create_session(&self, user_id: Uuid) -> Result<String, Box<dyn Error>> {
        self.delete_expired_sessions(user_id).await?;
        let args = (user_id,);
        let query = r#"
            insert Session {
                user := (select User filter .id <uuid>$0)
            }
            returning { <str>.id }
        "#;
        let result: String = self.client.query_required_single(query, &(args)).await?;
        Ok(result)
    }

    pub async fn delete_expired_sessions(&self, user_id: Uuid) -> Result<(), Box<dyn Error>> {
        let args = (user_id,);
        let query = r#"
            delete Session
            filter .user = (select User filter .id <uuid>$0)
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
                user_id_str := <str>.user.id
            }
            filter .id = <uuid>$0
        "#;
        let result: Session = self.client.query_required_single(query, &(args)).await?;
        Ok(result)
    }

    pub async fn validate_session(&self, session_id: Uuid) -> Result<Session, Box<dyn Error>> {
        let session = self.get_session(session_id).await?;
        let now = Datetime::try_from(SystemTime::now()).unwrap();
        if session.expires_at < now {
            self.invalidate_session(session_id).await?;
            return Err("Session expired".into());
        }
        Ok(session)
    }
}
