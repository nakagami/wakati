.PHONY: clean clean_all sync_cargo

PROJ_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))

EXTENSION_NAME=wakati

# Mark the extension as C_STRUCT_UNSTABLE: DuckDB 2.x requires the _init_c_api_v2 entrypoint.
# Binaries load only in the exact TARGET_DUCKDB_VERSION, not in newer DuckDB releases.
USE_UNSTABLE_C_API=1
CAPI_VERSION=2

# Single source of truth for DuckDB version (no "v" prefix)
DUCKDB_VERSION=2.0.0

ifeq ($(DUCKDB_VERSION),2.0)
	override DUCKDB_VERSION := 2.0.0
endif

# Derived variables — do not edit these directly
TARGET_DUCKDB_VERSION=v$(DUCKDB_VERSION)
DUCKDB_TEST_VERSION=$(DUCKDB_VERSION)

all: configure debug

# Include makefiles from DuckDB
include extension-ci-tools/makefiles/c_api_extensions/base.Makefile
include extension-ci-tools/makefiles/c_api_extensions/rust.Makefile

# Sync DUCKDB_VERSION into Cargo.toml
sync_cargo:

configure: sync_cargo venv platform extension_version

debug: build_extension_library_debug build_extension_with_metadata_debug
release: build_extension_library_release build_extension_with_metadata_release

test: test_debug
test_debug: debug test_extension_debug
test_release: release test_extension_release

clean: clean_build clean_rust
clean_all: clean_configure clean
