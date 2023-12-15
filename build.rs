use std::process;

use parol::{build::Builder, ParolErrorReporter};
use parol_runtime::Report;

fn main() {
    // CLI equivalent is:
    // parol -f ./semantic_triples.par -e ./semantic_triples-exp.par -p ./src/semantic_triples_parser.rs -a ./src/semantic_triples_grammar_trait.rs -t SemanticTriplesGrammar -m semantic_triples_grammar -g
    if let Err(err) = Builder::with_explicit_output_dir("src")
        .grammar_file("semantic_triples.par")
        .expanded_grammar_output_file("../semantic_triples-exp.par")
        .parser_output_file("semantic_triples_parser.rs")
        .actions_output_file("semantic_triples_grammar_trait.rs")
        .enable_auto_generation()
        .user_type_name("SemanticTriplesGrammar")
        .user_trait_module_name("semantic_triples_grammar")
        .trim_parse_tree()
        .generate_parser()
    {
        ParolErrorReporter::report_error(&err, "semantic_triples.par").unwrap_or_default();
        process::exit(1);
    }
}
