use super::*;

#[derive(Debug, Queryable, QueryableByName, Serialize)]
#[table_name = "client_demand"]
pub struct ClientDemand { 
    pub client_demand_id: i32,
    pub client_demand_date: NaiveDateTime,
    pub client_demand_status: i32,
    pub client_client_id: i32,
}

#[derive(Insertable, Debug, AsChangeset, Deserialize, Serialize)]
#[table_name = "client_demand"]
pub struct NewClientDemand {
    pub client_demand_date: NaiveDateTime,
    pub client_demand_status: i32,
    pub client_client_id: i32,
}
