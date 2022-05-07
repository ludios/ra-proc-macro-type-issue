#![feature(
    generators,
    proc_macro_hygiene,
    stmt_expr_attributes,
)]

use anyhow::{Error, bail};
use futures::stream::BoxStream;
use futures_async_stream::try_stream;

fn minimized_try_stream() -> BoxStream<'static, Result<String, Error>> {
    Box::pin(
        #[try_stream]
        async move {
            if false {
                bail!("bye");
            }
        }
    )
}

fn main() {
    let _ = minimized_try_stream();
}
