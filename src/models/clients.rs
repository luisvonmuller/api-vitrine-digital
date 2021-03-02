
/* To import all  macros from diesel ... */
use super::*;

#[derive(Debug, Queryable, QueryableByName, Serialize)]
#[table_name = "client"]
pub struct Client { 
    pub client_id: i32,
    pub client_name: String, 
    pub client_mail: Option<String>,
    pub client_phone: Option<String>,
    pub client_uf: Option<String>,
    pub client_city: Option<String>,
    pub client_address: Option<String>,
}

#[derive(Insertable, Debug, AsChangeset, Deserialize, Serialize)]
#[table_name = "client"]
pub struct NewClient { 
    pub client_name: String, 
    pub client_mail: Option<String>,
    pub client_phone: Option<String>,
    pub client_uf: Option<String>,
    pub client_city: Option<String>,
    pub client_address: Option<String>,
}   