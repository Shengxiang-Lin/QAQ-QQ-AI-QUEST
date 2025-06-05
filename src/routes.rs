use actix_web::web;

use crate::handlers::{show_info, show_config, update_config, get_config_new_list, show_new_config, update_model};

pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(show_info)
       .service(show_config)
       .service(update_config)
       .service(get_config_new_list)
       .service(show_new_config)
       .service(update_model);
}