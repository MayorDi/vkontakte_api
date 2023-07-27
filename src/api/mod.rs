pub mod account;
pub mod groups;

pub struct Api {
    access_token: String,
    version: String,
}

impl Api {
    pub fn new(access_token: String, version: String) -> Self {
        Self { access_token, version }
    }

    pub fn set_access_token(&mut self, new_access_token: String) {
        self.access_token = new_access_token;
    }

    pub fn set_version(&mut self, new_version: String) {
        self.version = new_version;
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }
}