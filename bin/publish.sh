#!/bin/bash

PROJECT="tools-manager"
VERSION="0.0.0"

yarn tauri build && \
./src-tauri/target/release/bundle/nsis/${PROJECT}_${VERSION}_x64-setup.exe
