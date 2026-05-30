#!/usr/bin/env bash
# Скрипт сборки и упаковки дистрибутива компилятора для защиты проекта

set -euo pipefail

echo "🚀 [1/3] Компиляция релизной версии компилятора..."
cargo build --release

# Создаем временную директорию для упаковки
DIST_DIR="dist/minicompiler_distro"
rm -rf dist
mkdir -p "$DIST_DIR"

echo "📁 [2/3] Копирование файлов в дистрибутив..."
# Копируем сам бинарник
if [[ -f "target/release/minicompiler.exe" ]]; then
    cp "target/release/minicompiler.exe" "$DIST_DIR/"
else
    cp "target/release/minicompiler" "$DIST_DIR/"
fi

# Копируем основные файлы
cp run.sh "$DIST_DIR/"
cp Cargo.toml "$DIST_DIR/" # Нужен для запуска Fuzz тестов через cargo

# Копируем директории
cp -r examples "$DIST_DIR/examples"
cp -r scripts "$DIST_DIR/scripts"
cp -r tests "$DIST_DIR/tests"

# Копируем нужную документацию
mkdir -p "$DIST_DIR/docs"
cp -r docs/guides "$DIST_DIR/docs/guides"
cp -r docs/spec "$DIST_DIR/docs/spec"

echo "📦 [3/3] Упаковка в ZIP архив..."
# Используем PowerShell для создания ZIP архива на Windows, так как стандартный zip может отсутствовать в Git Bash
powershell.exe -Command "Compress-Archive -Path dist/minicompiler_distro/* -DestinationPath minicompiler_distro.zip -Force"

# Очищаем временные файлы
rm -rf dist

echo "✅ ГОТОВО! Ваш дистрибутив успешно упакован в файл: minicompiler_distro.zip"
echo "Этот архив содержит релизный бинарник, скрипты тестирования, исходники тестов и документацию!"
