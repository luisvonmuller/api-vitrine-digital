use actix_identity::Identity;
use actix_web::{get, web, Error, HttpResponse};

use crate::DbPool;
use crate::MysqlConnection;

#[get("/client/{client_id}")]
pub async fn get_client(
    id: Identity,
    pool: web::Data<DbPool>,
    client_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Some(_identity) = id.identity() {
        let conn = pool.get().expect("couldn't get db connection from pool");

        let tmp_id = client_id.clone();
        // use web::block to offload blocking Diesel code without blocking server thread
        let client = web::block(move || execute_single(*client_id, &conn))
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

#[get("/clients")]
pub async fn get_all(id: Identity, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    if let Some(_identity) = id.identity() {
        let conn = pool.get().expect("couldn't get db connection from pool");

        // use web::block to offload blocking Diesel code without blocking server thread
        let clients = web::block(move || execute_all(&conn)).await.map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

        Ok(HttpResponse::Ok().json(clients))
    } else {
        Ok(HttpResponse::Unauthorized().json(r#"{status: unauthorized}"#))
    }
}

use crate::models::clients::Client;

pub fn execute_all(conn: &MysqlConnection) -> Result<Vec<Client>, diesel::result::Error> {
    use crate::schema::client;
    use diesel::prelude::*;

    client::table
        .select(client::all_columns)
        .load::<Client>(conn)
}

pub fn execute_single(
    client_id: i32,
    conn: &MysqlConnection,
) -> Result<Option<Client>, diesel::result::Error> {
    use crate::schema::client;
    use diesel::prelude::*;

    let client = client::table
        .filter(client::client_id.eq(client_id))
        .first::<Client>(conn)
        .optional()?;

    Ok(client)
}
