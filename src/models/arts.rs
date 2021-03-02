use super::*;

pub struct Art { 
    pub ard_id: i32,
    pub art_delivery_time: NaiveDateTime,
    pub art_image: String,
    pub client_demand_client_demand_id: i32,
}


pub struct NewArt<'a> { 
    pub art_delivery_time: NaiveDateTime,
    pub art_image: &'a str,
    pub client_demand_client_demand_id: i32,
}