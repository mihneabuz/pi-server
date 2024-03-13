#!/bin/bash

docker run --privileged --rm tonistiigi/binfmt --install all
docker run --privileged --rm multiarch/qemu-user-static --reset -p yes
