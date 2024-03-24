#!/bin/bash

wasm-pack build --target web && miniserve . --index test.html -p 3000
