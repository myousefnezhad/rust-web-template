pub mod base;
pub mod servers;
pub mod users;

use crate::handlers::base::*;
use crate::handlers::servers::*;
use crate::handlers::users::*;
use actix_web::web::{scope, ServiceConfig};
use actix_web_httpauth::middleware::HttpAuthentication;
use lib_middleware::validator;

pub fn handlers(cfg: &mut ServiceConfig) {
    let auth = HttpAuthentication::bearer(validator);
    cfg.service(get_index)
        .service(say_hi)
        .service(
            scope("/auth")
                .wrap(auth)
                .service(post_ping)
                .service(get_users)
                .service(post_user)
                .service(patch_user)
                .service(delete_user),
        );
}
