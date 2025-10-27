#!/usr/bin/env bash
set -e

# ==========================
# uniffi-bindgen generate shared/src/shared.udl --language kotlin --out-dir build/android/kotlin
# 配置部分
# ==========================

UID_PATH="shared/src/shared.udl"
OUT_DIR="build/android/kotlin"

# ==========================

echo "开始 Android Kotlin 构建: ${UID_PATH})"

uniffi-bindgen generate "$UID_PATH" --language kotlin --out-dir "$OUT_DIR"

echo "Android Kotlin 构建完成，输出目录: $OUT_DIR"
