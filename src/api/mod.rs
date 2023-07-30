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

#[macro_export]
macro_rules! create_query {
    {} => { "" };
    ($($KEY:tt : $VALUE:expr),*) => {
        {
            let mut query = String::new();

            $(
                query.push_str(format!("&{}={}", stringify!($KEY), stringify!($VALUE)).as_str());
            )*

            query
        }
    };
}