use edgedb_protocol::model::Uuid;
use serde::Serialize;
use specta::Type;
use std::{error::Error, sync::Arc};

use edgedb_tokio::{Client, Queryable};

pub struct Blogs {
    client: Arc<Client>,
}

#[derive(Debug, Queryable, Serialize, Type)]
pub struct Blog {
    pub id: Uuid,
    pub title: String,
}

impl Blogs {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn get_blogs_by_owner(&self, owner_id: Uuid) -> Result<Vec<Blog>, Box<dyn Error>> {
        let args = (owner_id,);
        let query = r#"
            select Blog {
                id,
                title,
            }
            filter .owner = <User><uuid>$0
        "#;

        let blogs: Vec<Blog> = self.client.query(query, &(args)).await?;
        Ok(blogs)
    }
}
