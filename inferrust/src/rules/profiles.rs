//! Provides type `RuleProfile` (other utility types).

use crate::inferray::*;
use crate::rules::*;
use std::fmt;

pub struct ClosureProfile {
    pub on_sa: bool,
    pub on_sco: bool,
    pub on_spo: bool,
    pub on_trp: bool,
}

/// A set of rules used for reasoning.
pub struct RuleProfile {
    pub(crate) cl_profile: ClosureProfile,
    pub(crate) axiomatic_triples: bool,
    pub(crate) before_rules: Vec<Box<Rule>>,
    pub(crate) rules: FixPointRuleSet,
    pub(crate) after_rules: Option<Box<dyn Fn(&InfGraph) -> RuleResult>>,
    name: String,
}

impl RuleProfile {
    /// The standard set of rules for RDF-Schema
    pub fn RDFS() -> Self {
        let rules: Vec<Box<Rule>> = vec![
            // Alpha class
            Box::new(CAX_SCO),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
            // Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
        ];
        let before_rules: Vec<Box<Rule>> = vec![
            // Zeta class (trivial rules)
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
            before_rules: before_rules,
            rules: FixPointRuleSet {
                rules: rules,
            },
            after_rules: Some(Box::new(type_all_resources)),
            name: "RDFS".to_string(),
        }
    }

    /// ρdf is a subset of RDFS, hence faster to compute,
    /// with no significant loss of expressivity.
    pub fn RhoDF() -> Self {
        let before_rules: Vec<Box<Rule>> = vec![
            // Zeta class (trivial rules)
            Box::new(RDFS4),
        ];
        let rules: Vec<Box<Rule>> = vec![
            // Alpha class
            Box::new(CAX_SCO),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG2),
            // Gamma class
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
            before_rules: before_rules,
            rules: FixPointRuleSet {
                rules: rules,
            },
            after_rules: None,
            name: "RHODF".to_string(),
        }
    }

    /// RDFS-Plus extends RDF with some terms from OWL:
    ///
    /// `equivalentClass`, `sameAs`, `equivalentProperty`, `FunctionalProperty`,
    /// `InverseFunctionalProperty`, `inverseOf`, `SymmetricProperty`, `TransitiveProperty`.
    pub fn RDFSPlus() -> Self {
        let before_rules: Vec<Box<Rule>> = vec![
            // Zeta class (trivial rules)
            Box::new(RDFS4),
            Box::new(SCM_DP_OP),
            Box::new(SCM_CLS),
        ];
        let rules: Vec<Box<Rule>> = vec![
            // Alpha class
            Box::new(CAX_SCO),
            Box::new(CAX_EQC1),
            Box::new(SCM_DOM1),
            Box::new(SCM_DOM2),
            Box::new(SCM_RNG1),
            Box::new(SCM_RNG2),
            // Beta class
            Box::new(SCM_SCO_EQC2),
            Box::new(SCM_SPO_EQP2),
            Box::new(SCM_EQC1),
            Box::new(SCM_EQP1),
            // Delta class
            Box::new(PRP_INV_1_2),
            Box::new(PRP_EQP_1_2),
            // Gamma class
            Box::new(PRP_DOM),
            Box::new(PRP_RNG),
            Box::new(PRP_SPO1),
            Box::new(PRP_SYMP),
            Box::new(EQ_TRANS),
            // Same as class
            Box::new(SAME_AS),
            // Other rules
            Box::new(PRP_FP),
            Box::new(PRP_IFP),
            Box::new(PRP_TRP),
        ];
        Self {
            cl_profile: ClosureProfile {
                on_sa: true,
                on_sco: true,
                on_spo: true,
                on_trp: true,
            },
            axiomatic_triples: false,
            before_rules: before_rules,
            rules: FixPointRuleSet {
                rules: rules,
            },
            after_rules: Some(Box::new(type_all_resources)),
            name: "RDFSPLUS".to_string(),
        }
    }

    /// Return the name of this RuleProfile
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Display for RuleProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name)
    }
}

impl fmt::Debug for RuleProfile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
