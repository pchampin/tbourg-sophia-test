use crate::inferray::*;
use crate::rules::*;

pub fn PRP_FP(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let pairs_mut = ts.elem().get(NodeDictionary::prop_idx_to_offset(
        NodeDictionary::rdftype as u64,
    ));
    if pairs_mut == None {
        return output;
    }
    let pairs: &Vec<[u64; 2]> = pairs_mut.unwrap().os(); // os copy
    let expected_o = NodeDictionary::owlfunctionalProperty as u64;
    for pair in &*pairs {
        if pair[0] > expected_o {
            break;
        }
        if pair[0] == expected_o {
            let prop = pair[1];
            let raw_prop = NodeDictionary::prop_idx_to_offset(prop);
            let pairs1 = ts.elem().get(raw_prop);
            if pairs1 == None {
                break;
            }
            let pairs2 = pairs1.unwrap().so();
            if pairs2.is_empty() {
                break;
            }
            let pairs1 = pairs1.unwrap().so();
            for pair1 in pairs1 {
                for pair2 in pairs2 {
                    if pair1[0] > pair2[0] {
                        break;
                    }
                    if pair1[0] == pair2[0] {
                        if pair1[1] != pair2[1] {
                            output.push([pair1[1], NodeDictionary::owlsameAs as u64, pair2[1]])
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn PRP_IFP(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let pairs = ts.elem().get(NodeDictionary::prop_idx_to_offset(
        NodeDictionary::rdftype as u64,
    ));
    if pairs == None {
        return output;
    }
    let pairs = pairs.unwrap().os(); // os copy
    let expected_o = NodeDictionary::owlinverseFunctionalProperty as u64;
    for pair in &*pairs {
        if pair[0] > expected_o {
            break;
        }
        if pair[0] == expected_o {
            let prop = pair[1];
            let raw_prop = NodeDictionary::prop_idx_to_offset(prop);
            let pairs1 = ts.elem().get(raw_prop);
            if pairs1 == None {
                break;
            }
            let pairs2 = pairs1.unwrap().os();
            if pairs2.is_empty() {
                break;
            }
            let pairs1 = pairs1.unwrap().os();
            for pair1 in &*pairs1 {
                for pair2 in &*pairs2 {
                    if pair1[0] > pair2[0] {
                        break;
                    }
                    if pair1[0] == pair2[0] {
                        if pair1[1] != pair2[1] {
                            output.push([pair1[1], NodeDictionary::owlsameAs as u64, pair2[1]])
                        }
                    }
                }
            }
        }
    }
    output
}

pub fn PRP_TRP(ts: &TripleStore) -> RuleResult {
    let mut output = vec![];
    let pairs = ts.elem().get(NodeDictionary::prop_idx_to_offset(
        NodeDictionary::rdftype as u64,
    ));
    if pairs == None {
        return output;
    }
    let pairs = pairs.unwrap().os();
    if pairs.is_empty() {
        return output;
    }
    let transitive = NodeDictionary::owltransitiveProperty as u64;
    let mut start = 0;
    let mut val = pairs[start][0];
    if val > transitive {
        return output;
    }
    if pairs[pairs.len() - 1][0] < transitive {
        return output;
    }
    while val < transitive {
        start += 1;
        val = pairs[start][0];
    }
    for idx in start..pairs.len() {
        let [val, prop] = pairs[idx];
        if val != transitive {
            break;
        }
        if prop != NodeDictionary::rdfssubClassOf as u64
            && prop != NodeDictionary::rdfssubPropertyOf as u64
            && prop != NodeDictionary::owlsameAs as u64
        {
            let pairs = ts.elem().get(NodeDictionary::prop_idx_to_offset(prop));
            if pairs == None {
                break;
            }
            let pairs2 = pairs.unwrap().so();
            let pairs3 = pairs.unwrap().os();
            let mut counter = 0;
            for i in 0..pairs2.len() {
                let [s1, o1] = pairs2[i];
                for j in counter..pairs3.len() {
                    let [o2, s2] = pairs3[j];
                    if s1 == o2 {
                        output.push([s2, prop, o1]);
                    } else if o2 > s1 {
                        counter = j;
                        break;
                    }
                }
            }
        }
    }
    output
}

pub fn finalize(graph: &mut InfGraph) {
    let type_index = NodeDictionary::prop_idx_to_offset(NodeDictionary::rdftype as u64);
    graph.store_mut().ensure_prop(type_index);
    let res = NodeDictionary::rdfsResource;
    ((NodeDictionary::START_INDEX as u64 + 1)..=graph.dict().get_res_ctr()).for_each(|e| {
        if !graph.dict().was_remapped(e) {
            graph.store_mut().add_triple_raw(e, type_index, res);
        }
    });
}
