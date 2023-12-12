mod app;
mod audio_query;
mod communications;
mod error;
mod metas;

use app::app_run;

fn main() {
    if let Err(err) = app_run() {
        eprintln!("Error: {}", err);
        ::std::process::exit(1);
    }
}
