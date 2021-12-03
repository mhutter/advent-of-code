set dotenv-load := false

# Run tests for all years
test-all: test-common
	#!/usr/bin/env bash
	set -e -u -o pipefail
	ls -1d 20* | \
	while read -r dir; do
		pushd $dir
		cargo test --locked
		popd
	done

test-common:
	cd common && cargo test --locked
