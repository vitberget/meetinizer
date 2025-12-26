use argon2::{Config, verify_encoded};
use rand::Rng;
use rpassword::prompt_password;
use tracing::warn;

use crate::config::get_admin_hash;

pub fn generate_admin_hash() {
    let password_1 = prompt_password("Your password   : ").unwrap();
    let password_2 = prompt_password("Repeat password : ").unwrap();

    if password_1 == password_2 {
        argon_hashy(&password_1);
    } else {
        warn!("Admin password mismatch!");
    }
}

fn argon_hashy(password: &str) {
    let salt: Vec<u8> = rand::rng()
        .random_iter::<u8>()
        .take(30)
        .collect();
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();

    eprintln!("Insert into settings.toml:");
    println!("[admin]");
    println!("hash=\"{hash}\"");
}

pub fn is_correct_admin_password(password: &str) -> anyhow::Result<bool> {
    let hash = get_admin_hash()?;
    let matches = verify_encoded(&hash, password.as_bytes())?;

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_correct_password() -> anyhow::Result<()> {
        let b = is_correct_admin_password("123")?; 
        assert!(b);
        Ok(())
    }
    #[test]
    #[ignore]
    fn test_inccorrect_password() -> anyhow::Result<()> {
        let b = is_correct_admin_password("abc")?; 
        assert!(!b);
        Ok(())
    }
}
