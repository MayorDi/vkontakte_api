use json::JsonValue;

use self::account::get_result_responce;

pub mod account;
pub mod auth;
pub mod groups;
pub mod messages;

#[derive(Debug, Clone, Copy)]
pub struct Api {
    access_token: &'static str,
    version: &'static str,
}

impl Api {
    pub fn new(access_token: &'static str, version: &'static str) -> Self {
        Self { access_token, version }
    }

    pub fn set_access_token(&mut self, new_access_token: &'static str) {
        self.access_token = new_access_token;
    }

    pub fn set_version(&mut self, new_version: &'static str) {
        self.version = new_version;
    }

    pub fn get_access_token(&self) -> &str {
        self.access_token
    }

    pub fn get_version(&self) -> &str {
        self.version
    }
}

pub async fn create_request(method: &str, api: &Api, query: String) -> Result<JsonValue, JsonValue> {
    let req = format!(
        "https://api.vk.com/method/{method}?access_token={}&v={}{query}",
        api.get_access_token(),
        api.get_version()
    );

    get_result_responce(req).await
}

#[macro_export]
macro_rules! create_query {
    {} => { "" };
    ($($KEY:tt : $VALUE:expr),*) => {
        {
            let mut query = String::new();

            $(
                query.push_str(format!("&{}={}", stringify!($KEY), $VALUE).as_str());
            )*

            query
        }
    };
}