# 需求分析（最小化终端）

## 需求背景
将 Cute 终端简化为最小化本地终端，移除所有云/远程功能。

## 本服务职责
- 本地终端模拟器
- 本地文件编辑
- 本地 shell 会话

## 需要删除的功能模块

### 已删除的 crate
- crates/cute_graphql
- crates/cute_graphql_schema
- crates/cute_server_client
- crates/managed_secrets
- crates/remote_server

### 已删除的 app 模块目录
- app/src/drive/
- app/src/remote_server/
- app/src/billing/
- app/src/pricing/
- app/src/resource_center/

### 需要清理的代码引用
1. app/src/lib.rs 中的模块声明和引用
2. app/Cargo.toml 中的 features 和依赖
3. 其他模块中对已删除功能的引用

## 需要从 default features 移除的 features
- cloud_mode, cloud_mode_*, cloud_conversations, cloud_environments
- drive_objects_as_context
- handoff_local_cloud, handoff_cloud_cloud
- hoa_remote_control
- remote_codebase_indexing
- api_key_authentication
- mcp_oauth
- cute_managed_secrets

## 风险点
- 大量代码引用需要清理
- features 之间有依赖关系
- 编译验证需要逐步进行
