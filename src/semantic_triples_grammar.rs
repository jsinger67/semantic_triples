use crate::semantic_triples_grammar_trait::{SemanticTriples, SemanticTriplesGrammarTrait};
#[allow(unused_imports)]
use parol_runtime::{Result, Token};
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our SemanticTriples grammar
///
#[derive(Debug, Default)]
pub struct SemanticTriplesGrammar<'t> {
    pub semantic_triples: Option<SemanticTriples<'t>>,
}

impl SemanticTriplesGrammar<'_> {
    pub fn new() -> Self {
        SemanticTriplesGrammar::default()
    }
}

impl Display for SemanticTriples<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        writeln!(f, "digraph {{\n  rankdir=LR;")?;
        self.semantic_triples_list.iter().try_for_each(|triple| {
            let t = &triple.semantic_triple;
            writeln!(
                f,
                "  {} -> {} [label=\"{}\"]",
                t.id.id.text(),
                t.id0.id.text(),
                t.predicate.id.id.text()
            )?;
            Ok(())
        })?;
        writeln!(f, "}}")
    }
}

impl Display for SemanticTriplesGrammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.semantic_triples {
            Some(semantic_triples) => writeln!(f, "{}", semantic_triples),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> SemanticTriplesGrammarTrait<'t> for SemanticTriplesGrammar<'t> {
    /// Semantic action for non-terminal 'SemanticTriples'
    fn semantic_triples(&mut self, arg: &SemanticTriples<'t>) -> Result<()> {
        self.semantic_triples = Some(arg.clone());
        Ok(())
    }
}
