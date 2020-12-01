use mongodb::{Collection, Cursor, bson::{Document, doc, oid::ObjectId}, error::Error, results::InsertOneResult};

#[derive(Debug,Clone)]
pub struct User{
    collection:Collection,
}


impl User {

    pub fn new(collection:Collection)->User {
        User{collection}
    }

    pub async  fn login(&self,email:&str,password:&str)->Result<Cursor,Error>{

        // self.collection.find(doc!{},None).await?
        
        Ok(
            self.collection.find(
                doc!{
                            "email":email,
                            "password":password
                    },None).await?
        )

    }
    pub async fn signup(&self,username:&str,email:&str,password:&str)->Result<InsertOneResult,Error>{

        Ok(
            self.collection.insert_one(
                doc!{
                            "username":username,
                            "email":email,
                            "password":password
                    },None).await?
        )

    }

    pub async fn delete_account(&self,user_id:&str)->Result<Option<Document>,Error>{

        Ok(
            self.collection.find_one_and_delete(
                doc!{
                    "_id":ObjectId::with_string(user_id).unwrap()
                        },None
                    ).await?
    
        )

    }

    pub async fn update_account(&self,user_id:&str,username:&str)->Result<Option<Document>,Error>{
        
        Ok(
        
        self.collection.find_one_and_update(doc!{
            "_id":ObjectId::with_string(user_id).unwrap()
        },doc!{
          "$set":{
              "username":username
          }  
        },None).await?

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
        },None).await?
    )
    }
   

    pub async fn delete_favorite(&self,user_id:&str,post_id:&str)->Result<Option<Document>,Error>{


        Ok(

        self.collection.find_one_and_update(doc!{
            "_id":ObjectId::with_string(user_id).unwrap()
        },doc!{
            "$pop":{
                "favorites":post_id
            }
        },None).await?
    )
    }


}



