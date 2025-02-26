use std::env;
use std::collections::HashMap;

pub fn load_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    if let Ok(keys) = env::var("TOGETHER_API_KEY") {
        config.insert("together_api_key".to_string(), keys);
    }
    config
}
