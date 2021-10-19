#!/usr/bin/bash
podman build -t tt:latest .
podman run --rm -it tt:latest
