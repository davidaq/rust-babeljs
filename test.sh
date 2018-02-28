#!/bin/bash

source ./env.sh

TEST_CASE_DIR=`pwd`/test-case
FETCH_TMP_DIR=tmp-test-fetch

fetch () {
  rm -rf $FETCH_TMP_DIR
  mkdir -p $FETCH_TMP_DIR
  pushd $FETCH_TMP_DIR
  curl 'https://codeload.github.com/babel/babylon/zip/master' > ./master.zip
  unzip master.zip
  cd babylon-master/test
  mvtest () {
    mkdir -p $TEST_CASE_DIR/$1
    mv $1/actual.js $TEST_CASE_DIR/$1/actual.js
    mv $1/expected.json $TEST_CASE_ DIR/$1/expected.json
  }
  find . -type f -name expected.json | while read x; do mvtest `dirname $x`; done
  popd
  rm -rf $FETCH_TMP_DIR
}

debug () {
  RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build || exit 1
  ./target/debug/rust-babeljs debug print_tokens
}

run () {
  RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build || exit 1
  runcase () {
    echo [ $1 ]
    ./target/debug/rust-babeljs "$1"
    local status=$?
    if [ $status -ne 0 ]; then
      cat $1
      echo ''
      echo "Failed" >&2
      exit 1
    fi
  }
  find test-case/fixtures/$2/ -name actual.js | while read x; do runcase $x; done
}

run_r () {
  RUSTFLAGS="$RUSTFLAGS -A dead_code" cargo build --release || exit 1
  runcase () {
    echo [ $1 ]
    ./target/release/rust-babeljs "$1" print_tokens
    local status=$?
    if [ $status -ne 0 ]; then
      cat $1
      echo ''
      echo "Failed" >&2
      exit 1
    fi
  }
  find test-case/fixtures/$2/ -name actual.js | while read x; do runcase $x; done
}

$@
