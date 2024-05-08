use std::{path::PathBuf, sync::Arc};

use rspc::{BuiltRouter, ExportConfig, Rspc};

use crate::{core::context::Context, middleware::cookies};

pub const R: Rspc<Context> = Rspc::new();

pub fn get() -> Arc<BuiltRouter<Context>> {
    let router = R
        .router()
        .procedure("version", R.with(cookies()).query(|_, _: ()| Ok("0.0.2")))
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
