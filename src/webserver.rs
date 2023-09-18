use tide::{
    StatusCode,
    Body, http::mime
};
use std::{fs, path::Path};
use lazy_regex::*;
use crate::appstate::AppState;

fn read_file_to_string_or_empty(path: &str) -> String {
    let path = Path::new(path);

    match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Failed to read the file at {:?}", path);
            String::new() // Return an empty string as a fallback
        }
    }
}

pub async fn start_web_server(state: AppState) {
    let mut app = tide::with_state(state.clone());
    // Serve /webcontent/index.html for the root /
    // Serve /webcontent/index.html for the root /
    app.at("/").get(serve_index_html);

    // Serve the rest of /webcontent/ as a directory
    let _ = app.at("/").serve_dir("webcontent/");
    let mut address = String::new();
    let config = state.config.lock().await;
    address += config.interface.clone().as_str();
    address += ":";
    address += config.port.as_str();
    drop(config);
    let _ = app.listen(address).await;
}

async fn serve_index_html(req: tide::Request<AppState>) -> tide::Result {
    let _state = req.state();
    let index_path = "webcontent/index.html";
    let index_content: String = read_file_to_string_or_empty(index_path);
    let uncommented_index: String = regex_replace_all!(
        r"(?s)<!--.*?-->|<!--[\s\S]*?-->",
        index_content.as_str(),
        |_| String::new()
    ).to_string();
    let mut body = Body::from_string(uncommented_index);
    body.set_mime(mime::HTML);
    let mut response = tide::Response::new(StatusCode::Ok);
    response.set_body(body);
    Ok(response)
}