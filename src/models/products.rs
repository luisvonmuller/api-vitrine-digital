use super::*;

pub struct Product { 
    pub product_id: i32, 
    pub product_description: String, 
    pub product_value: f64,
    pub product_second_value: f64,
    pub product_image: Option<String>,
    pub client_demand_client_demand_id: i32,   
}

pub struct NewProduct<'a> { 
    pub product_description: &'a str, 
    pub product_value: f64,
    pub product_second_value: f64,
    pub product_image: Option<&'a str>,
    pub client_demand_client_demand_id: i32,   
}