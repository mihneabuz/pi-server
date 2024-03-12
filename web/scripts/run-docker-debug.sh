#!/bin/bash

run_docker() {
  local arch="${1:-amd64}"

  case $arch in
    amd64)
      docker run -e DEPLOY_ADDR="0.0.0.0" -e DEPLOY_PORT=80 --rm -it pi-web-native
      ;;

    arm64)
      docker run -e DEPLOY_ADDR="0.0.0.0" -e DEPLOY_PORT=80 --rm -it --platform linux/arm64 pi-web
      ;;
  esac
}


run_docker "$@"
