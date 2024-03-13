#!/bin/bash

docker image save pi-web | gzip > /tmp/pi-web.gz
scp /tmp/pi-web.gz pi-server:~/Server/images
