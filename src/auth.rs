use ntex::web;
use crate::model;
use crate::db;
use crate::actions;
use crate::errors::ServiceError;
use jwt::{AlgorithmType, Header, SignWithKey, Token, VerifyWithKey};
use std::collections::BTreeMap;
use hmac::{Hmac, Mac};
use sha2::Sha384;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use pwhash::bcrypt;
use chrono::prelude::*;
use lazy_static::lazy_static; 

const PASWORD_SECRET_SALT: &str = "secret_phrase_121231321237548921";
const JWT_EXPIRATION_IN_SECONDS: i64 = 14*86400;
const JWT_ISSUER_ADMIN: &str = "admin api";
lazy_static! {
    static ref JWT_SALT_KEY: Hmac<Sha384> = Hmac::new_from_slice(b"some-secret-123123123").expect("error creating key");
}

#[derive(Serialize, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String
}

#[derive(Serialize)]
pub struct JwtResponse {
    id: String,
    token: String
}
pub fn get_system_time() -> i64 {
    let now = Utc::now();
    now.timestamp()
}

#[derive(Serialize, Deserialize)]
pub struct JwtBody {
    sub: String,
    iss: String,
    iat: i64,
    exp: i64,
    roles: Vec<String>
}

fn create_jwt_for_user(
    username: String,
    roles: Vec<String>
) -> String 
{
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };
    let item = JwtBody {
        sub: username,
        iss: JWT_ISSUER_ADMIN.to_string(),
        iat: get_system_time(),
        exp: get_system_time() + JWT_EXPIRATION_IN_SECONDS,
        roles
    };
    return Token::new(header, item).sign_with_key(&*JWT_SALT_KEY).expect("error creating key").as_str().to_string();
}

fn verify_jwt_for_user(
    token_str: String,
    conn: &mut MysqlConnection,
) -> bool 
{
    let token: Token<Header,JwtBody, _> = token_str.verify_with_key(&*JWT_SALT_KEY).expect("error veryfing key");
    let header = token.header();
    let claims = token.claims();
    let exp_date: i64 = claims.exp;
    let result = find_user_by_username(claims.sub.clone(), conn);
    if  exp_date>get_system_time() {
        if let Ok(Some((user, roles))) = result{
            return true;
        }
    }

    return false;
}

// pub fn guard_controller(username: String, role:String) -> bool {

// }

pub fn verify_jwt(
    token_str: String,
) -> bool 
{
    let token: Token<Header, JwtBody, _> = token_str.verify_with_key(&*JWT_SALT_KEY).expect("error veryfing key");
    let header = token.header();
    let claims = token.claims();
    let exp_date: i64 = claims.exp;
    if  exp_date>get_system_time() && claims.iss.eq(JWT_ISSUER_ADMIN) {
        return true;
    }
    return false;
}

pub fn get_jwt_data(
    token_str: String,
) -> (String, Vec<String>)
{
    let token: Token<Header, JwtBody, _> = token_str.verify_with_key(&*JWT_SALT_KEY).expect("error veryfing key");
    let claims = token.claims();
    return (claims.sub.clone(), claims.roles.clone())
}

fn find_user_by_username(
    username: String,
    conn: &mut MysqlConnection,
)->Result<Option<(model::User, Vec<String>)>, diesel::result::Error>{
    use crate::schema::*;
    let user = users::table
        .filter(users::name.eq(username))
        .first::<model::User>(conn)
        .optional()?;
    if let Some(u) = user {
        let roles:Vec<String> = model::UserToRole::belonging_to(&u)
        .inner_join(role::table)
        .select(model::Role::as_select())
        .load(conn)?
        .into_iter()
        .map(|r| r.authority)
        .collect();

        Ok(Some((u, roles)))
    }else {
        Ok(None)
    }
}

pub fn encrypt_password(string:String)->Result<String, ServiceError>{
    println!("trying to salt {}",string.clone());
    let mut salted =  string.clone();
    salted.push_str(PASWORD_SECRET_SALT);
    println!("encrypting salted password {}",string.clone());
    let encrypted = bcrypt::hash(salted).unwrap();
    Ok(encrypted)
}

pub fn verify_password(input:String, source:String)-> bool {
    println!("trying to salt {}", input.clone());
    let mut salted = input.clone();
    salted.push_str(PASWORD_SECRET_SALT);
    println!("veryfing salted password {} against {}",input.clone(), source.clone());
    return bcrypt::verify(salted, &source);
}


pub fn find_user_by_auth_data(
    auth_data: AuthData,
    conn: &mut MysqlConnection,
) -> Result<JwtResponse, ServiceError> {
    
    let result = find_user_by_username(auth_data.username, conn);

    if let Ok(Some((user, roles))) = result{
        if verify_password(auth_data.password, user.password){
            return Ok(JwtResponse{id: user.name.clone(), token:create_jwt_for_user(user.name.clone(), roles)});
        }
    }
        
    Err(ServiceError::Unauthorized("User not found".to_string()))
}

#[web::post("/auth")]
async fn get_auth_token(
    pool: web::types::State<db::DbPool>,
    form: web::types::Json<AuthData>,
) -> Result<web::HttpResponse, ServiceError> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let res = web::block(move || find_user_by_auth_data(form.into_inner(), &mut conn)).await;
    match res {
        Ok(user) => {

            Ok(web::HttpResponse::Ok().json(&user))
        }
        Err(err) => match err {
            web::error::BlockingError::Error(service_error) => Err(service_error),
            web::error::BlockingError::Canceled => Err(ServiceError::InternalServerError),
        },
    }
}