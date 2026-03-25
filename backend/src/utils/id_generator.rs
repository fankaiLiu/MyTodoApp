/*
雪花ID：sonyflake id: 612022676378137381

带类型位的雪花 ID 生成器

ID 结构：[类型位(8位)] [雪花ID(56位)]
- 高8位：存储实体类型，支持256种实体类型
- 低56位：存储雪花ID，保证全局唯一

优点：
1. 可读性好：十六进制显示时类型位一目了然
2. 类型安全：运行时可以验证ID类型
3. 性能优秀：仍然是u64，数据库索引高效
4. 向后兼容：可以轻松提取原始雪花ID
5. 扩展性好：支持256种实体类型
*/

use sonyflake::Sonyflake;

// 测试sonflake id
pub fn test_sonyflake_id() -> Result<u64, sonyflake::Error> {
    let sf = Sonyflake::new()?;
    let next_id = sf.next_id()?;
    tracing::info!("sonyflake id: {}", next_id);
    Ok(next_id)
}

/// 实体类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    User = 1,        // 用户
    Team = 2,        // 团队
    SubTeam = 3,     // 子团队
    Task = 4,        // 任务
    TeamInvite = 5,  // 团队邀请
    JoinRequest = 6, // 加入申请
    UserLog = 7,     // 用户日志
    TaskLog = 8,     // 任务日志
    TeamLog = 9,     // 团队日志
}

impl EntityType {
    /// 从类型代码创建EntityType
    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            1 => Some(EntityType::User),
            2 => Some(EntityType::Team),
            3 => Some(EntityType::SubTeam),
            4 => Some(EntityType::Task),
            5 => Some(EntityType::TeamInvite),
            6 => Some(EntityType::JoinRequest),
            7 => Some(EntityType::UserLog),
            8 => Some(EntityType::TaskLog),
            9 => Some(EntityType::TeamLog),
            _ => None,
        }
    }

    /// 获取类型代码
    pub fn code(&self) -> u8 {
        *self as u8
    }

    /// 获取类型名称
    pub fn name(&self) -> &'static str {
        match self {
            EntityType::User => "User",
            EntityType::Team => "Team",
            EntityType::SubTeam => "SubTeam",
            EntityType::Task => "Task",
            EntityType::TeamInvite => "TeamInvite",
            EntityType::JoinRequest => "JoinRequest",
            EntityType::UserLog => "UserLog",
            EntityType::TaskLog => "TaskLog",
            EntityType::TeamLog => "TeamLog",
        }
    }
}

/// 带类型位的ID生成器
pub struct TypedIdGenerator {
    sonyflake: Sonyflake,
}

impl TypedIdGenerator {
    /// 创建新的ID生成器
    pub fn new() -> Result<Self, sonyflake::Error> {
        Ok(Self {
            sonyflake: Sonyflake::new()?,
        })
    }

    /// 创建自定义配置的ID生成器
    /*
        何时使用
    情况	            推荐方法
    单机应用	        new()
    开发/测试	        new()
    生产环境（单机）	new()
    生产环境（分布式）	with_settings()
    需要自定义起始时间	with_settings()
    需要避免ID冲突	    with_settings() + check_machine_id: true

         */
    // pub fn with_settings(settings: sonyflake::SonyflakeSettings) -> Result<Self, sonyflake::Error> {
    //     Ok(Self {
    //         sonyflake: Sonyflake::new_with_settings(settings)?,
    //     })
    // }

    /// 生成带类型位的ID
    ///
    /// # 参数
    /// * `entity_type` - 实体类型
    ///
    /// # 返回
    /// 带类型位的u64 ID
    ///
    /// # 示例
    /// ```
    /// let generator = TypedIdGenerator::new().unwrap();
    /// let user_id = generator.generate(EntityType::User);
    /// println!("user_id: 0x{:016x}", user_id); // 0x0112345678901234
    /// ```
    pub fn generate(&self, entity_type: EntityType) -> u64 {
        let snowflake_id = self.sonyflake.next_id().unwrap();
        // 高8位存储类型，低56位存储雪花ID
        ((entity_type.code() as u64) << 56) | (snowflake_id & 0x00FFFFFFFFFFFFFF)
    }

    /// 解析ID的类型
    ///
    /// # 参数
    /// * `id` - 带类型位的ID
    ///
    /// # 返回
    /// 实体类型，如果类型无效则返回None
    pub fn parse_type(id: u64) -> Option<EntityType> {
        let type_code = (id >> 56) as u8;
        EntityType::from_code(type_code)
    }

    /// 提取原始雪花ID
    ///
    /// # 参数
    /// * `id` - 带类型位的ID
    ///
    /// # 返回
    /// 原始雪花ID（低56位）
    pub fn extract_snowflake(id: u64) -> u64 {
        id & 0x00FFFFFFFFFFFFFF
    }

    /// 验证ID是否为指定类型
    ///
    /// # 参数
    /// * `id` - 带类型位的ID
    /// * `expected_type` - 期望的实体类型
    ///
    /// # 返回
    /// 如果ID类型匹配则返回true
    pub fn validate_type(id: u64, expected_type: EntityType) -> bool {
        Self::parse_type(id) == Some(expected_type)
    }

    /// 格式化ID为十六进制字符串（带类型标识）
    ///
    /// # 参数
    /// * `id` - 带类型位的ID
    ///
    /// # 返回
    /// 格式化的字符串，如 "User:0x0112345678901234"
    pub fn format_id(id: u64) -> String {
        match Self::parse_type(id) {
            Some(entity_type) => format!("{}:0x{:016x}", entity_type.name(), id),
            None => format!("Unknown:0x{:016x}", id),
        }
    }
}

impl Default for TypedIdGenerator {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| panic!("Failed to create TypedIdGenerator"))
    }
}

/// 全局ID生成器实例（线程安全）
static GLOBAL_GENERATOR: std::sync::OnceLock<TypedIdGenerator> = std::sync::OnceLock::new();

/// 获取全局ID生成器实例
pub fn global_generator() -> &'static TypedIdGenerator {
    GLOBAL_GENERATOR.get_or_init(|| TypedIdGenerator::new().unwrap())
}

/// 便捷函数：生成用户ID
pub fn generate_user_id() -> u64 {
    global_generator().generate(EntityType::User)
}

/// 便捷函数：生成团队ID
pub fn generate_team_id() -> u64 {
    global_generator().generate(EntityType::Team)
}

/// 便捷函数：生成子团队ID
pub fn generate_sub_team_id() -> u64 {
    global_generator().generate(EntityType::SubTeam)
}

/// 便捷函数：生成任务ID
pub fn generate_task_id() -> u64 {
    global_generator().generate(EntityType::Task)
}

/// 便捷函数：生成团队邀请ID
pub fn generate_team_invite_id() -> u64 {
    global_generator().generate(EntityType::TeamInvite)
}

/// 便捷函数：生成加入申请ID
pub fn generate_join_request_id() -> u64 {
    global_generator().generate(EntityType::JoinRequest)
}

/// 便捷函数：生成用户日志ID
pub fn generate_user_log_id() -> u64 {
    global_generator().generate(EntityType::UserLog)
}

/// 便捷函数：生成任务日志ID
pub fn generate_task_log_id() -> u64 {
    global_generator().generate(EntityType::TaskLog)
}

/// 便捷函数：生成团队日志ID
pub fn generate_team_log_id() -> u64 {
    global_generator().generate(EntityType::TeamLog)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ids() {
        let generator = TypedIdGenerator::new().unwrap();

        let user_id = generator.generate(EntityType::User);
        let team_id = generator.generate(EntityType::Team);
        let task_id = generator.generate(EntityType::Task);

        println!("user_id: 0x{:016x}", user_id);
        println!("team_id: 0x{:016x}", team_id);
        println!("task_id: 0x{:016x}", task_id);

        assert!(user_id != team_id);
        assert!(team_id != task_id);
    }

    #[test]
    fn test_parse_type() {
        let generator = TypedIdGenerator::new().unwrap();

        let user_id = generator.generate(EntityType::User);
        let team_id = generator.generate(EntityType::Team);

        assert_eq!(
            TypedIdGenerator::parse_type(user_id),
            Some(EntityType::User)
        );
        assert_eq!(
            TypedIdGenerator::parse_type(team_id),
            Some(EntityType::Team)
        );
    }

    #[test]
    fn test_extract_snowflake() {
        let generator = TypedIdGenerator::new().unwrap();

        let user_id = generator.generate(EntityType::User);
        let snowflake = TypedIdGenerator::extract_snowflake(user_id);

        assert!(snowflake < 0x0100000000000000);
    }

    #[test]
    fn test_validate_type() {
        let generator = TypedIdGenerator::new().unwrap();

        let user_id = generator.generate(EntityType::User);

        assert!(TypedIdGenerator::validate_type(user_id, EntityType::User));
        assert!(!TypedIdGenerator::validate_type(user_id, EntityType::Team));
    }

    #[test]
    fn test_format_id() {
        let generator = TypedIdGenerator::new().unwrap();

        let user_id = generator.generate(EntityType::User);
        let formatted = TypedIdGenerator::format_id(user_id);

        assert!(formatted.starts_with("User:0x"));
        println!("formatted: {}", formatted);
    }

    #[test]
    fn test_convenience_functions() {
        let user_id = generate_user_id();
        let team_id = generate_team_id();

        assert_eq!(
            TypedIdGenerator::parse_type(user_id),
            Some(EntityType::User)
        );
        assert_eq!(
            TypedIdGenerator::parse_type(team_id),
            Some(EntityType::Team)
        );
    }
}
