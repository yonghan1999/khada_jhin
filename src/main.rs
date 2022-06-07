use std::{collections::HashMap, hash::Hash};

use crate::{
    config::GLOBAL_DATA,
    core::util::{http_util::HTTP_CONFIG, lock_file::get_lockfile},
};

mod config;
mod core;
fn main() {
    let lockfile_content = get_lockfile().expect("can't load lock file");
    let base_url = format!(
        "{}://127.0.0.1:{}",
        lockfile_content.protocol, lockfile_content.port
    );
    HTTP_CONFIG.lock().as_mut().unwrap().set_base_url(&base_url);
    let mut header: HashMap<String, String> = HashMap::new();
    header.insert("Content-Type".to_string(), "application/json".to_string());
    header.insert("Accept".to_string(), "application/json".to_string());
    header.insert("Authorization".to_string(), lockfile_content.password);
    HTTP_CONFIG.lock().as_mut().unwrap().set_base_url(&base_url);
}
