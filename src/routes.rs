use actix_web::{post, get, Responder, HttpResponse};

//use db::mongo::{NewUser, insert_user};

use crate::db::mongo::{NewUser, insert_user};


#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok()
    .body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    let msg:String = format!("Retorno: {}", req_body);
    HttpResponse::Ok().body(msg)
}

pub async fn add_user() -> impl Responder {
    let user: NewUser = NewUser {
        name: "Marcos Trajano".to_owned(),
        user_name: "Trajano".to_owned(),
        email: "mtraja@gmail.com".to_owned()
    };

    let res = insert_user(&user).await;

    HttpResponse::Ok().body(res.mensagem)

}