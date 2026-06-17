.PHONY: clean clean_all sync_cargo

PROJ_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

EXTENSION_NAME=wakati

# Set to 1 to enable Unstable API (binaries will only work on TARGET_DUCKDB_VERSION, forwards compatibility will be broken)
# Note: currently extension-template-rs requires this, as duckdb-rs relies on unstable C API functionality
USE_UNSTABLE_C_API=1

# Single source of truth for DuckDB version (no "v" prefix)
DUCKDB_VERSION=1.4.5

# Derived variables — do not edit these directly
TARGET_DUCKDB_VERSION=v$(DUCKDB_VERSION)
DUCKDB_TEST_VERSION=$(DUCKDB_VERSION)

all: configure debug

# Include makefiles from DuckDB
include extension-ci-tools/makefiles/c_api_extensions/base.Makefile
include extension-ci-tools/makefiles/c_api_extensions/rust.Makefile

# Sync DUCKDB_VERSION into Cargo.toml
sync_cargo:
	sed -i 's/^\(duckdb = { version = \)"[^"]*"/\1"$(DUCKDB_VERSION)"/' Cargo.toml
	sed -i 's/^\(libduckdb-sys = { version = \)"[^"]*"/\1"$(DUCKDB_VERSION)"/' Cargo.toml

configure: sync_cargo venv platform extension_version

debug: build_extension_library_debug build_extension_with_metadata_debug
release: build_extension_library_release build_extension_with_metadata_release

test: test_debug
test_debug: test_extension_debug
test_release: test_extension_release

clean: clean_build clean_rust
clean_all: clean_configure clean
