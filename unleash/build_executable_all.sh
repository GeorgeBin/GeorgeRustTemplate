#!/usr/bin/env bash
set -e

# ==========================
# 🌍 配置部分
# ==========================
# 历史脚本名保留不变，实际构建项目正式桌面端 demo
PACKAGE="demo"           # Cargo.toml 中的 package 名称
MODE="release"           # 编译类型: debug / release
TARGETS=(
    # 需要编译的平台列表
    # macOS
    "x86_64-apple-darwin"
    #    "aarch64-apple-darwin"
     # Windows
    "x86_64-pc-windows-gnu"
    #    "x86_64-pc-windows-msvc"
    # Linux
    #    "x86_64-unknown-linux-gnu"
)


# ==========================

echo "🚀 开始跨平台构建: ${PACKAGE} (${MODE} 模式)"
echo "----------------------------------------"

for TARGET in "${TARGETS[@]}"; do
    echo "🏗️  正在构建目标: ${TARGET}"

    # 执行 cargo bundle（macOS）或 cargo build（其他平台）
    if [[ "$TARGET" == *"apple-darwin"* ]]; then
        cargo bundle --package "$PACKAGE" --"$MODE" --target "$TARGET"
        SRC_APP="target/${TARGET}/${MODE}/bundle/osx/${PACKAGE}.app"
        OUT_DIR="build/${TARGET}/${MODE}"
        OUT_APP="${OUT_DIR}/${PACKAGE}.app"

        # 删除目标（如果存在）
        if [ -e "$OUT_APP" ]; then
            echo "删除已存在的目标应用程序..."
            rm -rf "$OUT_APP"
        fi

        mkdir -p "$OUT_DIR"
        cp -r "$SRC_APP" "$OUT_APP"
        echo "✅ macOS 应用输出: $OUT_APP"

    else
        cargo build --package "$PACKAGE" --"$MODE" --target "$TARGET"
        SRC_BIN="target/${TARGET}/${MODE}/${PACKAGE}"
        OUT_DIR="build/${TARGET}/${MODE}"

        mkdir -p "$OUT_DIR"

        # Windows 要加 .exe 后缀
        if [[ "$TARGET" == *"windows"* ]]; then
            cp "$SRC_BIN.exe" "$OUT_DIR/${PACKAGE}.exe"
            echo "✅ Windows 可执行文件输出: $OUT_DIR/${PACKAGE}.exe"
        else
            cp "$SRC_BIN" "$OUT_DIR/${PACKAGE}"
            echo "✅ Linux 可执行文件输出: $OUT_DIR/${PACKAGE}"
        fi
    fi

    echo "----------------------------------------"
done

echo "🎉 全部构建完成，输出目录: build/"
