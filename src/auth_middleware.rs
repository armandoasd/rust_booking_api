use crate::auth;
use crate::errors::ServiceError;
use lazy_static::lazy_static;
use ntex::http;
use ntex::service::{Middleware, Service, ServiceCtx};
use ntex::web;
use regex::Regex;
use ntex::util::Extensions;
use std::convert::Infallible;
use web::FromRequest;

lazy_static! {
    static ref WHITELISTED_URLS: Vec<Regex> = [r"^/auth", r"^/client"]
        .iter()
        .map(|x| Regex::new(x).unwrap())
        .collect();
}

pub fn is_url_matched(path: &str, source: Vec<Regex>) -> bool {
    let found = source.iter().find(|&source_path| {
        if let Some(_) = source_path.captures(&path) {
            return true;
        } else {
            return false;
        }
    });

    if let Some(_) = found {
        return true;
    } else {
        return false;
    }
}

pub struct AuthStruct;

impl<S> Middleware<S> for AuthStruct {
    type Service = AuthMiddleware<S>;

    fn create(&self, service: S) -> Self::Service {
        AuthMiddleware { service }
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

#[derive(Clone)]
pub struct JwtAuthData {
    pub username: String,
    pub roles: Vec<String>,
}

impl JwtAuthData {
    fn new(username: String, roles: Vec<String>) -> Self {
        Self {
            username,
            roles
        }
    }
    fn new_guest() -> Self {
        Self {
            username: "guest".to_string(),
            roles: vec!["guest".to_string()]
        }
    }
    fn get_from_request(extensions: &mut Extensions)-> Self {
        if let Some(auth_data) = extensions.get::<Self>(){
            return auth_data.clone();
        }else {
            return Self::new_guest();
        }
    }
}

impl<S, Err> Service<web::WebRequest<Err>> for AuthMiddleware<S>
where
    S: Service<web::WebRequest<Err>, Response = web::WebResponse, Error = web::Error>,
    Err: web::ErrorRenderer,
{
    type Response = web::WebResponse;
    type Error = web::Error;

    ntex::forward_poll_ready!(service);

    async fn call(
        &self,
        req: web::WebRequest<Err>,
        ctx: ServiceCtx<'_, Self>,
    ) -> Result<Self::Response, Self::Error> {
        println!("url requested: {}", req.path());
        let headers = req.headers();
        let auth_header = headers.get("Authorization");
        if !is_url_matched(req.path(), WHITELISTED_URLS.to_vec()) {
            if let Some(header) = auth_header {
                let auth_key_h = header.to_str().unwrap().to_string();
                let auth_key = auth_key_h.replace("Bearer ", "");
                println!("auth key {}", auth_key);
                if auth::verify_jwt(auth_key.clone()) {
                    let (username, roles) = auth::get_jwt_data(auth_key.clone());
                    req.extensions_mut().insert(JwtAuthData::new( username, roles));
                } else {
                    return Err(Self::Error::new(ServiceError::Unauthorized("could not verify jwt".to_string())));
                }
            } else {
                return Err(Self::Error::new(ServiceError::Unauthorized("no auth token provided".to_string())));
            }
        }

        let res = ctx.call(&self.service, req).await?;
        println!("Hi from response");
        Ok(res)
    }
}

impl<Err> FromRequest<Err> for JwtAuthData {
    type Error = Infallible;

    #[inline]
    async fn from_request(req: &web::HttpRequest, _: &mut http::Payload) -> Result<JwtAuthData, Infallible> {
        Ok(JwtAuthData::get_from_request(&mut req.extensions_mut()))
    }
}

pub struct AuthGuard<'a> {
    roles: [&'a str],
}

impl<'a> web::guard::Guard for AuthGuard<'a> {
    fn check(&self, req: &http::RequestHead) -> bool {
        let extensions = req.extensions();
        let auth_data = extensions.get::<JwtAuthData>();
        if let Some(jwt_auth_data) = auth_data {
            let matches = self.roles.iter().fold(0, |acc, requested_role| {
                if jwt_auth_data.roles.iter().any(|i| i.eq(requested_role)) {
                    return acc + 1;
                }
                return acc;
            });
            return matches == self.roles.len();
        }
        return false;
    }
}
