use actix_web::{get, post, web, HttpResponse, Responder};



{'self_id': 2603134759, 
'user_id': 2421468125, 
'time': 1743061547, 
'message_id': 1922123618, 
'message_seq': 11, 
'message_type': 'private', 
'sender': 
{'user_id': 2421468125, 
'nickname': 'Rhapsody', 
'card': ''}, 
'raw_message': '111', 
'font': 14, 
'sub_type': 'friend', 
'message': [{'type': 'text', 'data': {'text': '111'}}], 
'message_format': 'array', 
'post_type': 'message'}
127.0.0.1 - - [27/Mar/2025 15:45:46] "POST / HTTP/1.1" 200 -


{'self_id': 2603134759, 
'user_id': 2421468125, 
'time': 1743061807, 
'message_id': 1463962522, 'message_seq': 12, 'message_type': 'private',
'sender': {'user_id': 2421468125, 'nickname': 'Rhapsody', 'card': ''}, 
'raw_message': '[CQ:face,id=277]', 'font': 14, 'sub_type': 'friend', 
'message': [{'type': 'face', 'data': {'id': '277'}}], 'message_format': 'array', 'post_type': 'message'}
127.0.0.1 - - [27/Mar/2025 15:50:06] "POST / HTTP/1.1" 200 -

#[post("/")]
pub async fn show_info(
  info
) -> impl Responder {
  println!("Received info: {:?}", info);
  //HttpResponse::Ok().json(info);
}