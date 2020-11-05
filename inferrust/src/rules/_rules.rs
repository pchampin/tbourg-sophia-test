use crate::inferray::*;

use rayon::prelude::*;

/// Type aliases to unify all the rules of the reasoner
pub(crate) type Rule = fn(&TripleStore) -> RuleResult;
/// Type aliases for the result of a rule (actually a vector)
pub(crate) type RuleResult = Vec<[u64; 3]>;

/// A set of Rule, which can be applied on a InfGraph
pub(crate) trait RuleSet {
    /// Process this ruleset, possibly using multiple threads
    fn process(&self, graph: &mut InfGraph);
    fn is_empty(&self) -> bool;
}

impl RuleSet for Vec<Box<Rule>> {
    fn process(&self, graph: &mut InfGraph) {
        if self.is_empty() {
            return;
        }
        let ts = graph.store();
        let results = self.par_iter().map(|rule| rule(ts)).collect::<Vec<_>>();
        let merged = results.iter().flat_map(|a| a.iter());
        graph.merge_store(TripleStore::new(merged));
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// A specific ruleset (run rules until fixpoint is reached)
pub(crate) struct FixPointRuleSet {
    pub rules: Vec<Box<Rule>>,
}

impl RuleSet for FixPointRuleSet {
    fn process(&self, graph: &mut InfGraph) {
        if self.rules.is_empty() {
            return;
        }
        let mut size = graph.size();
        let mut prev_size = size + 1;
        while prev_size != size {
            prev_size = size;
            self.rules.process(graph);
            size = graph.size();
        }
    }

    fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
