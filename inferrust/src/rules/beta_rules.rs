//! Class beta groups the following rules :
//! <ul>
//! <li>SCM-SCO</li>
//! <li>SCM-EQC2</li>
//! <li>SCM-SPO</li>
//! <li>SCM-EQP2</li>
//! </ul>
//!
//! All these rules have the following properties :
//! <ol>
//! <li>same predicate in both parts</li>
//! </ol>

use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;
use crate::rules::*;

fn apply_beta_rule(ts: &TripleStore, rule_p: usize, infer_p: usize) -> RuleResult {
    let mut output = vec![];
    let pairs = ts.chunks().get(rule_p);
    if pairs == None {
        return output;
    }
    let pairs = pairs.unwrap();
    let rule_p = NodeDictionary::offset_to_prop_idx(rule_p);
    let infer_p = NodeDictionary::offset_to_prop_idx(infer_p);
    let pairs1 = pairs.os();
    let pairs2 = pairs.so();
    let mut counter = 0;
    let mut values = [0; 4];
    for pair1 in pairs1 {
        values[0] = pair1[1];
        values[1] = pair1[0];
        for j in counter..pairs2.len() {
            let pair2 = pairs2[j];
            values[2] = pair2[0];
            values[3] = pair2[1];
            if values[1] == values[2] {
                if values[0] == values[3] {
                    output.push([values[0], infer_p, values[1]]);
                    output.push([values[2], infer_p, values[3]]);
                } else {
                    output.push([values[0], rule_p, values[3]]);
                }
            }
            if values[2] > values[1] {
                counter = j;
                break;
            }
        }
    }
    output
}

fn apply_inverse_beta_rule(ts: &TripleStore, rule_p: usize, infer_p: usize) -> RuleResult {
    let mut output = vec![];
    let pairs = ts.chunks().get(rule_p);
    if pairs == None {
        return output;
    }
    let rule_p = NodeDictionary::offset_to_prop_idx(rule_p);
    let infer_p = NodeDictionary::offset_to_prop_idx(infer_p);
    let pairs1 = pairs.unwrap();
    for pair1 in pairs1.so() {
        output.push([pair1[0], infer_p, pair1[1]]);
        output.push([pair1[1], infer_p, pair1[0]]);
        output.push([pair1[0], rule_p, pair1[1]]);
    }
    output
}

pub(crate) fn SCM_SCO_EQC2(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubClassOf as u64);
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::owlequivalentClass as u64);
    apply_beta_rule(ts, id_1, id_2)
}

pub(crate) fn SCM_SPO_EQP2(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubPropertyOf as u64);
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::owlequivalentProperty as u64);
    apply_beta_rule(ts, id_1, id_2)
}

pub(crate) fn SCM_EQC1(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::owlequivalentClass as u64);
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubClassOf as u64);
    apply_inverse_beta_rule(ts, id_1, id_2)
}

pub(crate) fn SCM_EQP1(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::owlequivalentProperty as u64);
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubPropertyOf as u64);
    apply_inverse_beta_rule(ts, id_1, id_2)
}
