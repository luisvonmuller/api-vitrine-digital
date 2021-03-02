use actix_identity::Identity;
use crate::MysqlConnection;
use actix_web::{post, web, Error, HttpResponse};

use chrono::Utc;

use crate::models::demands::NewClientDemand;
use crate::DbPool;

#[post("/demand/{demand_id}")]
pub async fn add_demand(
    id: Identity,
    pool: web::Data<DbPool>,
    demand_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Some(_identity) = id.identity() {
        let conn = pool.get().expect("couldn't get db connection from pool");
        let client = web::block(move || {
            self::execute(
                *demand_id,
                &conn,
            )
        })
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        Ok(HttpResponse::Ok().json(client))
    } else {
        Ok(HttpResponse::Unauthorized().json(r#"{status: unauthorized}"#))
    }
}

pub fn execute<'a>(
    id: i32,
    conn: &MysqlConnection,
) -> Result<NewClientDemand, diesel::result::Error> {
    use crate::diesel::RunQueryDsl;
    use crate::schema::client_demand::dsl::*;
    use crate::models::enums::DemandStatus;

    let new_demand = NewClientDemand {
        client_demand_date: Utc::now().naive_utc(),
        client_demand_status: DemandStatus::Requested as i32,
        client_client_id: id,
    };

    diesel::insert_into(client_demand)
        .values(&new_demand)
        .execute(conn)?;

    Ok(new_demand)
}
