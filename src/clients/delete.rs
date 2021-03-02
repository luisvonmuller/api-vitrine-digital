use actix_identity::Identity;
use actix_web::{delete, web, Error, HttpResponse};

use crate::DbPool;
use crate::MysqlConnection;

#[delete("/client/{client_id}")]
pub async fn delete_client(
    id: Identity,
    pool: web::Data<DbPool>,
    client_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Some(_identity) = id.identity() {
        let conn = pool.get().expect("couldn't get db connection from pool");

        let tmp_id = client_id.clone();
        // use web::block to offload blocking Diesel code without blocking server thread
        let client = web::block(move || execute_delete(*client_id, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        if let Some(_client) = client {
            Ok(HttpResponse::Ok().json(r#"{status: "client deleted"}"#))
        } else {
            let res = HttpResponse::NotFound().body(format!("No client found with id: {}", tmp_id));
            Ok(res)
        }
    } else {
        Ok(HttpResponse::Unauthorized().json(r#"{status: unauthorized}"#))
    }
}

use crate::models::clients::Client;

pub fn execute_delete(
    id: i32,
    conn: &MysqlConnection,
) -> Result<Option<Client>, diesel::result::Error> {
    use crate::schema::client;
    use diesel::prelude::*;

    let client = client::table
        .filter(client::client_id.eq(id))
        .first::<Client>(conn)
        .optional()?;

    diesel::delete(client::table.filter(client::client_id.eq(id))).execute(conn)?;

    Ok(client)
}
