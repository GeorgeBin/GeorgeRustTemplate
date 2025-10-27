#!/usr/bin/env bash
set -e

# ==========================
# cargo ndk -t armeabi-v7a -t arm64-v8a -o ../build/android/jniLibs build --release  --package shared
# 配置部分
# ==========================
PACKAGE="shared"           # Cargo.toml 中的 package 名称
MODE="release"           # 编译类型: debug / release
TARGETS=(
    # 需要编译的架构
    "armeabi-v7a"
    "arm64-v8a"
)
OUT_DIR="build/android/jniLibs"

# ==========================

echo "开始 Android.so 构建: ${PACKAGE} (${MODE} 模式)"

cargo ndk -t armeabi-v7a -t arm64-v8a -o "$OUT_DIR" build --"$MODE" --package "$PACKAGE"

echo "Android.so 构建完成，输出目录: $OUT_DIR"
