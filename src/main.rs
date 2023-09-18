mod appstate;
mod configuration;
mod webserver;
mod macros;
use appstate::AppState;
use tracing::*;
use webserver::start_web_server;

#[tokio::main]
async fn main() -> tide::Result<()> {
    init_logger();
    let mut state = AppState::new();
    state.load_config("config.json").await;
    let handles = spawn_tasks!(state, start_web_server);
    state.add_handles(handles).await;
    info!("Press T to stop the program gracefully.");
    async_listener!("t").await;
    info!("Termination key pressed, asking to spool down tasks...");
    state.stop_tasks().await;
    Ok(())
}

fn init_logger() {
    tracing_subscriber::FmtSubscriber::builder()
        .pretty()
        .with_line_number(false)
        .with_file(false)
        .without_time()
        .init();
}