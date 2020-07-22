use crate::inferray::*;

use rayon::prelude::*;

/// Type aliases to unify all the rules of the reasoner
pub type Rule = fn(&TripleStore) -> RuleResult;
/// Type aliases for the result of a rule (actually a vector)
pub type RuleResult = Vec<[u64; 3]>;

/// A set of Rule, which can be applied on a InfGraph
pub trait RuleSet {
    /// Process this ruleset, possibly using multiple threads
    fn process(&mut self, graph: &mut InfGraph);
    fn is_empty(&self) -> bool;
}

impl RuleSet for Vec<Box<Rule>> {
    fn process(&mut self, graph: &mut InfGraph) {
        if self.is_empty() {
            return;
        }
        let mut outputs = TripleStore::default();
        let ts = graph.dict().ts();
        outputs.add_all(self.par_iter().map(|rule| rule(ts)).collect());
        outputs.sort();
        let ts = TripleStore::join(graph.dict().ts(), &outputs);
        graph.dict_mut().set_ts(ts);
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

pub struct StaticRuleSet {
    pub rules: Box<dyn RuleSet>,
}

impl RuleSet for StaticRuleSet {
    fn process(&mut self, graph: &mut InfGraph) {
        self.rules.process(graph)
    }

    fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}

/// A specific ruleset (run rules until fixpoint is reached)
pub struct FixPointRuleSet {
    pub rules: StaticRuleSet,
}

impl FixPointRuleSet {
    fn fixpoint<F: FnMut(&mut StaticRuleSet, &mut InfGraph)>(
        &mut self,
        graph: &mut InfGraph,
        mut process: F,
    ) {
        if self.rules.is_empty() {
            return;
        }
        let mut size = graph.size();
        let mut prev_size = size + 1;
        while prev_size != size {
            prev_size = size;
            process(&mut self.rules, graph);
            size = graph.size();
        }
    }
}

impl RuleSet for FixPointRuleSet {
    fn process(&mut self, graph: &mut InfGraph) {
        self.fixpoint(graph, <StaticRuleSet as RuleSet>::process)
    }

    fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
