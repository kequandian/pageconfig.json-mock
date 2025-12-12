# Docker 环境设置说明

这个目录包含了为 `json-mock-rust` 项目设置的完整 Docker 开发环境。

## 文件说明

- `Dockerfile` - 多阶段构建的 Rust 应用容器
- `docker-compose.yml` - 完整的服务编排，包含 MongoDB 和 Mongo Express
- `scripts/init-mongo.js` - MongoDB 初始化脚本
- `.dockerignore` - 构建时忽略的文件

## 快速开始

### 1. 启动整个环境

```bash
cd json-mock-rust
docker-compose up -d
```

这将启动：
- **MongoDB** (端口 27017) - 数据库服务
- **Rust 应用** (端口 3000) - JSON Mock 服务
- **Mongo Express** (端口 8081) - Web 界面的数据库管理工具

### 2. 查看服务状态

```bash
docker-compose ps
```

### 3. 查看日志

```bash
# 查看所有服务日志
docker-compose logs

# 查看特定服务日志
docker-compose logs json-mock-rust
docker-compose logs mongodb
```

### 4. 停止服务

```bash
docker-compose down
```

### 5. 完全清理（包括数据卷）

```bash
docker-compose down -v
```

## 访问服务

- **JSON Mock API**: http://localhost:3000
- **健康检查**: http://localhost:3000/health
- **Mongo Express**: http://localhost:8081 (用户名/密码: admin/admin)
- **MongoDB**: mongodb://admin:password@localhost:27017

## 数据库配置

### 连接信息
- **主机**: localhost
- **端口**: 27017
- **用户名**: admin
- **密码**: password
- **数据库**: json_mock

### 初始数据
启动时会自动创建以下集合和示例数据：
- `posts` - 示例文章数据
- `users` - 示例用户数据
- `forms` - 示例表单配置（包含测试表单 ID: 109）
- `pages` - 页面配置集合

## 开发工作流

### 1. 构建应用

```bash
docker-compose build json-mock-rust
```

### 2. 运行测试

```bash
docker-compose exec json-mock-rust cargo test
```

### 3. 进入容器调试

```bash
# 进入应用容器
docker-compose exec json-mock-rust /bin/bash

# 进入 MongoDB 容器
docker-compose exec mongodb mongosh -u admin -p password
```

### 4. 重新加载应用

```bash
docker-compose restart json-mock-rust
```

## 环境变量配置

主要环境变量在 `docker-compose.yml` 中定义：

```yaml
environment:
  MONGODB_URI: mongodb://admin:password@mongodb:27017
  DB_NAME: json_mock
  ENVIRONMENT: development
  RUST_LOG: info,tower_http=debug
```

## 测试 API

使用提供的 `.http` 文件测试 API：

```bash
# 在 VS Code 中使用 REST Client 扩展
# 或者使用 curl

# 健康检查
curl http://localhost:3000/health

# 获取所有表单
curl http://localhost:3000/forms?verbose=true

# 获取特定表单
curl http://localhost:3000/form?id=109
```

## 故障排除

### 1. 如果应用无法连接到 MongoDB

```bash
# 检查 MongoDB 是否健康
docker-compose ps mongodb

# 查看 MongoDB 日志
docker-compose logs mongodb
```

### 2. 如果端口冲突

修改 `docker-compose.yml` 中的端口映射：

```yaml
ports:
  - "3001:3000"  # 将外部端口改为 3001
```

### 3. 如果构建失败

```bash
# 清理构建缓存
docker-compose build --no-cache json-mock-rust
```

### 4. 如果权限问题

```bash
# 重新构建并重新设置权限
docker-compose down
docker-compose up --build
```

## 生产部署

要部署到生产环境，请：

1. 将 `ENVIRONMENT` 设置为 `production`
2. 更改默认密码
3. 移除 `mongo-express` 服务
4. 添加适当的资源限制
5. 配置适当的日志收集

```bash
# 生产环境示例
ENVIRONMENT=production docker-compose up -d json-mock-rust mongodb
```