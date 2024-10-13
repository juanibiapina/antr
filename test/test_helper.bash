common_setup() {
  load vendor/bats-support/load
  load vendor/bats-assert/load

  export ANTR_ROOT="${BATS_TEST_DIRNAME}/.."

  export ANTR_TEST_DIR="${BATS_TMPDIR}/antr"

  if [ -z $ANTR_BIN ]; then
    export ANTR_BIN=$ANTR_ROOT/target/debug/antr
  fi

  mkdir -p $ANTR_TEST_DIR
}

teardown() {
  rm -rf "$ANTR_TEST_DIR"
}
