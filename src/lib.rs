/*!
The Uiua programming language

This currently exists as a library only to reserve the name on crates.io.
The current API should be considered deeply unstable.
*/

#![allow(clippy::single_match, clippy::needless_range_loop)]

mod algorithm;
pub mod array;
pub mod ast;
mod check;
mod compile;
mod cowslice;
mod error;
pub mod format;
pub mod function;
mod grid_fmt;
pub mod lex;
pub mod lsp;
pub mod parse;
pub mod primitive;
#[doc(hidden)]
pub mod profile;
pub mod run;
mod sys;
mod sys_native;
pub mod value;

use std::sync::Arc;

pub use {error::*, run::Uiua, sys::*, sys_native::*};

pub type Ident = Arc<str>;

#[test]
fn suite() {
    for entry in std::fs::read_dir("tests").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|s| s == "ua") {
            let mut env = Uiua::with_native_sys();
            if let Err(e) = env.load_file(&path) {
                panic!("Test failed in {}:\n{}", path.display(), e.show(true));
            } else if let Some(diag) = env.take_diagnostics().into_iter().next() {
                panic!("Test failed in {}:\n{}", path.display(), diag.show(true));
            }
        }
    }
}

#[test]
fn no_dbgs() {
    fn recurse_dirs(dir: &std::path::Path, f: &impl Fn(&std::path::Path)) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.to_string_lossy().contains("target") {
                continue;
            }
            if path.is_dir() {
                recurse_dirs(&path, f);
            } else {
                f(&path);
            }
        }
    }
    recurse_dirs(std::path::Path::new("."), &|path| {
        if path.extension().is_some_and(|ext| ext == "rs") {
            if path.canonicalize().unwrap() == std::path::Path::new(file!()).canonicalize().unwrap()
            {
                return;
            }
            let contents = std::fs::read_to_string(path).unwrap();
            if contents.contains("dbg!") {
                panic!("File {} contains a dbg! macro", path.display());
            }
        }
    });
}
