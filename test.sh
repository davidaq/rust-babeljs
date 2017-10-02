pushd `dirname $0`

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

$1
