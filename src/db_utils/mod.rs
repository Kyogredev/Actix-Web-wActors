use diesel::{
  r2d2::{ConnectionManager, Pool},
  PgConnection,
};

/// Creates a pool of connections to database.
///
pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  Pool::builder()
      .build(manager)
      .expect("Error building a connection pool")
}