use actix_web::{get,post,web,App,HttpResponse,HttpServer,Responder};

#[get("/")]
async  fn hello()->impl Responder
{
    HttpResponse::Ok().body("Hello From RUST Server")
}


#[post("/echo")]
async fn echo(request_body:String) -> impl Responder
{
    HttpResponse::Created().body(request_body)
}

async fn mannual_hello() -> impl Responder
{
    HttpResponse::Ok().body("Hey! Diya")
}



#[actix_web::main]
async fn main()->std::io::Result<()>
{
    HttpServer::new( || {
        App::new()
        .service(hello)
        .service(echo)
        .route("/diya",web::get().to(mannual_hello))
    })
    .bind(("127.0.0.1",8080))?.run().await
}

