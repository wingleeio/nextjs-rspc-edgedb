use crate::{
    core::context::{query, Context},
    middleware::{auth, cookies},
    service::{
        auth::Session,
        blogs::{Blog, Blogs},
    },
};
use rspc::{Error, ErrorCode, Router};

use super::R;

async fn get_my_blogs(ctx: Context, _: ()) -> Result<Vec<Blog>, Error> {
    let (session, blogs_service) = query!(ctx, Session, Blogs);

    let blogs = blogs_service
        .get_blogs_by_owner(session.user.id)
        .await
        .map_err(|e| Error::new(ErrorCode::BadRequest, e.to_string()))?;

    Ok(blogs)
}

pub fn mount() -> Router<Context> {
    R.router().procedure(
        "getMyBlogs",
        R.with(cookies()).with(auth()).query(get_my_blogs),
    )
}
