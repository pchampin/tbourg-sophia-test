use crate::inferray::{NodeDictionary, TripleStore};
use crate::rules::*;

fn apply_zeta_rule(
    ts: &TripleStore,
    input_o: u64,
    output_p: u64,
    output_o: u64,
    object_is_subject: bool,
) -> RuleResult {
    let mut output = vec![];
    let pairs1 = ts.elem().get(NodeDictionary::prop_idx_to_offset(
        NodeDictionary::rdftype as u64,
    ));
    if pairs1 == None {
        return output;
    }
    let pairs1 = pairs1.unwrap().os();
    if pairs1.is_empty() {
        return output;
    }
    for pair1 in &*pairs1 {
        if pair1[0] > input_o {
            break;
        }
        if pair1[0] == input_o {
            if !object_is_subject {
                output.push([pair1[1], output_p, output_o]);
            } else {
                output.push([pair1[1], output_p, pair1[1]]);
            }
        }
    }
    output
}

pub fn RDFS6(ts: &TripleStore) -> RuleResult {
    let input_o = NodeDictionary::rdfProperty as u64;
    let output_p = NodeDictionary::rdfssubPropertyOf as u64;
    apply_zeta_rule(ts, input_o, output_p, 0, true)
}

pub fn RDFS8(ts: &TripleStore) -> RuleResult {
    let input_o = NodeDictionary::rdfsClass;
    let output_p = NodeDictionary::rdftype as u64;
    let output_o = NodeDictionary::rdfsResource;
    apply_zeta_rule(ts, input_o, output_p, output_o, false)
}

pub fn RDFS10(ts: &TripleStore) -> RuleResult {
    let input_o = NodeDictionary::rdfsClass;
    let output_p = NodeDictionary::rdfssubClassOf as u64;
    apply_zeta_rule(ts, input_o, output_p, 0, true)
}

pub fn RDFS12(ts: &TripleStore) -> RuleResult {
    let input_o = NodeDictionary::rdfsContainerMembershipProperty as u64;
    let output_p = NodeDictionary::rdfssubPropertyOf as u64;
    let output_o = NodeDictionary::rdfsMember as u64;
    apply_zeta_rule(ts, input_o, output_p, output_o, false)
}

pub fn RDFS13(ts: &TripleStore) -> RuleResult {
    let input_o = NodeDictionary::rdfsDatatype;
    let output_p = NodeDictionary::rdfssubClassOf as u64;
    let output_o = NodeDictionary::rdfsLiteral;
    apply_zeta_rule(ts, input_o, output_p, output_o, false)
}

pub fn SCM_DP_OP(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let pairs1 = ts.elem().get(NodeDictionary::prop_idx_to_offset(
        NodeDictionary::rdftype as u64,
    ));
    if pairs1 == None {
        return output;
    }
    let pairs1 = pairs1.unwrap().os();
    if pairs1.is_empty() {
        return output;
    }
    for pair1 in pairs1 {
        for object in [
            NodeDictionary::owldataTypeProperty as u64,
            NodeDictionary::owlobjectProperty as u64,
        ]
        .iter()
        {
            if pair1[0] > *object {
                break;
            }
            if pair1[0] == *object {
                output.push([pair1[1], NodeDictionary::rdfssubPropertyOf as u64, pair1[1]]);
                output.push([
                    pair1[1],
                    NodeDictionary::owlequivalentProperty as u64,
                    pair1[1],
                ]);
            }
        }
    }
    output
}

pub fn SCM_CLS(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let pairs1 = ts.elem().get(NodeDictionary::prop_idx_to_offset(
        NodeDictionary::rdftype as u64,
    ));
    if pairs1 == None {
        return output;
    }
    let pairs1 = pairs1.unwrap().os();
    if pairs1.is_empty() {
        return output;
    }
    let object = NodeDictionary::owlclass;
    for pair1 in pairs1 {
        if pair1[0] > object {
            break;
        }
        if pair1[0] == object {
            output.push([pair1[1], NodeDictionary::rdfssubClassOf as u64, pair1[1]]);
            output.push([
                pair1[1],
                NodeDictionary::owlequivalentClass as u64,
                pair1[1],
            ]);
            output.push([
                pair1[1],
                NodeDictionary::rdfssubClassOf as u64,
                NodeDictionary::owlthing as u64,
            ]);
            output.push([
                NodeDictionary::nothing as u64,
                NodeDictionary::rdfssubClassOf as u64,
                pair1[1],
            ]);
        }
    }
    output
}

pub fn RDFS4(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let mut resources_idx = Vec::new();
    let pairs1 = ts.elem().get(NodeDictionary::prop_idx_to_offset(
        NodeDictionary::rdftype as u64,
    ));
    if pairs1 == None {
        return output;
    }
    let pairs1 = pairs1.unwrap().os();
    if pairs1.is_empty() {
        return output;
    }
    let object = NodeDictionary::rdfsResource;
    for pair1 in pairs1 {
        if pair1[0] > object {
            break;
        }
        if pair1[0] == object {
            resources_idx.push(pair1[1])
        }
    }
    for pairs2 in ts.elem() {
        for pair2 in pairs2.so() {
            if resources_idx.contains(&pair2[1]) {
                output.push([pair2[0], NodeDictionary::rdftype as u64, object]);
            }
        }
    }
    output
}
