#[macro_use]
extern crate diesel_autoincrement_new_struct;


use ntex::web;

mod model;
mod schema;
mod db;
mod actions;
mod crud_controllers;
mod errors;
mod auth;
mod password_json;
mod auth_middleware;
mod client_actions;

#[web::get("/")]
async fn hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hello world!")
}

#[web::post("/echo")]
async fn echo(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl web::Responder {
    web::HttpResponse::Ok().body("Hey there!")
}

#[web::get("/user/{user_id}")]
async fn get_user(
    pool: web::types::State<db::DbPool>,
    user_uid: web::types::Path<i64>,
) -> Result<web::HttpResponse, web::Error> {
    let user_uid = user_uid.into_inner();
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &mut conn)).await?;

    if let Some(user) = user {
        Ok(web::HttpResponse::Ok().json(&user))
    } else {
        let res = web::HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

#[web::get("/user")]
async fn get_users(
    pool: web::types::State<db::DbPool>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user_list = web::block(move || actions::find_users(&mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&user_list))
}

#[web::post("/user")]
async fn add_user(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<model::NewUser>,
) -> Result<web::HttpResponse, web::Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(form.into_inner(), &mut conn)).await?;

    Ok(web::HttpResponse::Ok().json(&user))
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    let db_pool: db::DbPool = db::establish_connection_pool();

    web::HttpServer::new(move|| {
        web::App::new()
            .state(db_pool.clone())
            .wrap(web::middleware::Logger::default())
            .wrap(auth_middleware::AuthStruct)
            .configure(crud_controllers::wire_routes)
            .service(auth::get_auth_token)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(num_cpus::get())
    .run()
    .await
}