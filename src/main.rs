mod app;
mod audio_query;
mod communications;
mod error;
mod metas;

use app::app_run;

#[async_std::main]
async fn main() {
    if let Err(err) = app_run().await {
        eprintln!("Error: {}", err);
        ::std::process::exit(1);
    }
}
