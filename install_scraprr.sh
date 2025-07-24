#!/bin/bash

# Exit on error
set -e

# Find the wheel
WHEEL_PATH=$(ls target/wheels/scraprr-0.1.10-*.whl 2>/dev/null | head -n 1)

if [[ -z "$WHEEL_PATH" ]]; then
    echo "❌ scraprr wheel not found in target/wheels/"
    exit 1
fi

echo "📦 Installing $WHEEL_PATH..."
pip install "$WHEEL_PATH"
