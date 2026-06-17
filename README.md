# DuckDB extension `wakati` scala function

This is a DuckDB extension that splits Japanese morphemes and connects them with spaces.
It returns the same result as when executing mecab command like bellow.

```
mecab -O wakati
```

Requires Rust for building.

## Cloning

Clone the repo with submodules

```shell
git clone --recurse-submodules git@github.com:nakagami/wakati.git
```

## Dependencies
In principle, these extensions can be compiled with the Rust toolchain alone. However, this template relies on some additional
tooling to make life a little easier and to be able to share CI/CD infrastructure with extension templates for other languages:

- Python3
- Python3-venv
- [Make](https://www.gnu.org/software/make)
- Git

Installing these dependencies will vary per platform:
- For Linux, these come generally pre-installed or are available through the distro-specific package manager.
- For MacOS, [homebrew](https://formulae.brew.sh/).
- For Windows, [chocolatey](https://community.chocolatey.org/).

## Building
After installing the dependencies, building is a two-step process. Firstly run:
```shell
make configure
```
This will ensure a Python venv is set up with DuckDB and DuckDB's test runner installed. Additionally, depending on configuration,
DuckDB will be used to determine the correct platform for which you are compiling.

Then, to build the extension run:
```shell
make debug
```
This delegates the build process to cargo, which will produce a shared library in `target/debug/<shared_lib_name>`. After this step,
a script is run to transform the shared library into a loadable extension by appending a binary footer. The resulting extension is written
to the `build/debug` directory.

To create optimized release binaries, simply run `make release` instead.

### Requirements

You need the Mecab dictionary installation.

See https://github.com/nakagami/awabi?tab=readme-ov-file#requirements-and-how-to-install .

If mecabrc is not in the usual location, specify it via an environment variable.
For example, if you installed the Mecab dictionary using brew, you need to set environment variable as bellow.

```
export MECABRC=/opt/homebrew/etc/mecabrc
```

### Running the extension
To run the extension code, start `duckdb` with `-unsigned` flag. This will allow you to load the local extension file.

```sh
duckdb -unsigned
```

After loading the extension by the file path, you can use the functions provided by the extension (in this case, `wakati()`).

```sql
LOAD './build/debug/extension/wakati/wakati.duckdb_extension';
SELECT wakati(col0) FROM values ('すもももももももものうち'), ('私の名前は中野です') AS v;
```

```
┌────────────────────────────────┐
│          wakati(col0)          │
│            varchar             │
├────────────────────────────────┤
│ すもも も もも も もも の うち │
│ 私 の 名前 は 中野 です        │
└────────────────────────────────┘
```

## Testing
This extension uses the DuckDB Python client for testing. This should be automatically installed in the `make configure` step.
The tests themselves are written in the SQLLogicTest format, just like most of DuckDB's tests. A test can be found in
`test/sql/wakati.test`. To run the tests using the *debug* build:

```shell
make test_debug
```

or for the *release* build:
```shell
make test_release
```

### Version switching
To build and test with a specific DuckDB version, pass `DUCKDB_VERSION` to make:

```shell
make clean_all
make DUCKDB_VERSION=1.5.4
```

This will automatically update `Cargo.toml` and set the correct DuckDB version for building and testing.
The default version is defined in the `Makefile` (`DUCKDB_VERSION`).

### Known issues
This is a bit of a footgun, but the extensions produced by this template may (or may not) be broken on windows on python3.11
with the following error on extension load:
```shell
IO Error: Extension '<name>.duckdb_extension' could not be loaded: The specified module could not be found
```
This was resolved by using python 3.12
