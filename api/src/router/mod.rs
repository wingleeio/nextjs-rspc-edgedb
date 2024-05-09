use crate::{core::context::Context, middleware::cookies};
use rspc::{BuiltRouter, ExportConfig, Rspc};
use std::{path::PathBuf, sync::Arc};

mod auth;

pub const R: Rspc<Context> = Rspc::new();

pub fn get() -> Arc<BuiltRouter<Context>> {
    let router = R
        .router()
        .procedure("version", R.with(cookies()).query(|_, _: ()| Ok("0.0.2")))
        .merge("auth", auth::mount())
        .build()
        .unwrap()
        .arced();

    #[cfg(debug_assertions)]
    router
        .export_ts(ExportConfig::new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../web/src/generated/bindings.ts"),
        ))
        .unwrap();

    router
}
