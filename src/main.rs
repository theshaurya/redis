use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use redis::AsyncCommands;
use redis::Value;
use redis::{FromRedisValue};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let redis_client = redis::Client::open("redis://default:redispw@localhost:49153").unwrap();
    let redis_conn = Arc::new(Mutex::new(
        redis_client.get_async_connection().await.unwrap(),
    ));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(redis_conn.clone()))
            .route("/get", web::get().to(get))
            .route("/set", web::get().to(set))
            .route("/expire", web::get().to(expire))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn set(redis_conn: web::Data<Arc<Mutex<redis::aio::Connection>>>) -> impl Responder {
    let a:redis::RedisResult<()> = redis_conn
        .lock()
        .unwrap()
        .set("my_key", 1)
        .await;
    match a {
        Ok(res) => {
            dbg!(res);
            HttpResponse::Ok().finish();
        }
        Err(res) => {
            dbg!(res);
            HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

async fn expire(redis_conn: web::Data<Arc<Mutex<redis::aio::Connection>>>) -> impl Responder {
    let res:redis::RedisResult<i32> = redis_conn
        .lock()
        .unwrap()
        .expire("my_key", 2)
        .await;
    match res {
        Ok(res) => {
            dbg!(res);
            return HttpResponse::Ok().finish();
        }
        Err(res) => {
            dbg!(res);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

async fn get(redis_conn: web::Data<Arc<Mutex<redis::aio::Connection>>>) -> HttpResponse {
    let result: redis::RedisResult<Value> = redis_conn.lock().unwrap().get("my_key").await;

     match result {
        Ok(res) => if res!= Value::Nil{
           let result=i32::from_redis_value(&res).unwrap();
            return HttpResponse::Ok().body(format!("{:?}", result)); 
        }else{
            return HttpResponse::Ok().body("nil");  
        },
        Err(e) => {
            dbg!(e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    
}
