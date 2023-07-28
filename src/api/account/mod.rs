mod methods;
pub use methods::*;

#[derive(Debug, Clone)]
pub struct User {
    pub id: usize,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub maiden_name: Option<String>,
    pub screen_name: Option<String>,
    pub cancel_request_id: Option<usize>,
    pub sex: Option<usize>,
    pub relation: Option<usize>,
    pub relation_partner_id: Option<i64>,
    pub bdate: Option<String>,
    pub bdate_visibility: Option<usize>,
    pub home_town: Option<String>,
    pub country_id: Option<usize>,
    pub city_id: Option<usize>,
    pub status: Option<String>,
}