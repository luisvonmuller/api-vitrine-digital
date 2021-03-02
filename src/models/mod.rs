use crate::schema::*;

/* For JSON */
use serde::{Deserialize, Serialize};

/* Date handling... */
use chrono::NaiveDateTime;

pub mod arts;
pub mod clients; // Who holds the clients table
pub mod demands; // Demands 1:n table - 1 stands for demands
pub mod products; // Product n: table //Arts 1:n table - 1 stands for demands

pub struct NewLogin<'a> {
    pub login_id: i32,
    pub login_mail: &'a str,
}
