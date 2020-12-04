use mongodb::{Collection, error::Error, results::InsertOneResult};
use bson::{Document, doc, oid::ObjectId};

#[derive(Debug,Clone)]
pub struct UserModel{

    collection:Collection,
}

impl UserModel {

    pub fn new(collection:Collection)->UserModel {
        UserModel{collection}
    }

    pub async  fn login(&self,email:&str,password:&str)->Result<Option<Document>,Error>{

        // self.collection.find(doc!{},None).await?
        
        Ok(
            self.collection.find_one(
                doc!{
                            "email":email,
                            "password":password
                    },None).unwrap()
        )

    }
    pub async fn signup(&self,username:&str,email:&str,password:&str)->Result<InsertOneResult,Error>{

        Ok(
            self.collection.insert_one(
                doc!{
                            "username":username,
                            "email":email,
                            "password":password,
                            "favorites":[]
                    },None).unwrap()
        )

    }

    pub async fn delete_account(&self,user_id:&str)->Result<Option<Document>,Error>{

        Ok(
            self.collection.find_one_and_delete(
                doc!{
                    "_id":ObjectId::with_string(user_id).unwrap()
                        },None
                    ).unwrap()
    
        )

    }

    pub async fn update_account(&self,user_id:&str,username:&str,email:&str)->Result<Option<Document>,Error>{
        
        Ok(
        
        self.collection.find_one_and_update(doc!{
            "_id":ObjectId::with_string(user_id).unwrap()
        },doc!{
          "$set":{
              "username":username,
              "email":email
          }  
        },None).unwrap()

    )
    }

    pub async fn update_password(&self,user_id:&str,password:&str)->Result<Option<Document>,Error>{
        
        Ok(
        
        self.collection.find_one_and_update(doc!{
            "_id":ObjectId::with_string(user_id).unwrap()
        },doc!{
          "$set":{
              "password":password,
       
          }  
        },None).unwrap()

    )
    }



    pub async fn add_favorite(&self,user_id:&str,post_id:&str)->Result<Option<Document>,Error>{
        Ok(
        self.collection.find_one_and_update(doc!{
            "_id":ObjectId::with_string(user_id).unwrap()
        },doc!{
          "$push":{
              "favorites":post_id
          }  
        },None).unwrap()
    )
    }
   

    pub async fn delete_favorite(&self,user_id:&str,post_id:&str)->Result<Option<Document>,Error>{


        Ok(

        self.collection.find_one_and_update(doc!{
            "_id":ObjectId::with_string(user_id).unwrap()
        },doc!{
            "$pull":{
                "favorites":post_id
            }
        },None).unwrap()
    )
    }
    pub async fn verfiy_user(&self,username:&str,email:&str)->Result<Option<Document>,Error>{


        Ok(

        self.collection.find_one(doc!{
            "username":username,
            "email":email
        },None).unwrap()
    )
    }

}



