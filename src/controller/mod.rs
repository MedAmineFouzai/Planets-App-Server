use actix_web::{HttpRequest, HttpResponse, Responder, get,web};
use isahc::prelude::*;
const TOKEN:&str="1uLQHB7NKvy508EBLSfDXs-f54JX8FR-JcxE904OE54";

#[get("/plants")]
pub async fn plants(req:HttpRequest)-> impl Responder{
    
    match isahc::get(format!("https://trefle.io/api/v1/plants?token={}",TOKEN)){
        Ok(mut response)=>{
            response.text().ok().unwrap()
        }
        Err(e)=>{
            e.to_string()
        }

    }
    

}

#[get("/search/{query}")]
pub async fn search(req:HttpRequest,query:web::Path<String>)-> impl Responder{
    
    match isahc::get(format!("https://trefle.io/api/v1/plants?token={}&q={}",TOKEN,query)){
        
        Ok(mut response)=>{
            response.text().ok().unwrap()
        }
        Err(e)=>{
            e.to_string()
        }

    }
    

}



#[post("/login")]
pub async fn login(
    app_data: web::Data<crate::AppState>
    )-> impl Responder{
HttpResponse::new(StatusCode::OK)
}


