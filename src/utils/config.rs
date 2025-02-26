use std::env;
use std::collections::HashMap;

pub fn load_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    if let Ok(key) = env::var("TOGETHER_API_KEY") {
        config.insert("together_api_key".to_string(), key);
    }
    config
}

pub fn get_together_api_key() -> Result<String, std::env::VarError> {
    env::var("TOGETHER_API_KEY")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        // Set environment variable for testing
        std::env::set_var("TOGETHER_API_KEY", "test_key");
        let config = load_config();
        assert_eq!(config.get("together_api_key"), Some(&"test_key".to_string()));
    }

    #[test]
    fn test_get_together_api_key() {
        std::env::set_var("TOGETHER_API_KEY", "test_key");
        let key = get_together_api_key().unwrap();
        assert_eq!(key, "test_key");
    }
}