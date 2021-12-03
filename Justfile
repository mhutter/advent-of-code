set dotenv-load := false

# Run tests for all years
test-all: test-common
	#!/usr/bin/env bash
	set -e -u -o pipefail
	ls -1d 20* | \
	while read -r dir; do
		pushd $dir
		just test
		popd
	done

# Run tests for `common`
test-common:
	cd common && just test

# Remove all `target` directories
clean:
	rm -rf target */target
