#!/bin/bash

run_docker() {
  local arch="${1:-amd64}"

  case $arch in
    amd64)
      docker run -p 80:3000 --rm -it pi-web-native
      ;;

    arm64)
      docker run -p 80:3000 --rm -it --platform linux/arm64 pi-web
      ;;
  esac
}


run_docker "$@"
