#!/bin/bash
exec docker run --rm -v `pwd`:'/src' -it newretail.alpha.elenet.me:5000/rust-wasm $@
