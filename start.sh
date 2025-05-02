#!/bin/bash

# 确保脚本工作目录为当前脚本所在目录
cd "$(dirname "$0")"

# 检查是否存在 context_database.db 文件，如果不存在则创建
if [ ! -f "./context_database.db" ]; then
    echo "创建 context_database.db 文件..."
    touch "./context_database.db"
fi

# 启动 Rust 后端
echo "启动 Rust 后端..."
gnome-terminal -- bash -c "cargo run --release --bin QAQ; exec bash"

# 启动前端项目
echo "启动前端项目..."
gnome-terminal -- bash -c "cd ./fore-end && npm install && npm run dev; exec bash"