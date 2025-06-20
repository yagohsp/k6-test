use actix_web::{delete, get, http::header, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{
    bson::doc,
    options::{ClientOptions, IndexOptions},
    Client, Collection, IndexModel,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Programador {
    pub apelido: String,
    pub nome: String,
    pub stack: Option<Vec<String>>,
    pub nascimento: String,
}

struct AppState {
    user_collection: Arc<Collection<Programador>>,
}

#[post("/programadores")]
async fn create_user(data: web::Data<AppState>, user: web::Json<Programador>) -> impl Responder {
    let collection = &data.user_collection;
    let validation = Regex::new(r"^[a-zA-Z\s]+$").unwrap();

    let nome = user.nome.trim();
    if nome.is_empty() || !validation.is_match(nome) {
        return HttpResponse::UnprocessableEntity()
            .body("Nome não pode ter valores vazios ou caracteres especiais ou numeros");
    }

    let apelido = user.apelido.trim();
    if apelido.is_empty() || !validation.is_match(apelido) {
        return HttpResponse::UnprocessableEntity()
            .body("Apelido não pode ter valores vazios ou caracteres especiais ou numeros");
    }

    let date_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if user.nascimento.is_empty() || !date_regex.is_match(&user.nascimento) {
        return HttpResponse::BadRequest().body("Nascimento deve estar no formato AAAA-MM-DD.");
    }

    if let Some(stack) = &user.stack {
        if stack.iter().any(|s| s.trim().is_empty()) {
            return HttpResponse::BadRequest().body("Stack não pode ter valores vazios");
        }
    }

    match collection.insert_one(user.into_inner()).await {
        Ok(res) => HttpResponse::Created()
            .insert_header((
                header::LOCATION,
                format!("/programadores/:{}", res.inserted_id.to_string()),
            ))
            .body(format!("New document ID: {}", res.inserted_id)),
        Err(err) => {
            if err.to_string().contains("E11000") {
                return HttpResponse::UnprocessableEntity().body("Apelido já existente.");
            }
            HttpResponse::InternalServerError()
                .body(format!("Erro ao inserir no banco de dados: {}", err))
        }
    }
}

#[get("/contagem-programadores")]
async fn count_all(data: web::Data<AppState>) -> impl Responder {
    let collection = &data.user_collection;
    let count = collection.count_documents(doc! {}).await.unwrap();
    let string = format!("Programadores: {}", count);
    println!("{}", string);
    HttpResponse::Ok().body(string)
}

#[delete("/")]
async fn delete_all(data: web::Data<AppState>) -> impl Responder {
    let collection = &data.user_collection;
    let drop = Collection::drop(collection);
    drop.await.unwrap();
    HttpResponse::Ok().body("Database deleted")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://mongodb:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("yshen");

    let index_options = IndexOptions::builder().unique(true).build();
    let apelido_index = IndexModel::builder()
        .keys(doc! { "apelido": 1 })
        .options(index_options)
        .build();
    let user_collection = Arc::new(db.collection::<Programador>("programador"));

    let collection = db.collection::<mongodb::bson::Document>("programador");
    collection.create_index(apelido_index).await.unwrap();

    let state = web::Data::new(AppState { user_collection });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(create_user)
            .service(count_all)
            .service(delete_all)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
