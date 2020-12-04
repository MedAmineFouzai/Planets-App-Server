use actix_web::{HttpRequest,HttpResponse, Responder, http::StatusCode, get, post,delete,put, web::Json, web};


use bson::Bson;
use serde::{Deserialize,Serialize};
extern crate  jsonwebtoken as jwt;
use jwt::{encode,decode,Header,EncodingKey,DecodingKey,Validation};
use isahc::prelude::*;
const TOKEN:&str="1uLQHB7NKvy508EBLSfDXs-f54JX8FR-JcxE904OE54";


#[derive(Debug,Serialize,Deserialize)]
pub struct UserAccount {
    username: String,
    email: String,
    password:String,

}
#[derive(Debug,Serialize,Deserialize)]
pub struct UserID {
   user_id:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserAccountUpdate {
    user_id:String,
    username:String,
    email: String,
 
    
}
#[derive(Debug,Serialize,Deserialize)]
pub struct UserPasswordUpdate {
    user_id:String,
    old_password:String,
    new_password: String,
 
    
}
#[derive(Debug,Serialize,Deserialize)]
pub struct Favorite {
    user_id:String,
    post_id:String
 
}

#[derive(Debug,Serialize, Deserialize)]
struct Token {
    token: String,
}


#[get("/plants")]
pub async fn plants(

)-> impl Responder{
    
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
pub async fn search(query:web::Path<String>)-> impl Responder{
    
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
    app_data: web::Data<crate::AppState>,
    user_data:Json<UserAccount>
    )-> impl Responder{

        match app_data.models_container.user.login(&user_data.email, &user_data.password).await{
            Ok(user)=>{

                let payload = UserAccount{
                    username: user.unwrap().get("username").and_then(Bson::as_str).unwrap().to_string(),
                    email:user_data.email.to_owned(),
                    password:"".to_owned()
                };
                
            let token=encode(&Header::default(),&payload,&EncodingKey::from_secret("secret".as_ref())).unwrap();
            HttpResponse::Ok().json(Token{token:token})  
                
            }
            Err(e)=>{
                println!("Error:{}",e);
                HttpResponse::Ok().body("Error Occurred")

            }
        }
        
        


}



#[post("/signup")]
pub async fn signup(
    
    app_data: web::Data<crate::AppState>,
    user_data:Json<UserAccount>
    )-> impl Responder{

  
    match app_data.models_container.user.signup(&user_data.username, &user_data.email, &user_data.password).await{
            Ok(result)=>{

              match app_data.models_container.user.login(&user_data.email, &user_data.password).await{
                  Ok(user)=>{

                    let payload = UserAccount{
                        username:user_data.username.to_owned(),
                        email:user_data.email.to_owned(),
                        password:"".to_owned()
                    };
                     let token=encode(&Header::default(),&payload,&EncodingKey::from_secret("secret".as_ref())).unwrap();

                      println!("user object:{:?}",user.unwrap().to_string());

                      HttpResponse::Ok().json(Token{token:token})
                  }
                  Err(e)=>{
                    println!("Error:{}",e);
                    HttpResponse::Ok().body("Error Occurred")

                  }
              }
            
        }
            Err(e)=>{
                println!("Error:{}",e);
                HttpResponse::Ok().body("Error Occurred While Adding A User")
            }
    }

}


#[delete("/deleteAccount")]
pub async fn delete_account(
    req:HttpRequest,
    app_data: web::Data<crate::AppState>,
    user_id:Json<UserID>,
    )-> impl Responder{

        let basic_auth_header = req.headers().get("Authorization");
        let decoded_token= decode::<UserAccount>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
        match decoded_token {
            Ok(token)=>{
                match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                    Ok(user_data)=>{
                        println!("user_data:{:?}",user_data.unwrap());
                        
                         match app_data.models_container.user.delete_account(&user_id.user_id).await{

                         Ok(user)=>{
                                 user.unwrap().to_string()
                            },
                         Err(e)=>{
                            println!("Error :{}",e.to_string());
                            "User Not Found".to_string()
                        }
                        }
                    }
                    
                    Err(e)=>{
                        println!("Error :{}",e.to_string());
                        "User Not Allowed".to_string()
                    }
                }
            }
            Err(e)=>{
                println!("Error :{}",e.to_string());
                "User Not Authorized".to_string()
            }
        }

                
    }



    
#[put("/updateAccount")]
pub async fn update_account(
    req:HttpRequest,
    app_data: web::Data<crate::AppState>,
    user_data:Json<UserAccountUpdate>,
    
    )-> impl Responder{

        let basic_auth_header = req.headers().get("Authorization");
        let decoded_token= decode::<UserAccount>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
        match decoded_token {
            Ok(token)=>{
                match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                    Ok(user)=>{
                        println!("user_data:{:?}",user.unwrap());
                        
                         match app_data.models_container.user.update_account(&user_data.user_id,&user_data.username,&user_data.email).await{

                         Ok(user)=>{
                                 user.unwrap().to_string()
                            },
                         Err(e)=>{
                            println!("Error :{}",e.to_string());
                            "User Not Found".to_string()
                        }
                        }
                    }
                    
                    Err(e)=>{
                        println!("Error :{}",e.to_string());
                        "User Not Allowed".to_string()
                    }
                }
            }
            Err(e)=>{
                println!("Error :{}",e.to_string());
                "User Not Authorized".to_string()
            }
        }

                
    }
    



#[put("/updatePassword")]
pub async fn update_password(
        req:HttpRequest,
        app_data: web::Data<crate::AppState>,
        user_data:Json<UserPasswordUpdate>,
        
        )-> impl Responder{
    
            let basic_auth_header = req.headers().get("Authorization");
            let decoded_token= decode::<UserAccount>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
            match decoded_token {
                Ok(token)=>{
                    match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                        Ok(user)=>{
                            
                            if user.unwrap().get("password").and_then(Bson::as_str).unwrap().to_string().eq(&user_data.old_password){
                                match app_data.models_container.user.update_password(&user_data.user_id,&user_data.new_password).await{
    
                                    Ok(user)=>{
                                            user.unwrap().to_string()
                                       },
                                    Err(e)=>{
                                       println!("Error :{}",e.to_string());
                                       "User Not Found".to_string()
                                   }
                                   }
                            }else{
                                "User Not Allowed".to_string()
                            }
                            
                        }
                        
                        Err(e)=>{
                            println!("Error :{}",e.to_string());
                            "User Not Allowed".to_string()
                        }
                    }
                }
                Err(e)=>{
                    println!("Error :{}",e.to_string());
                    "User Not Authorized".to_string()
                }
            }
    
                    
        }




        
#[post("/addFavorite")]
pub async fn add_favorite(
        req:HttpRequest,
        app_data: web::Data<crate::AppState>,
        user_data:Json<Favorite>,
        
        )-> impl Responder{
    
            let basic_auth_header = req.headers().get("Authorization");
            let decoded_token= decode::<UserAccount>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
            match decoded_token {
                Ok(token)=>{
                    match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                        Ok(user)=>{
                            match app_data.models_container.user.add_favorite(&user_data.user_id, &user_data.post_id).await{
                                Ok(user)=>{
                                    user.unwrap().to_string()
                                }
                                Err(e)=>{
                                    println!("Error :{}",e.to_string());
                                    "Error Occured".to_string()
                                }
                            }

                        }
                        
                        Err(e)=>{
                            println!("Error :{}",e.to_string());
                            "User Not Allowed".to_string()
                        }
                    }
                }
                Err(e)=>{
                    println!("Error :{}",e.to_string());
                    "User Not Authorized".to_string()
                }
            }
    
                    
        }



#[delete("/deleteFavorite")]
pub async fn delete_favorite(
        req:HttpRequest,
        app_data: web::Data<crate::AppState>,
        user_data:Json<Favorite>,
        
        )-> impl Responder{
    
            let basic_auth_header = req.headers().get("Authorization");
            let decoded_token= decode::<UserAccount>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
            match decoded_token {
                Ok(token)=>{
                    match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                        Ok(user)=>{
                            match app_data.models_container.user.delete_favorite(&user_data.user_id, &user_data.post_id).await{
                                Ok(user)=>{
                                    user.unwrap().to_string()
                                }
                                Err(e)=>{
                                    println!("Error :{}",e.to_string());
                                    "Error Occured".to_string()
                                }
                            }

                        }
                        
                        Err(e)=>{
                            println!("Error :{}",e.to_string());
                            "User Not Allowed".to_string()
                        }
                    }
                }
                Err(e)=>{
                    println!("Error :{}",e.to_string());
                    "User Not Authorized".to_string()
                }
            }
    
                    
        }
