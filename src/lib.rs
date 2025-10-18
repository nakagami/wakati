extern crate duckdb;
extern crate duckdb_loadable_macros;
extern crate libduckdb_sys;

use duckdb::{
    core::{DataChunkHandle, Inserter, LogicalTypeHandle, LogicalTypeId},
    types::DuckString,
    vscalar::{ScalarFunctionSignature, VScalar},
    vtab::arrow::WritableVector,
    vtab::{BindInfo, InitInfo, TableFunctionInfo, VTab},
    Connection, Result,
};
use duckdb_loadable_macros::duckdb_entrypoint_c_api;
use libduckdb_sys as ffi;
use libduckdb_sys::duckdb_string_t;
use std::{
    error::Error,
    ffi::CString,
    sync::atomic::{AtomicBool, Ordering},
};

use awabi::tokenizer;

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
    //    let tokenizer = tokenizer::Tokenizer::new(None).expect("Can't find dictionary");
    con.register_scalar_function::<WakatiScalar>(EXTENSION_NAME)
        .expect("Failed to register wakati function");
    Ok(())
}
