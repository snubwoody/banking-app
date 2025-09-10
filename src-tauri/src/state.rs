use sqlx::SqlitePool;
use std::fs::{self, File};

/// Global app state.
#[derive(Clone)]
pub struct AppState {
    pool: sqlx::SqlitePool,
}

impl AppState {
    /// Initialise the [`AppState`]. This will load the data from a `data.db` file
    /// or create one if it doens't exist.
    pub async fn init() -> crate::Result<Self> {
        // TODO: store in AppData
        if !fs::exists("data.db")? {
            File::create("data.db")?;
        }

        let url = "sqlite:data.db";
        let pool = sqlx::SqlitePool::connect(url).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    /// Create an [`AppState`] from an existing pool, useful for testing.
    pub fn from_pool(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get a reference to the connection pool.
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}
