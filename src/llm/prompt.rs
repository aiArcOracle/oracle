use rig::completion::Prompt;

pub fn create_analysis_prompt(data: &str) -> Prompt {
    Prompt::new(&format!("Analyze this Solana data: {}. Provide a concise insight for DeFi users.", data))
}
