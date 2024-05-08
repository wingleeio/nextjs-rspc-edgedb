use std::sync::Mutex;

use rspc::integrations::httpz::Request;

use crate::core::context;

pub fn cookies() -> context::middleware!() {
    |mw, mut ctx| async move {
        let request = context::query!(ctx, Mutex<Request>);
        let mut request = request.lock().unwrap();
        let cookies = request.cookies().ok_or_else(|| {
            rspc::Error::new(
                rspc::ErrorCode::InternalServerError,
                "Failed to find cookies in the request.".to_string(),
            )
        })?;

        context::add!(ctx, cookies);

        Ok(mw.next(ctx))
    }
}
