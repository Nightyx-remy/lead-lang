extern crate core;

use crate::generator::Generator;
use crate::lexer::Lexer;
use crate::node::Node;
use crate::optimizer::Optimizer;
use crate::parser::Parser;
use crate::position::Positioned;
use crate::token::Token;
use crate::transpiler::Transpiler;

mod lexer;
mod token;
mod position;
mod node;
mod parser;
mod either;
mod optimizer;
mod cnode;
mod transpiler;
mod generator;

fn main() {
    // File
    let mut str = std::fs::read_to_string("main.lead").expect("Failed to read file 'main.lead'");

    // Lexer
    let mut lexer = Lexer::new(str);
    let lexer_result = lexer.tokenize();
    str = lexer.take();

    match lexer_result {
        Ok(tokens) => {
            for token in tokens.iter() {
                println!("{:?}", token);
            }

            println!("\n\n");

            // Parser
            let mut parser = Parser::new(str, tokens);
            let parser_result = parser.parse();
            let str = parser.take();

            match parser_result {
                Ok(ast) => {
                    for node in ast.iter() {
                        println!("{:?}", node);
                    }

                    println!("\n\n");

                    // Optimizer
                    let mut optimizer = Optimizer::new(str, ast);
                    let optimizer_result = optimizer.optimize(true);
                    let str = optimizer.take();

                    match optimizer_result {
                        Ok(ast) => {
                            for node in ast.iter() {
                                println!("{:?}", node);
                            }

                            println!("\n\n");

                            // Transpiler
                            let mut transpiler = Transpiler::new(str, ast);
                            let transpiler_result = transpiler.transpile();
                            let str = transpiler.take();

                            match transpiler_result {
                                Ok(ast) => {
                                    for node in ast.iter() {
                                        println!("{:?}", node);
                                    }

                                    println!("\n\n");

                                    // Generator
                                    let mut generator = Generator::new(ast);
                                    println!("{}", generator.generate());
                                }
                                Err(err) => {
                                    println!("[Transpiler Error]: {}", err.data);
                                    err.show_on_text(str);
                                }
                            }
                        }
                        Err(err) => {
                            println!("[Optimizer Error]: {}", err.data);
                            err.show_on_text(str);
                        }
                    }
                }
                Err(err) => {
                    println!("[Parser Error]: {}", err.data);
                    err.show_on_text(str);
                }
            }

        }
        Err(err) => {
            println!("[Lexer Error]: {}", err.data);
            err.show_on_text(str.clone());
        }
    }

}
