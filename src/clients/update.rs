use crate::MysqlConnection;
use actix_web::{put, web, Error, HttpResponse};
use actix_identity::Identity;

use crate::models::clients::NewClient;
use crate::DbPool;

#[put("/client/{client_id}")]
pub async fn update_client(
    id: Identity,
    pool: web::Data<DbPool>,
    form: web::Json<NewClient>,
    client_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    if let Some(_identity) = id.identity() {
        let conn = pool.get().expect("couldn't get db connection from pool");

        let client = web::block(move || {
            self::execute_update(
                *client_id,
                form.client_name.to_owned(),
                form.client_mail.to_owned(),
                form.client_phone.to_owned(),
                form.client_uf.to_owned(),
                form.client_city.to_owned(),
                form.client_address.to_owned(),
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

pub fn execute_update(
    id: i32,
    json_client_name: String,
    json_client_mail: Option<String>,
    json_client_phone: Option<String>,
    json_client_uf: Option<String>,
    json_client_city: Option<String>,
    json_client_address: Option<String>,
    conn: &MysqlConnection,
) -> Result<NewClient, diesel::result::Error> {
    use crate::schema::client;
    use diesel::prelude::*;

    let updated_client = NewClient {
        client_name: json_client_name.to_owned(),
        client_mail: json_client_mail.to_owned(),
        client_phone: json_client_phone.to_owned(),
        client_uf: json_client_uf.to_owned(),
        client_city: json_client_city.to_owned(),
        client_address: json_client_address.to_owned(),
    };

    diesel::update(client::table.filter(client::client_id.eq(id)))
        .set(&updated_client)
        .execute(conn)?;

    Ok(updated_client)
}
