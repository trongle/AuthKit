#!/usr/bin/env bash

cargo watch -w ./src/ -w ./resources/ -s "npx tailwindcss -i ./resources/css/app.css -o ./public/css/app.css && cargo run"

