:: 检查是否存在 context_database.db 文件，如果不存在则创建
if not exist "context_database.db" (
    echo. > "context_database.db"
)
:: 启动 Rust 后端
start /min cmd /k "cargo run --release --bin QAQ"

:: 启动前端项目
start /min cmd /k "cd ./fore-end && npm install &&  npm run dev"