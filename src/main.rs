extern crate parol_runtime;

mod semantic_triples_grammar;
// The output is version controlled
mod semantic_triples_grammar_trait;
mod semantic_triples_parser;

use crate::semantic_triples_grammar::SemanticTriplesGrammar;
use crate::semantic_triples_parser::parse;
use anyhow::{anyhow, Context, Result};
use parol_runtime::{log::debug, Report};
use std::{env, fs, time::Instant};

// To generate:
// parol -f ./semantic_triples.par -e ./semantic_triples-exp.par -p ./src/semantic_triples_parser.rs -a ./src/semantic_triples_grammar_trait.rs -t SemanticTriplesGrammar -m semantic_triples_grammar -g

struct ErrorReporter;
impl Report for ErrorReporter {}

fn main() -> Result<()> {
    env_logger::init();
    debug!("env logger started");

    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        let file_name = args[1].clone();
        let input = fs::read_to_string(file_name.clone())
            .with_context(|| format!("Can't read file {}", file_name))?;
        let mut semantic_triples_grammar = SemanticTriplesGrammar::new();
        let now = Instant::now();
        match parse(&input, &file_name, &mut semantic_triples_grammar) {
            Ok(_) => {
                let elapsed_time = now.elapsed();
                println!("Parsing took {} milliseconds.", elapsed_time.as_millis());
                if args.len() > 2 && args[2] == "-q" {
                    Ok(())
                } else {
                    println!("Success!\n{}", semantic_triples_grammar);
                    Ok(())
                }
            }
            Err(e) => ErrorReporter::report_error(&e, file_name),
        }
    } else {
        Err(anyhow!("Please provide a file name as first parameter!"))
    }
}
