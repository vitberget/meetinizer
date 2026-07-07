#!/usr/bin/env bash

echo http://localhost:9001
cd "$(dirname "$0")"
nginx -c "nginx.conf" -p ./
