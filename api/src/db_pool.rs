use r2d2::{Pool, PooledConnection};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, State};

type ManagedPgConn = ConnectionManager<PgConnection>;
pub struct DbConn(pub PooledConnection<ManagedPgConn>);

// Implementation of Rocket FromRequest trait https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html
// Attempts to request a connection from the connection pool
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool<ManagedPgConn>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

pub fn init(database_url: &str) -> Pool<ManagedPgConn> {
    let manager = ConnectionManager::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap()
}