use crate::inferray::*;
use crate::rules::*;

/// A type alias  to unify all the rules of the reasoner
pub type Rule = fn(&mut InfGraph) -> TripleStore;
// pub trait Rule {
//     // fn specialize(&mut self, graph: std::rc::Rc<&'static InfGraph>);
//     fn fire(&mut self, graph: &mut InfGraph) -> TripleStore;
// }

/// A set of Rule, which can be aplly on a InfGraph
pub trait RuleSet {
    // fn new() -> Vec<Box<Rule>>;
    // fn new(&mut self, rules: Vec<Box<Rule>>);
    fn process(&mut self, graph: &mut InfGraph);
}

impl RuleSet for Vec<Box<Rule>> {
    // fn new(&mut self, rules: Vec<Box<Rule>>) {
    //     self.append(&mut rules);
    // }

    // fn new() -> Vec<Box<Rule>> {
    //     vec![
    //         /// Alpha class
    //         Box::new(CAX_SCO),
    //         Box::new(CAX_EQC1),
    //         Box::new(CAX_EQC2),
    //         Box::new(SCM_DOM1),
    //         Box::new(SCM_DOM2),
    //         Box::new(SCM_RNG1),
    //         Box::new(SCM_RNG2),
    //         /// Beta class
    //         Box::new(SCM_SCO_EQC2),
    //         Box::new(SCM_SPO_EQP2),
    //         Box::new(SCM_EQC1),
    //         Box::new(SCM_EQP1),
    //         /// Delta class
    //         Box::new(PRP_INV_1_2),
    //         Box::new(PRP_EQP_1_2),
    //         /// Gamma class
    //         Box::new(PRP_DOM),
    //         Box::new(PRP_RNG),
    //         Box::new(PRP_SPO1),
    //         Box::new(PRP_SYMP),
    //         Box::new(EQ_TRANS),
    //         /// Same as class
    //         Box::new(SAME_AS),
    //         /// Zeta class (trivial rules)
    //         Box::new(RDFS4),
    //         Box::new(RDFS6),
    //         Box::new(RDFS8),
    //         Box::new(RDFS10),
    //         Box::new(RDFS12),
    //         Box::new(RDFS13),
    //         Box::new(SCM_DP_OP),
    //         Box::new(SCM_CLS),
    //     ]
    // }

    fn process(&mut self, graph: &mut InfGraph) {
        let mut outputs = TripleStore::new();
        for rule in self.iter_mut() {
            outputs.add_all(rule(graph));
        }
        graph.dictionary.ts.add_all(outputs);
        graph.dictionary.ts.sort();
    }
}

pub struct StaticRuleSet {
    rules: Box<dyn RuleSet>,
}

impl StaticRuleSet {
    pub fn process(&mut self, graph: &mut InfGraph) {
        self.rules.process(graph);
    }
}

pub struct FixPointRuleSet {
    rules: StaticRuleSet,
}

impl FixPointRuleSet {
    pub fn process(&mut self, graph: &mut InfGraph) {
        let mut size = graph.size();
        let mut prev_size = size + 1;
        while prev_size != size {
            prev_size = size;
            self.rules.process(graph);
            size = graph.size();
        }
    }
}

pub struct ClosureProfile {
    pub on_sa: bool,
    pub on_sco: bool,
    pub on_spo: bool,
    pub on_trp: bool,
}

pub struct RuleProfile {
    pub cl_profile: ClosureProfile,
    pub axiomatic_triples: bool,
    pub before_rules: StaticRuleSet,
    pub rules: FixPointRuleSet,
    pub after_rules: Option<Rule>,
}

impl RuleProfile {
    pub fn RDFS() -> Self {
        let rules: Vec<Box<Rule>> = vec![
            /// Alpha class
            Box::new(CAX_SCO),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
            /// Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
        ];
        let before_rules: Vec<Box<Rule>> = vec![
            /// Zeta class (trivial rules)
            Box::new(RDFS4),
            Box::new(RDFS6),
            Box::new(RDFS8),
            Box::new(RDFS10),
            Box::new(RDFS12),
            Box::new(RDFS13),
        ];
        Self {
            cl_profile: ClosureProfile {
                on_sa: false,
                on_sco: true,
                on_spo: true,
                on_trp: false,
            },
            axiomatic_triples: true,
            before_rules: StaticRuleSet {
                rules: Box::new(before_rules),
            },
            rules: FixPointRuleSet {
                rules: StaticRuleSet {
                    rules: Box::new(rules),
                },
            },
            after_rules: Some(finalize),
        }
    }

    pub fn RDFSDefault() -> Self {
        Self {
            axiomatic_triples: false,
            ..Self::RDFS()
        }
    }
    pub fn RhoDF() -> Self {
        let before_rules: Vec<Box<Rule>> = vec![
            /// Zeta class (trivial rules)
            Box::new(RDFS4),
        ];
        let rules: Vec<Box<Rule>> = vec![
            /// Alpha class
            Box::new(CAX_SCO),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG2),
            /// Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
        ];
        Self {
            cl_profile: ClosureProfile {
                on_sa: false,
                on_sco: true,
                on_spo: true,
                on_trp: false,
            },
            axiomatic_triples: false,
            before_rules: StaticRuleSet {
                rules: Box::new(before_rules),
            },
            rules: FixPointRuleSet {
                rules: StaticRuleSet {
                    rules: Box::new(rules),
                },
            },
            after_rules: None,
        }
    }

    pub fn Closure() -> Self {
        Self {
            cl_profile: ClosureProfile {
                on_sa: true,
                on_sco: true,
                on_spo: true,
                on_trp: true,
            },
            axiomatic_triples: false,
            before_rules: StaticRuleSet {
                rules: Box::new(vec![]),
            },
            rules: FixPointRuleSet {
                rules: StaticRuleSet {
                    rules: Box::new(vec![]),
                },
            },
            after_rules: None,
        }
    }
    pub fn RDFSPlus() -> Self {
        let before_rules: Vec<Box<Rule>> = vec![
            /// Zeta class (trivial rules)
            Box::new(RDFS4),
            Box::new(SCM_DP_OP),
            Box::new(SCM_CLS),
        ];
        let rules: Vec<Box<Rule>> = vec![
            /// Alpha class
            Box::new(CAX_SCO),
            Box::new(CAX_EQC1),
            Box::new(CAX_EQC2),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
            /// Beta class
            Box::new(SCM_SCO_EQC2),
            Box::new(SCM_SPO_EQP2),
            Box::new(SCM_EQC1),
            Box::new(SCM_EQP1),
            /// Delta class
            Box::new(PRP_INV_1_2),
            Box::new(PRP_EQP_1_2),
            /// Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
            Box::new(PRP_SYMP),
            Box::new(EQ_TRANS),
            /// Same as class
            Box::new(SAME_AS),
            /// Other rules
            Box::new(PRP_FP),
            Box::new(PRP_IFP),
        ];
        Self {
            cl_profile: ClosureProfile {
                on_sa: true,
                on_sco: true,
                on_spo: true,
                on_trp: true,
            },
            axiomatic_triples: false,
            before_rules: StaticRuleSet {
                rules: Box::new(before_rules),
            },
            rules: FixPointRuleSet {
                rules: StaticRuleSet {
                    rules: Box::new(rules),
                },
            },
            after_rules: None,
        }
    }
}

pub fn PRP_FP(graph: &mut InfGraph) -> TripleStore {
    let mut output = TripleStore::new();
    let pairs = graph
        .dictionary
        .ts
        .elem
        .get(NodeDictionary::prop_idx_to_idx(
            graph.dictionary.rdftype as u64,
        ));
    if pairs == None {
        return output;
    }
    let pairs = &pairs.unwrap()[1]; // so copy
    let expected_o = graph.dictionary.owlfunctionalProperty as u64;
    for pair in pairs {
        if pair[0] > expected_o {
            break;
        }
        if pair[0] == expected_o {
            let prop = pair[1];
            let raw_prop = NodeDictionary::prop_idx_to_idx(prop);
            let pairs1 = graph.dictionary.ts.elem.get(raw_prop);
            if pairs1 == None {
                break;
            }
            let pairs2 = &pairs1.unwrap()[0];
            if pairs2.is_empty() {
                break;
            }
            let pairs1 = &pairs1.unwrap()[0];
            for pair1 in pairs1 {
                for pair2 in pairs2 {
                    if pair1[0] > pair2[0] {
                        break;
                    }
                    if pair1[0] == pair2[0] {
                        if pair1[1] != pair2[1] {
                            output.add_triple([
                                pair1[1],
                                graph.dictionary.owlsameAs as u64,
                                pair2[1],
                            ])
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn PRP_IFP(graph: &mut InfGraph) -> TripleStore {
    let mut output = TripleStore::new();
    let pairs = graph
        .dictionary
        .ts
        .elem
        .get(NodeDictionary::prop_idx_to_idx(
            graph.dictionary.rdftype as u64,
        ));
    if pairs == None {
        return output;
    }
    let pairs = &pairs.unwrap()[1]; // so copy
    let expected_o = graph.dictionary.owlinverseFunctionalProperty as u64;
    for pair in pairs {
        if pair[0] > expected_o {
            break;
        }
        if pair[0] == expected_o {
            let prop = pair[1];
            let raw_prop = NodeDictionary::prop_idx_to_idx(prop);
            let pairs1 = graph.dictionary.ts.elem.get(raw_prop);
            if pairs1 == None {
                break;
            }
            let pairs2 = &pairs1.unwrap()[1];
            if pairs2.is_empty() {
                break;
            }
            let pairs1 = &pairs1.unwrap()[1];
            for pair1 in pairs1 {
                for pair2 in pairs2 {
                    if pair1[0] > pair2[0] {
                        break;
                    }
                    if pair1[0] == pair2[0] {
                        if pair1[1] != pair2[1] {
                            output.add_triple([
                                pair1[1],
                                graph.dictionary.owlsameAs as u64,
                                pair2[1],
                            ])
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn finalize(graph: &mut InfGraph) -> TripleStore {
    let mut output = TripleStore::new();
    let type_ = graph.dictionary.rdftype as u64;
    let res = graph.dictionary.rdfsResource;
    ((NodeDictionary::START_INDEX as u64 + 1)..=graph.dictionary.get_res_ctr())
        .filter(|e| !graph.dictionary.was_removed(e))
        .for_each(|e| {
            output.add_triple([e, type_, res]);
        });
    output
}
