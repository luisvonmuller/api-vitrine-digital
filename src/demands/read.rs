/* use actix_identity::Identity;
use actix_web::{get, web, Error, HttpResponse};

use crate::DbPool;
use crate::MysqlConnection;

#[get("/client/{demand_id}")]
pub async fn get_client(
    id: Identity,
    pool: web::Data<DbPool>,
    demand_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Some(_identity) = id.identity() {
        let conn = pool.get().expect("couldn't get db connection from pool");

        let tmp_id = demand_id.clone();
        // use web::block to offload blocking Diesel code without blocking server thread
        let client = web::block(move || execute_single(*demand_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        if let Some(client) = client {
            Ok(HttpResponse::Ok().json(client))
        } else {
            let res = HttpResponse::NotFound().body(format!("No client found with id: {}", tmp_id));
            Ok(res)
        }
    } else {
        Ok(HttpResponse::Unauthorized().json(r#"{status: unauthorized}"#))
    }
}
*/