use lazy_static::lazy_static;
use chrono::Duration;

// Database constants
pub const MAX_DB_CONNECTIONS: u32 = 5;

// The length of cryptographic salts across the app
pub const SALT_LENGTH: usize = 16;

// Token auth configuration - auto refresh interval, token key length
pub const TOKEN_LENGTH: usize = 30;

lazy_static! {
    pub static ref TOKEN_LIFETIME: Duration = Duration::days(60);
    pub static ref REFRESH_INTERVAL: Duration = Duration::days(2);
}
