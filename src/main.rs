#![feature(nll)]

extern crate luax;
use std::io::Read;
use std::fs::File;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use luax::{ast, codegen, vm, runtime};
use vm::executor::ExecutorImpl;
use vm::errors::VMError;

fn gen_and_run(ast: ast::Block) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        let module = codegen::ModuleBuilder::new();
        let fn_builder = codegen::FunctionBuilder::new(&module);
        let fn_id = fn_builder.build(&ast, Vec::new()).unwrap();

        let mut executor = ExecutorImpl::new();
        runtime::invoke(&mut executor, module, fn_id);
    }));
    if let Err(e) = result {
        match e.downcast::<VMError>() {
            Ok(e) => panic!("VMError: {}", e.unwrap().to_string()),
            Err(v) => resume_unwind(v)
        }
    }
}

fn main() {
    let ast_path = std::env::args().nth(1).expect("Expecting AST path");
    let mut ast_file = File::open(ast_path).expect("Cannot open AST file");
    let mut code = String::new();

    ast_file.read_to_string(&mut code).expect("Unable to read file");

    let blk = ast::Block::from_json(code).unwrap();
    gen_and_run(blk);
}
