use actix_web::{get, web, App, HttpServer, Responder};

struct AppState {
    app_name: String,
}

// async fn index()->impl Responder
// {
//     "Hello-World"
// }

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}


#[get("/name")]
async fn get_my_name_handler() -> impl Responder
{
    "Pranay Babu"
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()>
// {
//     HttpServer::new (
//     || {
//         App::new().service(
//             web::scope("/app")
//             .route("index.html", web::get().to(index)),
//         )
//     })
//     .bind(("127.0.0.1",8080))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
            .service(get_my_name_handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
}



