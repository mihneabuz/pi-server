#!/bin/bash

build_docker() {
  local arch="${1:-amd64}"

  case $arch in
    amd64)
      docker build -f ./scripts/native.dockerfile -t pi-web-native .
      ;;

    arm64)
      docker build -f ./scripts/cross.dockerfile --platform linux/arm64 -t pi-web .
      ;;
  esac
}


build_docker "$@"
