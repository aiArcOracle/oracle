use std::env;
use std::collections::HashMap;

pub fn load_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    if let Ok(keys) = env::var("OPENAI_API_KEY") {
        config.insert("openai_api_key".to_string(), keys);
    }
    if let Ok(keys) = env::var("GEMINI_API_KEY") {
        config.insert("gemini_api_key".to_string(), keys);
    }
    config
}
