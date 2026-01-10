#!/usr/bin/env bash
set -e

# ==========================
# uniffi-bindgen generate --library target/aarch64-linux-android/release/librsShared.so --language kotlin --out-dir build/android/kotlin
# 配置部分
# ==========================

OUT_DIR="build/android/kotlin"
CONFIG_PATH="shared/uniffi.toml"
LIB_PATH_CANDIDATES=(
    "target/aarch64-linux-android/release/librsShared.so"
    "target/armv7-linux-androideabi/release/librsShared.so"
    "build/android/jniLibs/arm64-v8a/librsShared.so"
)

# ==========================

LIB_PATH=""
for candidate in "${LIB_PATH_CANDIDATES[@]}"; do
    if [[ -f "$candidate" ]]; then
        LIB_PATH="$candidate"
        break
    fi
done

if [[ -z "$LIB_PATH" ]]; then
    echo "未找到库文件:"
    printf '  %s\n' "${LIB_PATH_CANDIDATES[@]}"
    echo "请先运行: unleash/build_shared_android_so.sh"
    exit 1
fi

echo "开始 Android Kotlin 构建: ${LIB_PATH}"

uniffi-bindgen generate --library "$LIB_PATH" --language kotlin --out-dir "$OUT_DIR" --config "$CONFIG_PATH"

echo "Android Kotlin 构建完成，输出目录: $OUT_DIR"
