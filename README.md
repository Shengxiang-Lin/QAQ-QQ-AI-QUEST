<h1 align="center">QAQ: QQ-AI-Quest</h1>

## 项目概述
QAQ 是一个基于 Rust 语言开发的智能聊天机器人系统，旨在通过集成大模型 API 与 QQ 平台对接，提供智能化聊天体验。系统包含安全稳定的用户信息存储服务、标准化的 Restful API 路由设计以及与 LLOneBot 的对接逻辑，实现了 QQ 平台与 AI 模型的无缝交互。

## 技术架构
- **核心语言**：Rust（2024 edition）
- **Web 框架**：actix-web 4.0（异步 Web 服务）
- **数据存储**：SQLite（通过 sqlx 进行数据库交互）
- **网络请求**：reqwest（HTTP 客户端）
- **QQ 对接**：基于 LLOneBot 协议
- **异步运行时**：tokio
- **配置管理**：serde + serde_json（JSON 序列化/反序列化）
- **跨域支持**：actix-cors

## 核心功能
1. **多 AI 模型集成**
   - 支持 Deepseek 模型（`model_url::DEEPSEEK`）
   - 支持 Doubao 模型（`model_url::DOUBAO_VISION`）
   - 可动态切换模型（通过 `/update_model` 接口）

2. **QQ 平台对接**
   - 支持私聊与群聊消息处理（`LLOneBotPrivate`/`LLOneBotGroup`）
   - 消息格式转换（CQ 码解析与生成）
   - 表情、@ 等特殊消息类型支持

3. **数据管理**
   - 聊天记录持久化存储（`context_database.db`）
   - 上下文会话管理（支持配置上下文限制条数）
   - API 调用统计（请求次数与 Token 消耗计数）

4. **配置系统**
   - 动态配置加载与重载（`config.json`）
   - 多配置文件支持（`config_new` 目录）
   - 可配置参数：温度系数、上下文长度、API 密钥等

## 工程结构
```
QAQ-QQ-AI-QUEST/
├── src/
│   ├── main.rs          # 程序入口（HTTP 服务启动）
│   ├── lib.rs           # 核心模块定义
│   ├── config.rs        # 配置管理
│   ├── services.rs      # API 调用与 QQ 消息发送
│   ├── handlers.rs      # HTTP 路由处理
│   ├── llm_api/         # 大模型接口封装
│   ├── ll_one_bot/      # LLOneBot 协议处理
│   ├── db/              # 数据库操作
│   └── pipeline.rs      # 消息处理流水线
├── config.json          # 主配置文件
├── config_new/          # 配置文件模板目录
├── start.bat            # Windows 启动脚本
├── start.sh             # Linux 启动脚本
└── Cargo.toml           # 项目依赖配置
```

## 安装部署
### 前置依赖
- Rust 环境（[配置指南](https://blog.csdn.net/qq_45325459/article/details/138031515)）
- Node.js（含 npm，用于前端运行）
- LLOneBot 搭载的 QQNT（[下载地址](https://github.com/LLOneBot/LLOneBot)）

### 环境配置
1. **API 密钥配置**
   - 申请 Deepseek 密钥：[https://platform.deepseek.com/api_keys](https://platform.deepseek.com/api_keys)
   - 申请 Doubao 密钥：[https://www.volcengine.com/product/ark](https://www.volcengine.com/product/ark)
   - 配置到 `config.json` 或通过前端页面（`localhost:5173`）设置

2. **LLOneBot 配置**
   - 设置消息转发端口为 8080（默认）    
   ![](doc/J9BREAH88A257OBD.png)      
   ![](doc/P[JH8Y$[ZNQQ1F1W8TGAER.png)     
   ![](doc/FC5{N$``HO]R57GUWLX8D8G.png)  
   - 按照文档配置 QQ 机器人账号

### 启动步骤
- **Windows**：运行 `start.bat`（自动创建数据库文件并启动服务）
- **Linux**：运行 `start.sh`

## 接口说明
| 路径 | 方法 | 功能 |
|------|------|------|
| `/` | POST | 接收 LLOneBot 消息并处理 |
| `/config` | GET | 获取当前配置 |
| `/update_config` | POST | 更新配置文件 |
| `/config_new_list` | GET | 获取配置模板列表 |
| `/config_new/{filename}` | GET | 获取指定配置模板 |
| `/update_model` | POST | 切换 AI 模型 |
| `/usage_stats` | GET | 获取 API 使用统计 |

## 配置参数说明
| 参数 | 类型 | 说明 |
|------|------|------|
| `context_limit` | int | 上下文最大条数限制 |
| `deepseek_key` | string | Deepseek API 密钥 |
| `doubao_key` | string | Doubao API 密钥 |
| `default_prompt` | string | 聊天默认提示词 |
| `presence_penalty` | float | 话题转移倾向（-2~2） |
| `temperature` | float | 回复随机性（0~2） |
| `rust_port` | int | 服务运行端口 |

## 配置参数说明

| 参数               | 类型   | 说明                                  |
|--------------------|--------|---------------------------------------|
| `context_limit`    | int    | 上下文最大条数限制                    |
| `deepseek_key`     | string | Deepseek API 密钥                     |
| `doubao_key`       | string | Doubao API 密钥                       |
| `default_prompt`   | string | 聊天默认提示词                        |
| `presence_penalty` | float  | 话题转移倾向（-2~2）                  |
| `temperature`      | float  | 回复随机性（0~2）                     |
| `rust_port`        | int    | 服务运行端口                          |
| `open_face_support`| bool   | 是否支持 OpenFace 功能                |
| `topic_continue_threshold` | float | 话题延续阈值（0~1，越小越易延续当前话题） |
| `topic_guide_threshold` | float | 话题引导阈值（0~1，越大越易引导新话题） |
| `valid_QQid`       | array  | 需要回复的QQ号列表                    |
| `vue_port`         | int    | Vue 开发服务器运行端口                |

## 统计信息

### API 使用统计
系统会记录各模型的调用次数与 Token 消耗，可通过 `/usage_stats` 接口查询实时数据。

