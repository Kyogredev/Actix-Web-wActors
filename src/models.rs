use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;
use crate::schema::users;
use crate::actors::db::DbActor;
use crate::actix::Addr;

pub struct AppState {   // holds address of the actor
  pub db: Addr<DbActor>
}

#[derive(Debug, Clone, Queryable, Serialize, Deserialize)]    // struct for querying
pub struct User {
  pub uuid: Uuid,
  pub name: String,
  pub age: i16,
  pub is_sub: bool
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[table_name="users"]                                        // insert structs must have this macro
pub struct NewUser {
  pub uuid: Uuid,
  pub name: String,
  pub age: i16,
  pub is_sub: bool
}

/* PAYLOADS */

#[derive(Serialize, Deserialize)]  // Expected JSON payload on user creation
pub struct UserPayload {
    pub name: String,
    pub age: i16,
}

#[derive(Serialize, Deserialize)] // Expected JSON payload on name change
pub struct NamePayload {
  pub name: String,
} 