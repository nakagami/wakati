extern crate duckdb;
extern crate duckdb_loadable_macros;
extern crate libduckdb_sys;

use duckdb::{
    core::{DataChunkHandle, Inserter, LogicalTypeId},
    types::DuckString,
    vscalar::{ScalarFunctionSignature, VScalar},
    vtab::arrow::WritableVector,
    Connection, Result,
};
use duckdb_loadable_macros::duckdb_entrypoint_c_api;
use libduckdb_sys as ffi;
use libduckdb_sys::duckdb_string_t;
use std::error::Error;

use awabi::tokenizer;
use std::env;

#[derive(Clone)]
struct WakatiState {
    tokenizer: tokenizer::Tokenizer,
}
impl Default for WakatiState {
    fn default() -> Self {
        Self {
            tokenizer: tokenizer::Tokenizer::new(None).unwrap(),
        }
    }
}

struct WakatiScalar {}

impl VScalar for WakatiScalar {
    type State = WakatiState;

    unsafe fn invoke(
        state: &Self::State,
        input: &mut DataChunkHandle,
        output: &mut dyn WritableVector,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let values = input.flat_vector(0);
        let values = values.as_slice_with_len::<duckdb_string_t>(input.len());
        let strings = values
            .iter()
            .map(|ptr| DuckString::new(&mut { *ptr }).as_str().to_string())
            .take(input.len());
        let output = output.flat_vector();

        for (i, s) in strings.enumerate() {
            state.tokenizer.tokenize(&s);
            output.insert(i, s.as_str());
        }
        Ok(())
    }

    fn signatures() -> Vec<ScalarFunctionSignature> {
        vec![ScalarFunctionSignature::exact(
            vec![LogicalTypeId::Varchar.into()],
            LogicalTypeId::Varchar.into(),
        )]
    }
}

const EXTENSION_NAME: &str = env!("CARGO_PKG_NAME");

#[duckdb_entrypoint_c_api()]
pub unsafe fn extension_entrypoint(con: Connection) -> Result<(), Box<dyn Error>> {
    let tokenizer = if let Ok(mecabrc) = env::var("MECABRC") {
        tokenizer::Tokenizer::new(Some(&mecabrc))
    } else {
        tokenizer::Tokenizer::new(None)
    }
    .expect("Can't find dictionary");
    con.register_scalar_function_with_state::<WakatiScalar>(
        EXTENSION_NAME,
        &WakatiState { tokenizer },
    )
    .expect("Failed to register wakati function");
    Ok(())
}
