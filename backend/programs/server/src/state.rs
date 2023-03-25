use sqlx::{Pool, Postgres};

pub struct State {
    pub db_pool: Pool<Postgres>,
}
