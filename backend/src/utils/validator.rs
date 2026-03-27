use anyhow::Result;

const MIN_USERNAME_LENGTH: usize = 3;
const MAX_USERNAME_LENGTH: usize = 32;
const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 128;
const MIN_TASK_NAME_LENGTH: usize = 1;
const MAX_TASK_NAME_LENGTH: usize = 100;
const MIN_TEAM_NAME_LENGTH: usize = 2;
const MAX_TEAM_NAME_LENGTH: usize = 50;
const MAX_DESCRIPTION_LENGTH: usize = 500;
const MAX_KEYWORDS_COUNT: usize = 20;
const MAX_TEAM_MEMBERS: u16 = 1000;

pub fn validate_user_username(username: &str) -> Result<()> {
    if username.is_empty() {
        return Err(anyhow::anyhow!("用户名不能为空"));
    }
    let len = username.len();
    if len < MIN_USERNAME_LENGTH {
        return Err(anyhow::anyhow!(
            "用户名至少需要 {} 个字符",
            MIN_USERNAME_LENGTH
        ));
    }
    if len > MAX_USERNAME_LENGTH {
        return Err(anyhow::anyhow!(
            "用户名不能超过 {} 个字符",
            MAX_USERNAME_LENGTH
        ));
    }
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(anyhow::anyhow!("用户名只能包含字母、数字、下划线和连字符"));
    }
    if username
        .chars()
        .next()
        .map(|c| c.is_numeric())
        .unwrap_or(false)
    {
        return Err(anyhow::anyhow!("用户名不能以数字开头"));
    }
    Ok(())
}

pub fn validate_user_email(email: &str) -> Result<()> {
    if email.is_empty() {
        return Err(anyhow::anyhow!("邮箱不能为空"));
    }
    let email_regex = regex_lite::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map_err(|e| anyhow::anyhow!("正则表达式错误: {}", e))?;
    if !email_regex.is_match(email) {
        return Err(anyhow::anyhow!("邮箱格式无效"));
    }
    Ok(())
}

pub fn validate_user_phone(phone: &str) -> Result<()> {
    if phone.is_empty() {
        return Err(anyhow::anyhow!("手机号不能为空"));
    }
    let phone_regex = regex_lite::Regex::new(r"^1[3-9]\d{9}$")
        .map_err(|e| anyhow::anyhow!("正则表达式错误: {}", e))?;
    if !phone_regex.is_match(phone) {
        return Err(anyhow::anyhow!("手机号格式无效"));
    }
    Ok(())
}

pub fn validate_user_password(password: &str) -> Result<()> {
    if password.is_empty() {
        return Err(anyhow::anyhow!("密码不能为空"));
    }
    let len = password.len();
    if len < MIN_PASSWORD_LENGTH {
        return Err(anyhow::anyhow!(
            "密码长度至少需要 {} 个字符",
            MIN_PASSWORD_LENGTH
        ));
    }
    if len > MAX_PASSWORD_LENGTH {
        return Err(anyhow::anyhow!(
            "密码长度不能超过 {} 个字符",
            MAX_PASSWORD_LENGTH
        ));
    }
    let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
    let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    let strength = [has_lower, has_upper, has_digit, has_special]
        .iter()
        .filter(|&&x| x)
        .count();
    if strength < 2 {
        return Err(anyhow::anyhow!(
            "密码必须包含至少两种类型的字符(大小写字母、数字、特殊字符)"
        ));
    }
    Ok(())
}

pub fn validate_user_description(description: &str) -> Result<()> {
    let len = description.len();
    if len > MAX_DESCRIPTION_LENGTH {
        return Err(anyhow::anyhow!(
            "用户描述不能超过 {} 个字符",
            MAX_DESCRIPTION_LENGTH
        ));
    }
    Ok(())
}

pub fn validate_task_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("任务名称不能为空"));
    }
    let len = name.len();
    if len < MIN_TASK_NAME_LENGTH {
        return Err(anyhow::anyhow!(
            "任务名称至少需要 {} 个字符",
            MIN_TASK_NAME_LENGTH
        ));
    }
    if len > MAX_TASK_NAME_LENGTH {
        return Err(anyhow::anyhow!(
            "任务名称不能超过 {} 个字符",
            MAX_TASK_NAME_LENGTH
        ));
    }
    Ok(())
}

pub fn validate_task_description(description: &str) -> Result<()> {
    let len = description.len();
    if len > MAX_DESCRIPTION_LENGTH {
        return Err(anyhow::anyhow!(
            "任务描述不能超过 {} 个字符",
            MAX_DESCRIPTION_LENGTH
        ));
    }
    Ok(())
}

pub fn validate_task_priority(priority: u8) -> Result<()> {
    if priority > 10 {
        return Err(anyhow::anyhow!("任务优先级不能超过10"));
    }
    Ok(())
}

pub fn validate_task_deadline(deadline: i64) -> Result<()> {
    let now = chrono::Utc::now().timestamp();
    if deadline < now - 86400 {
        return Err(anyhow::anyhow!("截止时间不能早于昨天"));
    }
    Ok(())
}

pub fn validate_task_keywords(keywords: &[String]) -> Result<()> {
    if keywords.len() > MAX_KEYWORDS_COUNT {
        return Err(anyhow::anyhow!(
            "任务标签数量不能超过 {}",
            MAX_KEYWORDS_COUNT
        ));
    }
    for keyword in keywords {
        if keyword.len() > 20 {
            return Err(anyhow::anyhow!("每个标签长度不能超过20个字符"));
        }
    }
    Ok(())
}

pub fn validate_team_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("团队名称不能为空"));
    }
    let len = name.len();
    if len < MIN_TEAM_NAME_LENGTH {
        return Err(anyhow::anyhow!(
            "团队名称至少需要 {} 个字符",
            MIN_TEAM_NAME_LENGTH
        ));
    }
    if len > MAX_TEAM_NAME_LENGTH {
        return Err(anyhow::anyhow!(
            "团队名称不能超过 {} 个字符",
            MAX_TEAM_NAME_LENGTH
        ));
    }
    Ok(())
}

pub fn validate_team_description(description: &str) -> Result<()> {
    let len = description.len();
    if len > MAX_DESCRIPTION_LENGTH {
        return Err(anyhow::anyhow!(
            "团队描述不能超过 {} 个字符",
            MAX_DESCRIPTION_LENGTH
        ));
    }
    Ok(())
}

pub fn validate_team_member_limit(limit: u16) -> Result<()> {
    if limit > MAX_TEAM_MEMBERS {
        return Err(anyhow::anyhow!("团队成员上限不能超过 {}", MAX_TEAM_MEMBERS));
    }
    Ok(())
}

pub fn validate_id(id: u64) -> Result<()> {
    if id == 0 {
        return Err(anyhow::anyhow!("ID不能为0"));
    }
    Ok(())
}

pub fn validate_not_empty(value: &str, field_name: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(anyhow::anyhow!("{}不能为空", field_name));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_user_username() {
        assert!(validate_user_username("john_doe").is_ok());
        assert!(validate_user_username("john").is_ok());
        assert!(validate_user_username("jo").is_err());
        assert!(validate_user_username("a").is_err());
        assert!(validate_user_username("a".repeat(50).as_str()).is_err());
        assert!(validate_user_username("123abc").is_err());
        assert!(validate_user_username("abc@123").is_err());
    }

    #[test]
    fn test_validate_user_email() {
        assert!(validate_user_email("test@example.com").is_ok());
        assert!(validate_user_email("user.name+tag@domain.co.uk").is_ok());
        assert!(validate_user_email("invalid").is_err());
        assert!(validate_user_email("@example.com").is_err());
    }

    #[test]
    fn test_validate_user_phone() {
        assert!(validate_user_phone("13812345678").is_ok());
        assert!(validate_user_phone("19912345678").is_ok());
        assert!(validate_user_phone("1234567890").is_err());
        assert!(validate_user_phone("1381234567").is_err());
    }

    #[test]
    fn test_validate_user_password() {
        assert!(validate_user_password("Password1").is_ok());
        assert!(validate_user_password("Pass@word1").is_ok());
        assert!(validate_user_password("12345678").is_err());
        assert!(validate_user_password("abcdefgh").is_err());
    }

    #[test]
    fn test_validate_task_name() {
        assert!(validate_task_name("Task 1").is_ok());
        assert!(validate_task_name("").is_err());
        assert!(validate_task_name(&"a".repeat(150)).is_err());
    }

    #[test]
    fn test_validate_team_name() {
        assert!(validate_team_name("My Team").is_ok());
        assert!(validate_team_name("A").is_err());
        assert!(validate_team_name("").is_err());
    }

    #[test]
    fn test_validate_id() {
        assert!(validate_id(1).is_ok());
        assert!(validate_id(0).is_err());
    }
}
