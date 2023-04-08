use crate::constants::SALT_LENGTH;
use crate::error::ServerResult;
use argon2::{Config, ThreadMode, Variant, Version};
use lazy_static::lazy_static;
use rand::distributions::Alphanumeric;
use rand::distributions::DistString;
use std::ops::Deref;
use std::thread::available_parallelism;

#[inline]
pub fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), length)
}

fn get_argon_config<'a>(passes: u32) -> Config<'a> {
    let parallelism = available_parallelism().unwrap().get() as u32;
    Config {
        ad: &[],
        hash_length: 32,
        lanes: parallelism,
        mem_cost: 4096,
        secret: &[],
        thread_mode: ThreadMode::Parallel,
        time_cost: passes,
        variant: Variant::default(),
        version: Version::default(),
    }
}

lazy_static! {

    // Passwords should be heavily encrypted
    static ref PASSWORD_CONFIG: Config<'static> = get_argon_config(
        10
    );
    // Tokens only need to be lightly encrypted, and we want this to be fast
    static ref TOKEN_CONFIG: Config<'static> = get_argon_config(
        2
    );
}

#[inline]
fn hash_string(string: String, config: &Config) -> ServerResult<String> {
    let salt = random_string(SALT_LENGTH);
    Ok(argon2::hash_encoded(
        string.as_ref(),
        salt.as_ref(),
        config,
    )?)
}

pub fn hash_password(password: String) -> ServerResult<String> {
    hash_string(password, PASSWORD_CONFIG.deref())
}

pub fn hash_token(token: String) -> ServerResult<String> {
    hash_string(token, TOKEN_CONFIG.deref())
}

#[inline]
pub fn verify_hash(hash: String, secret: String) -> ServerResult<bool> {
    Ok(argon2::verify_encoded(hash.as_ref(), secret.as_ref())?)
}
