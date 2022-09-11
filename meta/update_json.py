#! /usr/bin/env python3

import json
import os
import sys
from github import Github


def get_release_data():
    g = Github(os.environ["GITHUB_TOKEN"])
    repo = g.get_repo(os.environ["GITHUB_REPOSITORY"])
    release = repo.get_latest_release()
    return release.raw_data

def get_json_data():
    with open("/tmp/github_event.json", "r") as f:
        return json.load(f)

def update_json_data():
    github_event_data = get_json_data()
    release_data = get_release_data()
    github_event_data["release"] = release_data
    with open("/tmp/github_event.json", "w") as f:
        json.dump(github_event_data, f)

def main():
    update_json_data()


if __name__ == '__main__':
    main()
