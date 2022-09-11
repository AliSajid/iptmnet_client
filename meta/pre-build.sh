#!/bin/sh

set -eu

set_output() {
  echo "::set-output name=$1::$2"
}
info() {
  echo "::info::$*"
}
warn() {
  echo "::warning file=entrypoint.sh::$*"
}
error() {
  echo "::error file=entrypoint.sh::$*"
}

info "Running pre-build script"
info "This script modifies the json file at GITHUB_EVENT_PATH"


pip install --upgrade pip
pip install PyGithub

python3 update_json.py
