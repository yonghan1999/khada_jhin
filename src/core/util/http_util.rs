use lazy_static::lazy_static;
use reqwest::{self, header};
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    pub static ref HTTP_CONFIG: Mutex<HttpConfig> = Mutex::new(HttpConfig::default());
}

pub struct HttpConfig {
    base: String,
    header: HashMap<String, String>,
}

impl HttpConfig {
    pub fn default() -> Self {
        Self {
            base: "https://www.baidu.com".to_string(),
            header: HashMap::new(),
        }
    }

    pub fn do_get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(reqwest::blocking::get(format!("{}{}", self.base, url))?.text()?)
    }

    pub fn set_base_url(&mut self, url: &str) {
        self.base = String::from(url);
    }

    pub fn set_header(&mut self, header: HashMap<String,String>) {
        self.header = header;
    }
}
