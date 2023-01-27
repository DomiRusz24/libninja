use std::path::Path;
use proc_macro2::TokenStream;
use crate::rust::codegen::ToRustCode;
use crate::rust::format::format_code;
use crate::util;
use crate::util::open;

pub fn write_rust_file_to_path(path: &Path, file: ln_model::File<TokenStream>) -> anyhow::Result<()> {
    let code = file.to_rust_code();
    write_rust_code_to_path(path, code)
}

pub fn write_rust_code_to_path(path: &Path, code: TokenStream) -> anyhow::Result<()> {
    write_rust_to_path(path, code, "")
}

pub fn write_rust_to_path(path: &Path, code: TokenStream, template: &str) -> anyhow::Result<()> {
    let code = format_code(code)?;
    let mut f = open(path)?;
    let mut s = template.to_string();
    if !s.ends_with('\n') {
        s += "\n";
    }
    s += &code;
    util::write_file(path, &s)
}
