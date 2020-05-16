use std::collections::HashMap;

// Global server configurations
#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Config {
	pub captcha: bool,
	pub links: HashMap<String, String>,
}

protocol::gen_global!(pub, pub, Config);
