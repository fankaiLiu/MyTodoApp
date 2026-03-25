use anyhow::Result;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 128;

pub fn hash_password(password: &str) -> Result<String> {
    if password.is_empty() {
        return Err(anyhow::anyhow!("密码不能为空"));
    }
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("密码哈希失败: {}", e))?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool> {
    if password.is_empty() || password_hash.is_empty() {
        return Err(anyhow::anyhow!("密码或哈希值不能为空"));
    }
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| anyhow::anyhow!("无效的密码哈希格式: {}", e))?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn validate_password_strength(password: &str) -> Result<()> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(anyhow::anyhow!(
            "密码长度至少需要 {} 个字符",
            MIN_PASSWORD_LENGTH
        ));
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(anyhow::anyhow!(
            "密码长度不能超过 {} 个字符",
            MAX_PASSWORD_LENGTH
        ));
    }
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace());
    let strength_score = [has_lower, has_upper, has_digit, has_special]
        .iter()
        .filter(|&&x| x)
        .count();
    if strength_score < 2 {
        return Err(anyhow::anyhow!(
            "密码必须包含至少两种字符类型（小写字母、大写字母、数字、特殊字符）"
        ));
    }
    Ok(())
}

pub fn generate_random_password(length: usize) -> Result<String> {
    if length < MIN_PASSWORD_LENGTH {
        return Err(anyhow::anyhow!(
            "生成的密码长度至少需要 {} 个字符",
            MIN_PASSWORD_LENGTH
        ));
    }
    if length > MAX_PASSWORD_LENGTH {
        return Err(anyhow::anyhow!(
            "生成的密码长度不能超过 {} 个字符",
            MAX_PASSWORD_LENGTH
        ));
    }
    let charset: Vec<char> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-=[]{}|;:,.<>?"
            .chars()
            .collect();
    let mut password = String::with_capacity(length);
    for _ in 0..length {
        let idx = (rand_simple_u64() as usize) % charset.len();
        password.push(charset[idx]);
    }
    Ok(password)
}

fn rand_simple_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as u64) * 1103515245 + 12345
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify() {
        let password = "TestPassword123!";
        let hash = hash_password(password).unwrap();
        assert!(verify_password(password, &hash).unwrap());
        assert!(!verify_password("WrongPassword", &hash).unwrap());
    }

    #[test]
    fn test_empty_password() {
        assert!(hash_password("").is_err());
        assert!(verify_password("", "hash").is_err());
    }

    #[test]
    fn test_validate_strength() {
        assert!(validate_password_strength("abc").is_err());
        assert!(validate_password_strength("abcd1234").is_ok());
        assert!(validate_password_strength("Abcd1234!").is_ok());
        assert!(validate_password_strength("a").is_err());
        assert!(validate_password_strength(&"a".repeat(200)).is_err());
    }

    #[test]
    fn test_generate_password() {
        let pwd = generate_random_password(16).unwrap();
        assert_eq!(pwd.len(), 16);
        let pwd = generate_random_password(8).unwrap();
        assert_eq!(pwd.len(), 8);
        assert!(generate_random_password(7).is_err());
    }
}
