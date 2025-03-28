use actix_web::web;

use crate::handlers::{show_info};

pub fn config(cfg: &mut web::ServiceConfig){
  cfg.service(show_info);
  
}