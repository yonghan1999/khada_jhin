use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static!{
    pub static ref GLOBAL_DATA:Mutex<GlobalConfig> = Mutex::new(GlobalConfig::default());
}
/// 全局配置
/// auto_accept 自动接受对局
pub struct GlobalConfig {
    pub auto_accept: bool,
}

impl GlobalConfig {
    pub fn clear_auto_accept(&mut self) {
        self.auto_accept = false;
    }
    
    pub fn auto_accept_game(&mut self) {
        self.auto_accept = true;
    }

    pub fn default() -> Self {
        Self{ auto_accept: false}
    }
}
