use crate::inferray::{NodeDictionary, TripleStore};
use crate::rules::*;

// /**
//  * Same-as special Rule
//  *
//  * Encompasses :
//  * <ul>
//  * <li>eq-rep-o</li>
//  * <li>eq-rep-p</li>
//  * <li>eq-rep-s</li>
//  * <li>eq-sym</li>
//  * </ul>
//  *
//  * @author Julien Subercaze
//  *
//  *         Dec. 13
//  */
fn apply_same_as_rule(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let sameas_chunk = ts.chunks().get(NodeDictionary::prop_idx_to_offset(
        NodeDictionary::owlsameAs as u64,
    ));
    if let Some(sameas_chunk) = sameas_chunk {
        for same in sameas_chunk.so() {
            output.push([same[1], NodeDictionary::owlsameAs as u64, same[0]]);
            if same[0] < NodeDictionary::START_INDEX as u64 {
                // EQ-REP-P
                if let Some(pairs) = ts.chunks().get(NodeDictionary::prop_idx_to_offset(same[0])) {
                    for [si, oi] in pairs.so() {
                        // TODO: ensure that same[1] is a property index
                        output.push([*si, same[1], *oi]);
                    }
                }
            } else {
                for (idx, chunk) in ts.chunks().iter().enumerate() {
                    let pi = NodeDictionary::offset_to_prop_idx(idx);
                    if pi == NodeDictionary::owlsameAs as u64 {
                        continue;
                    }
                    // EQ-REP-S
                    let so_pairs = chunk.so();
                    if !so_pairs.is_empty() {
                        let first_s = so_pairs[0][0];
                        let last_s = so_pairs[so_pairs.len() - 1][0];
                        if first_s <= same[0] && same[0] <= last_s {
                            for [si, oi] in so_pairs {
                                if *si > same[0] {
                                    break;
                                }
                                if *si == same[0] {
                                    output.push([same[1], pi, *oi]);
                                }
                            }
                        }
                    }
                    // EQ-REP-O
                    let os_pairs = chunk.os();
                    if !os_pairs.is_empty() {
                        let first_o = os_pairs[0][0];
                        let last_o = os_pairs[os_pairs.len() - 1][0];
                        if first_o <= same[0] && same[0] <= last_o {
                            for [oi, si] in os_pairs {
                                if *oi > same[0] {
                                    break;
                                }
                                if *oi == same[0] {
                                    output.push([*si, pi, same[1]]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn SAME_AS(ts: &TripleStore) -> RuleResult {
    apply_same_as_rule(ts)
}
