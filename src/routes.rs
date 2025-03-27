use actix_web::web;

use crate::handlers::{send_info, show_info};

pub fn config(cfg: &mut web::ServiceConfig){
  cfg.service(show_info)
      .service(send_info);
}