use actix_web::{HttpRequest, HttpResponse, Responder, http::StatusCode, delete, get, options, post, put, web::Json, web};
use bson::Bson;
use serde::{Deserialize,Serialize};
extern crate  jsonwebtoken as jwt;
use jwt::{encode,decode,Header,EncodingKey,DecodingKey,Validation};
use isahc::prelude::*;
const TOKEN:&str="1uLQHB7NKvy508EBLSfDXs-f54JX8FR-JcxE904OE54";


#[derive(Debug,Serialize,Deserialize)]
pub struct UserLoginModel {
    email: String,
    password:String,

}


#[derive(Debug,Serialize,Deserialize)]
pub struct UserSignUpModel {
    username: String,
    email: String,
    password:String,

}


#[derive(Debug,Serialize,Deserialize)]
pub struct UserPayload {
    username: String,
    email: String,
}



#[derive(Debug,Serialize,Deserialize)]
pub struct UserID {
   id:String
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserAccountUpdate {
    id:String,
    username:String,
    email: String,
 
}

#[derive(Debug,Serialize,Deserialize)]
pub struct UserPasswordUpdate {
    id:String,
    old_password:String,
    new_password: String,
     
}


#[derive(Debug,Serialize,Deserialize)]
pub struct UserObject {
    id:String,
    username:String,
    email: String,
    token:String,
    favourites:Vec<Bson>
}


#[derive(Debug,Serialize,Deserialize)]
pub struct Favorite {
    id:String,
    post_id:String
 
}

#[get("/plants")]
pub async fn plants()-> impl Responder{
    
    match isahc::get(format!("https://trefle.io/api/v1/plants?token={}",TOKEN)){
       
        Ok(mut response)=>{
    
            response.text().ok().unwrap()
        }
        Err(e)=>{

            e.to_string()
        
        }
    }           
}



#[get("/getPlant")]
pub async fn get_plant(id:web::Path<String>)-> impl Responder{
    
    match isahc::get(format!("https://trefle.io/api/v1/plants/{}?token={}",id,TOKEN)){
       
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
    
    match isahc::get(format!("https://trefle.io/api/v1/plants/search?token={}&q={}",TOKEN,query)){
        
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
user_data:Json<UserLoginModel>
)-> impl Responder{

        match app_data.models_container.user.login(&user_data.email, &user_data.password).await{
            Ok(user)=>{
                let  data=user.unwrap();
                let payload = UserObject{
                    id:data.get_object_id("_id").unwrap().to_string(),
                    username:data.get_str("username").unwrap().to_string(),
                    email:data.get_str("email").unwrap().to_string(),
                    token:String::from(""),
                    favourites:data.get_array("favourites").unwrap().to_vec()
                };
                
            let token=encode(&Header::default(),&payload,&EncodingKey::from_secret("secret".as_ref())).unwrap();
            HttpResponse::Ok().json(UserObject{
                id:data.get_object_id("_id").unwrap().to_string(),
                username:data.get_str("username").unwrap().to_string(),
                email:data.get_str("email").unwrap().to_string(),
                token:token,
                favourites:data.get_array("favourites").unwrap().to_vec()
            })  
                
            }
            Err(e)=>{
                println!("Error:{}",e);
                HttpResponse::Ok().body("Error Occurred")
            }
        }
}

// #[options("/signup")]
// pub async fn gurd()->impl Responder{
//     HttpResponse::new(StatusCode::OK())
// }

#[post("/signup")]
pub async fn signup(
    app_data: web::Data<crate::AppState>,
    user_data:Json<UserSignUpModel>
    )-> impl Responder{

    match app_data.models_container.user.signup(&user_data.username, &user_data.email, &user_data.password).await{
            Ok(_inserted_user)=>{
              match app_data.models_container.user.login(&user_data.email, &user_data.password).await{
                  Ok(user)=>{
                    let data=user.unwrap();
                    let payload = UserObject{
                        id:data.get_object_id("_id").unwrap().to_string(),
                        username:data.get_str("username").unwrap().to_string(),
                        email:data.get_str("email").unwrap().to_string(),
                        token:String::from(""),
                        favourites:data.get_array("favourites").unwrap().to_vec()
                    };
                     let token=encode(&Header::default(),&payload,&EncodingKey::from_secret("secret".as_ref())).unwrap();

                     HttpResponse::Ok().json(UserObject{
                        id:data.get_object_id("_id").unwrap().to_string(),
                        username:data.get_str("username").unwrap().to_string(),
                        email:data.get_str("email").unwrap().to_string(),
                        token:token,
                        favourites:data.get_array("favourites").unwrap().to_vec()
                    })  
                  }
                  Err(e)=>{
                    println!("Error:{}",e);
                    HttpResponse::Ok().body("Error Occurred While Loging The User")

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
        let decoded_token= decode::<UserPayload>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
        match decoded_token {
            Ok(token)=>{
                match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                    Ok(_user_data)=>{
                        
                         match app_data.models_container.user.delete_account(&user_id.id).await{

                         Ok(user)=>{
                            let data=user.unwrap();
                            HttpResponse::Ok().json(UserObject{
                                id:data.get_object_id("_id").unwrap().to_string(),
                                username:data.get_str("username").unwrap().to_string(),
                                email:data.get_str("email").unwrap().to_string(),
                                token:"".to_owned(),
                                favourites:data.get_array("favourites").unwrap().to_vec()
                            }) 
                            },
                         Err(e)=>{
                            println!("Error :{}",e.to_string());
                            HttpResponse::Ok().body("User Not Found")

                        }
                        }
                    }
                    
                    Err(e)=>{
                        println!("Error :{}",e.to_string());
                        HttpResponse::Ok().body("User Not Allowed")
                    }
                }
            }
            Err(e)=>{
                println!("Error :{}",e.to_string());
                HttpResponse::Ok().body("User Not Authorized")
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
        let decoded_token= decode::<UserPayload>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
        match decoded_token {
            Ok(token)=>{
                match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                    Ok(user)=>{
                        println!("user_data:{:?}",user.unwrap());
                        
                         match app_data.models_container.user.update_account(&user_data.id,&user_data.username,&user_data.email).await{
                        
                         Ok(_user)=>{
                            let user=app_data.models_container.user.verfiy_user(&user_data.username, &user_data.email).await.ok().unwrap();
                            let data=user.unwrap();
                            let payload = UserPayload{
                                username:data.get_str("username").unwrap().to_string(),
                                email:data.get_str("email").unwrap().to_string(),
                            };
                            let token=encode(&Header::default(),&payload,&EncodingKey::from_secret("secret".as_ref())).unwrap();

                            HttpResponse::Ok().json(UserObject{
                                id:data.get_object_id("_id").unwrap().to_string(),
                                username:data.get_str("username").unwrap().to_string(),
                                email:data.get_str("email").unwrap().to_string(),
                                token:token,
                                favourites:data.get_array("favourites").unwrap().to_vec()
                            }) 
                            },
                         Err(e)=>{
                            println!("Error :{}",e.to_string());
                            HttpResponse::Ok().body("User Not Found")
                        }
                        }
                    }
                    
                    Err(e)=>{
                        println!("Error :{}",e.to_string());
                        HttpResponse::Ok().body("User Not Allowed")
                    }
                }
            }
            Err(e)=>{
                println!("Error :{}",e.to_string());
                HttpResponse::Ok().body("User Not Authorized")
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
            let decoded_token= decode::<UserPayload>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
            match decoded_token {
                Ok(token)=>{
                    match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                        Ok(user)=>{
                            let data=user.unwrap();
                            if data.get_str("password").unwrap().to_string().eq(&user_data.old_password){
                                match app_data.models_container.user.update_password(&user_data.id,&user_data.new_password).await{
    
                                    Ok(user)=>{
                                        let data=user.unwrap();
                                        HttpResponse::Ok().json(UserObject{
                                            id:data.get_object_id("_id").unwrap().to_string(),
                                            username:data.get_str("username").unwrap().to_string(),
                                            email:data.get_str("email").unwrap().to_string(),
                                            token:"".to_owned(),
                                            favourites:data.get_array("favourites").unwrap().to_vec()
                                        }) 
                                       },
                                    Err(e)=>{
                                       println!("Error :{}",e.to_string());
                                  
                                       HttpResponse::Ok().body("User Not Found")
                                   }
                                   }
                            }else{

                                HttpResponse::Ok().body("Password incorrect")

                            }

                        }
    
                        Err(e)=>{
                            println!("Error :{}",e.to_string());
                            HttpResponse::Ok().body("User Not Allowed")
                        }
                    }
                }
                Err(e)=>{
                    println!("Error :{}",e.to_string());
                    HttpResponse::Ok().body("User Not Authorized")
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
            let decoded_token= decode::<UserPayload>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
            match decoded_token {
                Ok(token)=>{
                    match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                        Ok(user)=>{
                            let data =user.unwrap();
                            match app_data.models_container.user.add_favorite(&user_data.id, &user_data.post_id).await{
                                Ok(_user)=>{
                                    let user=app_data.models_container.user.verfiy_user(&data.get_str("username").unwrap().to_string(),&data.get_str("email").unwrap().to_string()).await.ok().unwrap();
                                    let data=user.unwrap();
                                    HttpResponse::Ok().json(UserObject{
                                            id:data.get_object_id("_id").unwrap().to_string(),
                                            username:data.get_str("username").unwrap().to_string(),
                                            email:data.get_str("email").unwrap().to_string(),
                                            token:"".to_owned(),
                                            favourites:data.get_array("favourites").unwrap().to_vec()
                                        })
                                }
                                Err(e)=>{
                                    println!("Error :{}",e.to_string());
                            

                                    HttpResponse::Ok().body("Error Occured")
                                }
                            }

                        }
                        
                        Err(e)=>{
                            println!("Error :{}",e.to_string());
                      
                            HttpResponse::Ok().body("User Not Allowed")
                        }
                    }
                }
                Err(e)=>{
                    println!("Error :{}",e.to_string());
                
                    HttpResponse::Ok().body("User Not Authorized")
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
            let decoded_token= decode::<UserPayload>(&basic_auth_header.unwrap().to_str().unwrap().replace("Bearer ", ""), &DecodingKey::from_secret("secret".as_ref()), &Validation {validate_exp:false, ..Default::default()});
            match decoded_token {
                Ok(token)=>{
                    match app_data.models_container.user.verfiy_user(&token.claims.username,&token.claims.email).await{
                        Ok(user)=>{
                            let data =user.unwrap();
                            match app_data.models_container.user.delete_favorite(&user_data.id, &user_data.post_id).await{
                                Ok(_user)=>{
                                    let user=app_data.models_container.user.verfiy_user(&data.get_str("username").unwrap().to_string(),&data.get_str("email").unwrap().to_string()).await.ok().unwrap();
                                    let data=user.unwrap();
                                    HttpResponse::Ok().json(UserObject{
                                            id:data.get_object_id("_id").unwrap().to_string(),
                                            username:data.get_str("username").unwrap().to_string(),
                                            email:data.get_str("email").unwrap().to_string(),
                                            token:"".to_owned(),
                                            favourites:data.get_array("favourites").unwrap().to_vec()
                                        })
                                }
                                Err(e)=>{
                                    println!("Error :{}",e.to_string());
                            

                                    HttpResponse::Ok().body("Error Occured")
                                }
                            }

                        }
                        
                        Err(e)=>{
                            println!("Error :{}",e.to_string());
                      
                            HttpResponse::Ok().body("User Not Allowed")
                        }
                    }
                }
                Err(e)=>{
                    println!("Error :{}",e.to_string());
                
                    HttpResponse::Ok().body("User Not Authorized")
                }
            }
    
                    
        }
