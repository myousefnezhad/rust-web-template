pub mod base;
pub mod google;
pub mod groups;
pub mod index;
pub mod login;
pub mod register;
pub mod servers;
pub mod session;
pub mod users;

use crate::handlers::base::*;
use crate::handlers::google::*;
use crate::handlers::groups::*;
use crate::handlers::index::*;
use crate::handlers::login::*;
use crate::handlers::register::*;
use crate::handlers::servers::*;
use crate::handlers::session::*;
use crate::handlers::users::*;
use actix_web::web::{scope, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use lib_middleware::validator;

pub fn handlers(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    cfg.service(get_index)
        .service(page_index)
        .service(page_login)
        .service(page_register)
        .service(page_session)
        .service(post_login)
        .service(page_google_token)
        .service(post_register)
        .service(get_google_auth)
        .service(get_google_callback)
        .service(say_hi)
        .service(
            scope("/auth")
                .wrap(auth)
                .service(post_ping)
                .service(get_users)
                .service(post_user)
                .service(patch_user)
                .service(delete_user)
                .service(get_groups)
                .service(post_group)
                .service(post_session)
                .service(delete_session)
                .service(patch_group)
                .service(delete_group),
        );
}
