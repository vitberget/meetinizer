#!/usr/bin/env bash

cd "$(dirname "$0")"
nginx -c "nginx.conf" -p ./
