
#### 1.1 工具函数层
- [完成] **雪花 ID 生成器** - 创建 `backend/src/utils/id_generator.rs`
  - 使用 Sonyflake 生成全局唯一 ID
  - 提供便捷的 ID 生成函数
  
- [完成] **密码哈希工具** - 创建 `backend/src/utils/utils_passwd.rs`
  - 使用 Argon2 
  - 密码验证函数
  - 密码强度验证

- [完成] **数据验证工具** - 创建 `backend/src/utils/validator.rs`
  - 请求参数验证
  - 数据格式校验
  - 自定义验证规则

#### 1.2 数据库操作层（核心基础）
- [完成] **用户数据库操作** - 完善 `backend/src/db/db_user.rs`
  ```
  基础 CRUD 操作：
  1. create_user() - 创建用户
  2. get_user_by_id() - 根据 ID 查询
  3. get_user_by_email() - 根据邮箱查询（登录需要）
  4. get_user_by_username() - 根据用户名查询（登录需要）
  5. update_user() - 通用更新（支持部分字段更新）
  6. delete_user() - 删除用户

  特殊业务逻辑：
  7. update_user_password() - 更新密码（需要哈希验证）
  8. update_last_login_time() - 更新最后登录时间（自动设置）
  9. get_user_teams() - 获取用户团队列表
  10. add_user_team() - 添加团队关联
  11. remove_user_team() - 移除团队关联
  ```

- [完成] **任务数据库操作** - 完善 `backend/src/db/db_task.rs`
  ```
  基础 CRUD 操作：
  1. create_task() - 创建任务
  2. get_task_by_id() - 根据 ID 查询
  3. list_tasks() - 获取任务列表（支持多条件筛选：用户/团队/负责人/状态/优先级/截止时间）
  4. update_task() - 通用更新（支持部分字段更新）
  5. delete_task() - 删除任务

  特殊业务逻辑：
  6. complete_task() - 完成任务（状态变更特殊处理）
  ```

- [完成] **团队数据库操作** - 完善 `backend/src/db/db_team.rs`
  ```
  基础 CRUD 操作：
  1. create_team() - 创建团队
  2. get_team_by_id() - 根据 ID 查询
  3. list_teams() - 获取团队列表（支持筛选：负责人/用户）
  4. update_team() - 通用更新（支持部分字段更新）
  5. delete_team() - 删除团队

  成员管理：
  6. add_team_member() - 添加成员
  7. remove_team_member() - 移除成员
  8. get_team_members() - 获取成员列表
  9. update_member_role() - 更新成员角色
  10. check_team_membership() - 检查用户是否是团队成员

  邀请和申请管理：
  11. create_team_invite() - 创建邀请
  12. get_team_invites() - 获取团队邀请列表
  13. update_team_invite_status() - 更新邀请状态
  14. create_join_request() - 创建加入申请
  15. get_join_requests() - 获取加入申请列表（团队/用户）
  16. update_join_request_status() - 更新申请状态
  ```

#### 1.3 日志数据库操作
- [完成] **用户日志操作** - 创建 `backend/src/db/db_user_log.rs`
  ```
  1. create_user_log() - 创建用户日志
  2. list_user_logs() - 获取用户日志列表（支持筛选：操作类型/日期范围，支持分页）
  ```

- [完成] **任务日志操作** - 创建 `backend/src/db/db_task_log.rs`
  ```
  1. create_task_log() - 创建任务日志
  2. list_task_logs() - 获取任务日志列表（支持筛选：操作者/操作类型，支持分页）
  ```

- [完成] **团队日志操作** - 创建 `backend/src/db/db_team_log.rs`
  ```
  1. create_team_log() - 创建团队日志
  2. list_team_logs() - 获取团队日志列表（支持筛选：操作者/操作类型/目标，支持分页）
  ```

#### 1.4 子团队数据库操作
- [完成] **子团队操作** - 创建 `backend/src/db/db_sub_team.rs`
  ```
  基础 CRUD 操作：
  1. create_sub_team() - 创建子团队
  2. get_sub_team_by_id() - 根据ID获取子团队
  3. list_sub_teams() - 获取子团队列表（支持筛选：团队/负责人）
  4. update_sub_team() - 更新子团队信息
  5. delete_sub_team() - 删除子团队
  ```

- [完成] **子团队成员操作** - 创建 `backend/src/db/db_sub_team_member.rs`
  ```
  1. add_sub_team_member() - 添加子团队成员
  2. remove_sub_team_member() - 移除子团队成员
  3. get_sub_team_members() - 获取子团队成员列表
  4. update_member_level() - 更新子团队成员级别
  ```

---

### 第二阶段：认证和授权（依赖第一阶段）

#### 2.1 JWT 认证
- [完成] **JWT 工具** - 创建 `backend/src/utils/jwt.rs`
  - Token 生成（登录时使用）
  - Token 验证（中间件使用）
  - [可选] Token 刷新（后续根据需求添加）

#### 2.2 中间件（执行顺序：日志 → 认证 → 权限）
- [完成] **日志中间件** - 创建 `backend/src/middleware/logging.rs`
  - 使用 Salvo 内置 `Logger` 组件
  - [可选] 自定义请求日志（耗时统计、请求ID追踪）

- [完成] **认证中间件** - 创建 `backend/src/middleware/auth.rs`
  - JWT Token 验证
  - 用户信息提取
  - 认证状态检查（设置 user_id 到 Depot）

- [完成] **权限中间件** - 创建 `backend/src/middleware/permission.rs`
  
  ##### 2.2.3.1 角色定义——[完成]
  - 系统角色枚举 (Owner, Admin, Member)
  - 团队成员角色枚举 (Owner, Admin, Member)
  - 角色层级和权限判断
  <!-- 后续升级方案：
  是否可以给指定级别添加特定权限，团队创建者默认拥有最高权限，可以编辑其他级别拥有哪些权限，然后再指定成员的级别
  级别类型(u8) 0~255，还可以给级别添加别称，方便区分
  0：没有权限(默认最低权限)、255：所有权限(默认最高权限)
  比如创建者可以指定 [1,3,5] 只能查看任务，不能创建+编辑任务，[2,4,6] 只能创建+编辑任务，就像军营中的工兵、炊事兵、通信兵一样
   -->
  
  ##### 2.2.3.2 权限服务——[完成]
  - PermissionService 权限检查逻辑
  - 资源所有权检查 (用户/任务/团队/子团队)
  - 团队权限检查
  
  ##### 2.2.3.3 认证检查中间件——[完成]
  - require_auth - 检查用户登录状态
  - 设置用户 ID 到 Depot
  

### 第三阶段：用户 API（依赖第二阶段）

#### 3.1 用户后端架构
```
HTTP 请求
    ↓
routes/user_routes.rs     (路由定义)
    ↓
handlers/user_handler.rs  (处理请求)
    ↓
services/user_service.rs  (业务逻辑)
    ↓
db/db_user.rs            (数据库操作)
```

#### 3.2 用户 API 端点
- [完成] **注册接口** - `POST /api/users/register`
- [完成] **登录接口** - `POST /api/users/login`
- [完成] **获取用户信息** - `GET /api/users/{user_id}`
- [完成] **更新用户信息** - `PUT /api/users/{user_id}`
- [完成] **修改密码** - `PUT /api/users/{user_id}/password`
- [完成] **更新设置** - `PUT /api/users/{user_id}/settings`
- [完成] **获取用户团队** - `GET /api/users/{user_id}/teams`
- [完成] **获取用户日志** - `GET /api/users/{user_id}/logs`

---

### 第四阶段：前端基础（依赖第三阶段） [无单元测试]

#### 4.1 基础组件
- [完成] **UI 组件库** - 创建 `frontend/src/components/`
- Button 组件
- Input 组件
- Form 组件
- Card 组件
- Modal 组件
- Loading 组件

#### 4.2 状态管理
- [完成] **全局状态** - 创建 `frontend/src/store/`
- 用户状态
- 任务状态
- 团队状态
- 主题状态

#### 4.3 API 客户端
- [完成] **HTTP 客户端** - 创建 `frontend/src/api/`
- 请求封装
- 错误处理
- Token 管理
- 请求拦截器

#### 4.4 路由配置
- [完成] **路由设置** - 完善 `frontend/src/router.rs`
- 登录页路由
- 注册页路由
- Dashboard 路由
- 任务管理路由
- 团队管理路由

---

### 第五阶段：前端用户界面（依赖第四阶段） [无单元测试]

#### 5.1 认证页面
- [完成] **登录页面** - `frontend/src/pages/login.rs`
- [完成] **注册页面** - `frontend/src/pages/register.rs`

#### 5.2 主题切换
- [完成] **主题切换组件** - `frontend/src/components/theme_switcher.rs`
- [完成] **主题持久化** - LocalStorage 存储

#### 5.3 用户信息页面
- [完成] **用户信息展示** - `frontend/src/pages/profile.rs`
- [完成] **用户信息编辑** - 表单组件
- [完成] **密码修改** - 密码修改表单

---

### 第六阶段：任务 API（依赖第三阶段） [无单元测试]

#### 6.1 任务后端架构
```
HTTP 请求
    ↓
routes/task_routes.rs     (路由定义)
    ↓
handlers/task_handler.rs  (处理请求)
    ↓
services/task_service.rs  (业务逻辑)
    ↓
db/db_task.rs             (数据库操作)
```

#### 6.2 任务 API 端点
- [完成] **创建任务** - `POST /api/tasks`
- [完成] **获取任务详情** - `GET /api/tasks/{task_id}`
- [完成] **获取任务列表** - `GET /api/tasks`
- [完成] **更新任务** - `PUT /api/tasks/{task_id}`
- [完成] **删除任务** - `DELETE /api/tasks/{task_id}`
- [完成] **更新状态** - `PUT /api/tasks/{task_id}/status`
- [完成] **更新优先级** - `PUT /api/tasks/{task_id}/priority`
- [完成] **获取任务日志** - `GET /api/tasks/{task_id}/logs`

---

### 第七阶段：任务前端（依赖第六阶段） [无单元测试]

#### 7.1 任务列表
- [完成] **任务列表页面** - `frontend/src/pages/tasks.rs`
- [完成] **任务卡片组件** - `frontend/src/components/task_card.rs`
- [完成] **任务筛选器** - 状态、优先级、截止时间
- [完成] **任务搜索** - 关键词搜索
- [完成] **分页组件** - 分页加载

#### 7.2 任务创建/编辑
- [ ] **任务表单** - `frontend/src/components/task_form.rs`
- [ ] **关键词标签输入** - 标签组件
- [ ] **截止时间选择器** - 日期选择器
- [ ] **优先级选择器** - 优先级组件

#### 7.3 任务详情
- [ ] **任务详情页面** - `frontend/src/pages/task_detail.rs`
- [ ] **任务状态切换** - 状态按钮
- [ ] **任务进度显示** - 进度条
- [ ] **任务历史记录** - 时间线组件

---

### 第八阶段：团队 API（依赖第三阶段） [无单元测试]

#### 8.1 团队后端架构
```
HTTP 请求
    ↓
routes/team_routes.rs     (路由定义)
    ↓
handlers/team_handler.rs  (处理请求)
    ↓
services/team_service.rs  (业务逻辑)
    ↓
db/db_team.rs             (数据库操作)
```

#### 8.2 团队 API 端点
- [ ] **创建团队** - `POST /api/teams`
- [ ] **获取团队详情** - `GET /api/teams/{team_id}`
- [ ] **获取团队列表** - `GET /api/teams`
- [ ] **更新团队** - `PUT /api/teams/{team_id}`
- [ ] **删除团队** - `DELETE /api/teams/{team_id}`
- [ ] **添加成员** - `POST /api/teams/{team_id}/members`
- [ ] **移除成员** - `DELETE /api/teams/{team_id}/members/{user_id}`
- [ ] **更新成员角色** - `PUT /api/teams/{team_id}/members/{user_id}/role`
- [ ] **获取成员列表** - `GET /api/teams/{team_id}/members`
- [ ] **创建邀请** - `POST /api/teams/{team_id}/invites`
- [ ] **申请加入** - `POST /api/teams/{team_id}/join-requests`
- [ ] **处理申请** - `PUT /api/teams/{team_id}/join-requests/{request_id}`
- [ ] **获取团队日志** - `GET /api/teams/{team_id}/logs`

---

### 第九阶段：团队前端（依赖第八阶段） [无单元测试]

#### 9.1 团队列表
- [ ] **团队列表页面** - `frontend/src/pages/teams.rs`
- [ ] **团队卡片组件** - `frontend/src/components/team_card.rs`
- [ ] **创建团队按钮** - 模态框

#### 9.2 团队详情
- [ ] **团队详情页面** - `frontend/src/pages/team_detail.rs`
- [ ] **团队信息展示**
- [ ] **成员列表** - 成员卡片
- [ ] **成员管理** - 添加/移除/角色变更

#### 9.3 团队任务
- [ ] **团队任务列表** - 团队任务卡片
- [ ] **任务分配** - 分配给成员
- [ ] **任务进度** - 团队任务统计

---

### 第十阶段：Dashboard（依赖第七、九阶段） [无单元测试]

#### 10.1 Dashboard API
- [ ] **概览数据** - `GET /api/dashboard/overview`
- [ ] **个人任务统计** - `GET /api/dashboard/tasks`
- [ ] **团队任务统计** - `GET /api/dashboard/teams`

#### 10.2 Dashboard 前端
- [ ] **Dashboard 页面** - `frontend/src/pages/dashboard.rs`
- [ ] **个人任务看板** - 统计卡片
- [ ] **团队任务看板** - 统计图表
- [ ] **最近任务** - 任务列表
- [ ] **数据可视化** - 图表组件

---

### 第十一阶段：子任务（依赖第六阶段） [无单元测试]

#### 11.1 子任务 API
- [ ] **创建子任务** - `POST /api/tasks/{task_id}/subtasks`
- [ ] **获取子任务列表** - `GET /api/tasks/{task_id}/subtasks`
- [ ] **更新子任务** - `PUT /api/tasks/{task_id}/subtasks/{subtask_id}`
- [ ] **删除子任务** - `DELETE /api/tasks/{task_id}/subtasks/{subtask_id}`

#### 11.2 子任务前端
- [ ] **子任务列表组件**
- [ ] **子任务创建/编辑**
- [ ] **子任务状态管理**
- [ ] **子任务与父任务联动**

---

### 第十二阶段：子团队（依赖第八阶段） [无单元测试]

#### 12.1 子团队 API
- [ ] **创建子团队** - `POST /api/teams/{team_id}/subteams`
- [ ] **获取子团队列表** - `GET /api/teams/{team_id}/subteams`
- [ ] **获取子团队详情** - `GET /api/subteams/{sub_team_id}`
- [ ] **更新子团队** - `PUT /api/subteams/{sub_team_id}`
- [ ] **删除子团队** - `DELETE /api/subteams/{sub_team_id}`

#### 12.2 子团队前端
- [ ] **子团队列表**
- [ ] **子团队创建**
- [ ] **子团队成员管理**

---

### 第十三阶段：实时通信（可选，后期优化） [无单元测试]

#### 13.1 WebSocket 后端
- [ ] **WebSocket 服务器**
- [ ] **连接管理**
- [ ] **消息推送**
- [ ] **心跳检测**

#### 13.2 WebSocket 前端
- [ ] **WebSocket 客户端**
- [ ] **连接管理**
- [ ] **消息接收**
- [ ] **实时通知**

---

### 第十四阶段：离线存储（可选，后期优化） [无单元测试]

#### 14.1 离线存储前端 [个人离线模式]
离线模式下，只显示用户的个人任务，与在线模式中的任务数据完全隔离，不干扰团队数据。
- [ ] **IndexedDB 封装** - 创建本地数据库
- [ ] **本地数据存储** - 存储个人任务数据
- [ ] **离线任务操作** - 创建/编辑/删除个人任务
- [ ] **离线状态显示** - 显示当前是否为离线状态
- [ ] **上线同步（可选）** - 联网后同步到服务器

---

### 第十五阶段：测试和优化

#### 15.1 测试
- [ ] **单元测试**
- [ ] **集成测试**
- [ ] **端到端测试**
- [ ] **性能测试**

#### 15.2 优化
- [ ] **性能优化**
- [ ] **代码优化**
- [ ] **安全优化**
- [ ] **用户体验优化**

---

## 📊 优先级总结

### 🔴 高优先级（必须完成）
1. 第一阶段：基础设施
2. 第二阶段：认证和授权
3. 第三阶段：用户 API
4. 第四阶段：前端基础
5. 第五阶段：前端用户界面
6. 第六阶段：任务 API
7. 第七阶段：任务前端

### 🟡 中优先级（建议完成）
8. 第八阶段：团队 API
9. 第九阶段：团队前端
10. 第十阶段：Dashboard
11. 第十一阶段：子任务
12. 第十二阶段：子团队

### 🟢 低优先级（可选优化）
13. 第十三阶段：实时通信
14. 第十四阶段：离线存储
15. 第十五阶段：测试和优化

---

## 💡 开发建议

1. **按顺序开发**：严格按照上述顺序，不要跳过依赖关系
2. **迭代测试**：每个阶段完成后都要进行测试
3. **文档同步**：开发过程中及时更新文档
4. **代码规范**：遵循 Rust 和 Leptos 的最佳实践
5. **版本控制**：每个阶段完成后提交代码

---

## 📝 进度跟踪

### 当前进度
- [x] 第一阶段：基础设施 (10/10)
- [x] 第二阶段：认证和授权 (4/4)
- [x] 第三阶段：用户 API (8/8)
- [x] 第四阶段：前端基础 (0/16)
- [x] 第五阶段：前端用户界面 (0/7)
- [ ] 第六阶段：任务 API (0/8)
- [ ] 第七阶段：任务前端 (0/13)
- [ ] 第八阶段：团队 API (0/14)
- [ ] 第九阶段：团队前端 (0/9)
- [ ] 第十阶段：Dashboard (0/8)
- [ ] 第十一阶段：子任务 (0/8)
- [ ] 第十二阶段：子团队 (0/8)
- [ ] 第十三阶段：实时通信 (0/8)
- [ ] 第十四阶段：离线存储 (0/4)
- [ ] 第十五阶段：测试和优化 (0/8)

### 总体进度
- **总任务数**: 123
- **已完成**: 22
- **进行中**: 0
- **待完成**: 101
- **完成率**: 18%

---

## 🔗 相关文档

- [项目计划文档](./PROJECT_PLAN.md)
- [README](./readme.md)
- [数据库设计](./backend/migrations/001_initial_schema.sql)

---

**文档版本**: 1.0  
**创建日期**: 2026-03-23  
**最后更新**: 2026-03-23