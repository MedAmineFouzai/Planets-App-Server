mod controller;
mod model;
use actix_web::{App,HttpServer};
use mongodb::{options::ClientOptions, Client};

pub struct ModelsContainer {
   user:model::User,
 
}
impl ModelsContainer {
    pub fn new(user:model::User) -> Self {
        ModelsContainer {user}
    }
  }

pub struct AppState{
    models_container:ModelsContainer,

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const DB:&str="Tournaments";
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(DB);
    let user_collection = db.collection("User");

    HttpServer::new(move || {
        let models_container = ModelsContainer::new(
            model::User::new(user_collection.clone()),
           
        );

        App::new()
        .data(AppState{models_container})
        .service(controller::plants)
        .service(controller::search)
        .service(controller::login)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
