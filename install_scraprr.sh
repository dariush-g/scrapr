#!/bin/bash

set -e

VERSION="0.1.12"
WHEEL="scraprr-${VERSION}-cp313-cp313-macosx_11_0_arm64.whl"
URL="https://github.com/dariush-g/scraprr/releases/download/v${VERSION}/${WHEEL}"

echo "⬇️ Downloading wheel from GitHub Releases..."
curl -LO "$URL"

echo "📦 Installing scraprr..."
pip install "$WHEEL"
