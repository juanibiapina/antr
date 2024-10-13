#!/usr/bin/env bats

setup() {
  load test_helper
  common_setup
}

@test "antr: --help" {
  run antr --help

  assert_success
  assert_output "A simple to use and high performance file watcher

Usage: antr <COMMAND> [ARGS]...

Arguments:
  <COMMAND>  Command to run
  [ARGS]...  Command with args

Options:
  -h, --help     Print help
  -V, --version  Print version"
}
