#!/bin/sh

info "Running pre-build script"
info "This script modifies the json file at GITHUB_EVENT_PATH"


pip install --upgrade pip
pip install PyGithub

python3 meta/update_json.py
