use crate::inferray::{NodeDictionary, TripleStore};

use rayon::prelude::*;

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
//  * Since same-as will be added for s-o symetrically, eq-rep-o is implide by
//  * eq-rep-s
//  *
//  * @author Julien Subercaze
//  *
//  *         Dec. 13
//  */
#[cfg_attr(debug_assertions, flamer::flame)]
fn apply_same_as_rule(ts: &TripleStore) -> Box<dyn Iterator<Item = [u64; 3]> + Sync + Send> {
    let mut output = vec![];
    let pairs1 = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owlsameAs as u64,
    ));
    if pairs1 == None {
        return Box::new(output.into_iter());
    }
    let pairs1 = pairs1.unwrap().so();
    if pairs1.is_empty() {
        Box::new(output.into_iter())
    } else {
        for pair1 in pairs1 {
            output.push([pair1[1], NodeDictionary::owlsameAs as u64, pair1[0]]);
            if pair1[0] < NodeDictionary::START_INDEX as u64 {
                if let Some(pairs2) = ts.elem().get(NodeDictionary::prop_idx_to_idx(pair1[0])) {
                    for pair2 in pairs2.so() {
                        output.push([pair2[0], pair1[1], pair2[1]]);
                    }
                }
            } else {
                for (idx, chunk) in ts.elem().iter().enumerate() {
                    let pairs = chunk.so();
                    let pi = NodeDictionary::idx_to_prop_idx(idx);
                    if pi == NodeDictionary::owlsameAs as u64 {
                        continue;
                    }
                    if !pairs.is_empty() {
                        if pairs[0][0] <= pair1[0] && pairs[pairs.len() - 1][0] >= pair1[0] {
                            for pair in pairs.iter() {
                                if pair[0] > pair1[0] {
                                    break;
                                }
                                if pair[0] == pair1[0] {
                                    output.push([pair1[1], pi, pair[1]]);
                                }
                            }
                        }
                    }
                    let pairs = chunk.os();
                    if !pairs.is_empty() {
                        if pairs[0][0] <= pair1[0] && pairs[pairs.len() - 1][0] >= pair1[0] {
                            for pair in pairs.iter() {
                                if pair[0] > pair1[0] {
                                    break;
                                }
                                if pair[0] == pair1[0] {
                                    output.push([pair[1], pi, pair1[1]]);
                                }
                            }
                        }
                    }
                }
            }
        }
        Box::new(output.into_iter())
    }
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn SAME_AS(ts: &TripleStore) -> Box<dyn Iterator<Item = [u64; 3]> + Sync + Send> {
    apply_same_as_rule(ts)
}
