use actix_web::{Responder, HttpRequest};
use actix_identity::Identity;

pub async fn execute_login(id: Identity, _req: HttpRequest) -> impl Responder {
    id.remember("213093-21893092183210983".to_owned());
    String::from("Welcome")
}
