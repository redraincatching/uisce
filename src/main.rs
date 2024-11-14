use uisce::*;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

/// this struct holds the app state
struct AppState {
    team: Vec<Swimmer>
}

fn initialise_team() -> Vec<Swimmer> {
    let mut team: Vec<Swimmer> = Vec::new();
    let _ = read_csv(&mut team);

    team
}

#[get("/")]
async fn show_team(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&data.team)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

/// endpoint for mixed medley relay
/// TODO: generalise relays so that we can use the same function and signature for them all
#[get("/mixed_medley")]
async fn return_mixed_medley(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(mixed_medley(&data.team))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                team: initialise_team()
            }))
            .service(show_team)
            .service(echo)
            .service(return_mixed_medley)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
