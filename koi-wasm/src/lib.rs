use wasm_bindgen::prelude::*;
use koi_core::lexer::new as new_lexer;
use koi_core::ast;
use koi_core::parser;
use koi_core::interp;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn evaluate(input: &str) -> String {
   let lexer = new_lexer(input.to_owned());

   let mut parser = parser::Parser::new(lexer);
   let prog = parser.parse();

   let mut interpreter = interp::Interpreter::new();
   
   interpreter.do_collect();
   interpreter.run(prog);

   interpreter.collector.take().unwrap()
}