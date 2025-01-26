pub mod base;
pub mod groups;
pub mod index;
pub mod login;
pub mod servers;
pub mod users;
pub mod register;

use crate::handlers::base::*;
use crate::handlers::groups::*;
use crate::handlers::index::*;
use crate::handlers::login::*;
use crate::handlers::servers::*;
use crate::handlers::users::*;
use crate::handlers::register::*;
use actix_web::web::{scope, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use lib_middleware::validator;

pub fn handlers(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    cfg.service(get_index)
        .service(page_index)
        .service(page_login)
        .service(page_register)
        .service(post_login)
        .service(post_register)
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
                .service(patch_group)
                .service(delete_group),
        );
}
