use diesel::{
    r2d2::{ConnectionManager, Pool},
    MysqlConnection,
};

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection_pool()->DbPool {
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .max_size(100)
        .build(manager)
        .expect("Failed to create pool.");
    pool
}