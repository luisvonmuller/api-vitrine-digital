use super::*;

pub struct ClientDemands { 
    pub client_demand_id: i32,
    pub client_demand_date: NaiveDateTime,
    pub client_client_id: i32,
}

pub struct NewClientDemand {
    pub client_demand_date: NaiveDateTime,
    pub client_client_id: i32,
}
