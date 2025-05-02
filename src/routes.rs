use actix_web::web;

use crate::handlers::{show_info, show_config,update_config};

pub fn config(cfg: &mut web::ServiceConfig){
  cfg.service(show_info)
  .service(show_config)
  .service(update_config);
  
}