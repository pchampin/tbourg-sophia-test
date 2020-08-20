//! Class alpha groups the following rules :
//! <ul>
//! <li>CAX-SCO</li>
//! <li>SCM-DOM1</li>
//! <li>SCM-DOM2</li>
//! <li>SCM-RNG1</li>
//! <li>SCM-RNG2</li>
//! </ul>
//!
//! All these rules have the following properties :
//! <ol>
//! <li>2 fixed predicates in the head triples</li>
//! <li>Equality between first subject second object or first object second
//! subject</li>
//! <li>Inferred triple contains only s,p,o from the head</li>
//! </ol>

use crate::inferray::NodeDictionary;
use crate::inferray::TripleStore;

use crate::rules::*;
use std::cmp::Ordering;

fn apply_alpha_rule(
    ts: &TripleStore,
    id_1: u64,
    id_2: u64,
    id_s: usize,
    id_p: usize,
    id_o: usize,
) -> RuleResult {
    fn same_s_new_o(output: &mut Vec<[u64; 3]>, o: u64, number: usize) {
        let old_size = output.len();
        let p = output[0][1];
        for i in (old_size - number)..old_size {
            output.push([output[i][0], p, o]);
        }
    }
    let mut output = vec![];
    let property_1_pairs = ts.chunks().get(id_1 as usize);
    let property_2_pairs = ts.chunks().get(id_2 as usize);
    if property_1_pairs == None || property_2_pairs == None {
        return output;
    }
    let property_1_pairs = property_1_pairs.unwrap();
    let property_2_pairs = property_2_pairs.unwrap();
    let mut counter = 0;
    let mut previous = 0;
    let mut last_number = 0;
    let mut values: [u64; 6] = [0; 6];
    values[1] = id_1;
    values[4] = id_2;
    let property_1_pairs = property_1_pairs.so();
    let property_2_pairs = property_2_pairs.os();
    for mut i in 0..property_1_pairs.len() {
        let property_1_pair = property_1_pairs[i];
        values[0] = property_1_pair[0];
        values[2] = property_1_pair[1];
        if values[0] == previous && last_number != 0 {
            same_s_new_o(&mut output, values[2], last_number);
        } else {
            last_number = 0;
            let mut broke = false;
            for j in counter..property_2_pairs.len() {
                let property_2_pair = property_2_pairs[j];
                values[3] = property_2_pair[1];
                values[5] = property_2_pair[0];
                match values[5].cmp(&values[0]) {
                    Ordering::Equal => {
                        output.push([
                            values[id_s],
                            NodeDictionary::offset_to_prop_idx(values[id_p] as usize),
                            values[id_o],
                        ]);
                        last_number += 1;
                    }
                    Ordering::Greater => {
                        broke = true;
                        counter = j;
                        break;
                    }
                    Ordering::Less => (),
                }
            }
            if !broke {
                // Reached the end of second list - Check if subjects in
                // first list remains the same. See example in the paper
                if i < property_1_pairs.len() {
                    let property_1_pair = property_1_pairs[i];
                    values[0] = property_1_pair[0];
                    values[2] = property_1_pair[1];
                    i += 1;
                    while values[0] == previous {
                        // Infer
                        same_s_new_o(&mut output, values[2], last_number);
                        if i == property_1_pairs.len() {
                            break;
                        }
                        let property_1_pair = property_1_pairs[i];
                        values[0] = property_1_pair[0];
                        values[2] = property_1_pair[1];
                        i += 1;
                    }
                }
                break;
            }
        }
        previous = values[0];
    }
    output
}

pub fn CAX_SCO(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubClassOf as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdftype as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 4, 2)
}

pub fn CAX_EQC1(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::owlequivalentClass as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdftype as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 4, 2)
}

// CAX-EQC2 is implied cause a = b -> b = a

pub fn SCM_DOM1(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubClassOf as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfsdomain as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 4, 2)
}

pub fn SCM_DOM2(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfsdomain as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubPropertyOf as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 1, 2)
}

pub fn SCM_RNG1(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubClassOf as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfsrange as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 4, 2)
}

pub fn SCM_RNG2(ts: &TripleStore) -> RuleResult {
    let id_1 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfsrange as u64) as u64;
    let id_2 = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdfssubPropertyOf as u64) as u64;
    apply_alpha_rule(ts, id_1, id_2, 3, 1, 2)
}
