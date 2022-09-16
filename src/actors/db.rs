use crate::actix::{Actor, Handler, Message, SyncContext};
use crate::diesel::prelude::*;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::{
		r2d2::{ConnectionManager, Pool},
		PgConnection,
};
use ::uuid::Uuid as genUuid;


/* ACTOR */

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
		type Context = SyncContext<Self>;
}


/* MESSAGES */

#[derive(Message)]
#[rtype(result = "QueryResult<User>")] // expected message on creation
pub struct Create {
		pub name: String,
		pub age: i16,
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")] // expected message on update (notice that uuid must be provided for retrieval)
pub struct Update {
		pub uuid: genUuid,
		pub name: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<usize>")] // expected message on delete
pub struct Delete {
		pub uuid: genUuid,
}

/// Retrieves all users.
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct GetAll;


/* HANDLERS */

impl Handler<Create> for DbActor {
		type Result = QueryResult<User>; // must be the same rtype as the Message it handles

		fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
				let mut conn = self.0.get().expect("Unable to connect to pool");
				let new_user = NewUser {
						uuid: genUuid::new_v4(),
						name: msg.name,
						age: msg.age,
						is_sub: false,
				};

				diesel::insert_into(users)
						.values(new_user)
						.get_result::<User>(&mut conn)                      // RETURNING *
		}
}

impl Handler<Update> for DbActor {
		type Result = QueryResult<User>; 

		fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
				let mut conn = self.0.get().expect("Unable to connect to pool");

				diesel::update(users)
						.filter(uuid.eq(msg.uuid))                      // WHERE uuid = $1
						.set(name.eq(msg.name))                         // SET name = $2
						.get_result::<User>(&mut conn)
		}
}

impl Handler<Delete> for DbActor {
		type Result = QueryResult<usize>;  

		fn handle(&mut self, msg: Delete, _: &mut Self::Context) -> Self::Result {
			let mut conn = self.0.get().expect("Unable to get a connection");

			diesel::delete(users)
							.filter(uuid.eq(msg.uuid))
							.execute(&mut conn)
		}
}

impl Handler<GetAll> for DbActor {
		type Result = QueryResult<Vec<User>>;

		fn handle(&mut self, _msg: GetAll, _: &mut Self::Context) -> Self::Result {
				let mut conn = self.0.get().expect("Unable to connect to pool");
				users
				.get_results::<User>(&mut conn)
				// Vec<T> return type is implicit in get_results::<T>  
		}
}
