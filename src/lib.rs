use std::env;

use awabi::tokenizer;
use duckdb_loadable_macros::duckdb_entrypoint_c_api_v2;
use duckdb_neo::{
    DuckDBType, Result,
    extension::Extension,
    scalar::{ScalarCallbacks, ScalarFunctionBuilder},
    signature::{FunctionParameter, SignatureBuilder},
};

struct WakatiScalar {
    tokenizer: tokenizer::Tokenizer,
}

impl ScalarCallbacks for WakatiScalar {
    type BindData = ();
    type InitData = ();

    fn exec(
        &self,
        _bind_data: Option<&Self::BindData>,
        _init_data: Option<&mut Self::InitData>,
        _context: &duckdb_neo::connection::Context,
        vectors: &mut duckdb_neo::data_chunk::VectorCollection,
        output: duckdb_neo::vector::Vector<'_, duckdb_neo::vector::Unknown>,
    ) -> Result<()> {
        let mut output = output.cast::<String>()?;
        output.set_size(vectors.row_count()?)?;

        let vector = vectors.get_vector_at::<String>(0)?;
        let reader = vector.reader()?;
        let mut writer = output.writer()?;

        for (i, row) in reader.iter().enumerate() {
            match row {
                Some(row) => {
                    let tokens = self.tokenizer.tokenize(&row);
                    let s2 = tokens
                        .iter()
                        .map(|t| t.0.as_str())
                        .collect::<Vec<&str>>()
                        .join(" ");
                    writer.set(i, Some(&s2))
                }
                None => writer.set(i, None),
            }?;
        }
        Ok(())
    }
}

const EXTENSION_NAME: &str = env!("CARGO_PKG_NAME");

#[duckdb_entrypoint_c_api_v2(extension_name = "wakati", api_version = "v2.0.0")]
pub fn entrypoint(extension: &Extension) -> Result<()> {
    let tokenizer = if let Ok(mecabrc) = env::var("MECABRC") {
        tokenizer::Tokenizer::new(Some(&mecabrc))
    } else {
        tokenizer::Tokenizer::new(None)
    }
    .expect("Can't find dictionary");

    ScalarFunctionBuilder::new(
        EXTENSION_NAME,
        SignatureBuilder::new()
            .param(FunctionParameter::new::<String>("input"))
            .return_type(String::logical_type(extension)?),
        WakatiScalar { tokenizer },
    )
    .register(extension)?;

    Ok(())
}
