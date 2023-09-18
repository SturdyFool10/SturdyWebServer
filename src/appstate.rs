use std::{sync::Arc, fs::{File, self}, io::Write, path::Path};
use tokio::{
    sync::Mutex,
    task::JoinHandle,
    time::{timeout, Duration},
};
use tracing::*;

use crate::configuration::Config;
#[derive(Clone)]
pub struct AppState {
    task_handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
    pub config: Arc<Mutex<Config>>,
}

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

impl AppState {
    pub fn new() -> Self {
        Self {
            task_handles: Arc::new(Mutex::new(vec![])),
            config: Arc::new(Mutex::new(Default::default())),
        }
    }
    pub async fn add_handles(&mut self, mut handles: Vec<JoinHandle<()>>) {
        let mut self_handles = self.task_handles.lock().await;
        for _ in 0..handles.len() {
            if let Some(handle) = handles.pop() {
                self_handles.push(handle);
            }
        }
    }
    pub async fn add_handle(&mut self, handle: JoinHandle<()>) {
        let mut handles = self.task_handles.lock().await;
        handles.push(handle);
    }
    pub async fn stop_tasks(&mut self) {
        let mut handles = self.task_handles.lock().await;
        info!("Spolling down all tasks.");
        for (index, handle) in handles.iter_mut().enumerate() {
            info!("terminating task ID: {}", &index);
            let inf = timeout(Duration::from_secs_f64(5.), handle).await;
            match inf {
                Ok(_) => {}
                Err(what) => error!(
                    "Task ID: {} failed to terminate on time, Error: {:?}",
                    &index, what
                ),
            }
        }
    }
    pub async fn load_config(&mut self, path: &str) {
        info!("Loading config from {}...", &path);
        let data = read_file_to_string_or_empty(path);
        if data == String::new() {
            info!("it would seem that we could not find a config file, we will generate a default for you!");
            let conf: Config = Default::default();
            let str = serde_json::to_string_pretty(&conf).unwrap();
            let mut f = File::create(path).expect(
                &format!("There was an error creating the file specified: {}", &path)[..],
            );
            f.write_all(str.as_bytes()).unwrap();
            let mut confi = self.config.lock().await;
            confi.overwrite(conf);
            info!("loaded the default successfully!")
        } else {
            info!("file found! attempting to load...");
            let conf: Config = serde_json::from_str(data.as_str()).unwrap();
            let mut confi = self.config.lock().await;
            confi.overwrite(conf);
            info!("done! configuration loaded!");
        }
    }
}

