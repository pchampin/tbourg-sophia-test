use crate::inferray::{NodeDictionary, TripleStore};

#[cfg_attr(debug_assertions, flamer::flame)]
fn apply_gamma_rule(
    ts: &TripleStore,
    head_prop: usize,
    output_prop: u64,
    subject: bool,
    raw_idx: bool,
) -> Vec<[u64; 3]> {
    let mut output = vec![];
    let pairs1 = ts.elem().get(head_prop);
    if pairs1 == None {
        return output;
    }
    let pairs1 = pairs1.unwrap().so();
    for pair1 in pairs1 {
        let pairs2 = ts.elem().get(NodeDictionary::prop_idx_to_idx(pair1[0]));
        if pairs2 == None {
            break;
        }
        let pairs2 = pairs2.unwrap().so();
        for pair2 in pairs2 {
            if raw_idx {
                output.push([pair2[if subject { 0 } else { 1 }], output_prop, pair1[1]]);
            } else {
                output.push([pair2[0], pair1[1], pair2[1]]);
            }
        }
    }
    output
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn PRP_DOM(ts: &TripleStore) -> Vec<[u64; 3]> {
    apply_gamma_rule(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfsdomain as u64),
        NodeDictionary::rdftype as u64,
        true,
        true,
    )
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn PRP_RNG(ts: &TripleStore) -> Vec<[u64; 3]> {
    apply_gamma_rule(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfsrange as u64),
        NodeDictionary::rdftype as u64,
        false,
        true,
    )
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn PRP_SPO1(ts: &TripleStore) -> Vec<[u64; 3]> {
    apply_gamma_rule(
        ts,
        NodeDictionary::prop_idx_to_idx(NodeDictionary::rdfssubPropertyOf as u64),
        0,
        false,
        false,
    )
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn PRP_SYMP(ts: &TripleStore) -> Vec<[u64; 3]> {
    let mut output = vec![];
    let expected_ip = NodeDictionary::prop_idx_to_idx(NodeDictionary::rdftype as u64);
    let expected_io = NodeDictionary::owlsymmetricProperty as u64;
    let pairs1 = ts.elem().get(expected_ip);
    if pairs1 == None {
        return output;
    }
    let pairs1 = pairs1.unwrap().os(); // os sorted copy
    for pair1 in &*pairs1 {
        if pair1[0] == expected_io {
            let pairs2 = ts.elem().get(NodeDictionary::prop_idx_to_idx(pair1[1]));
            if pairs2 == None {
                break;
            }
            let pairs2 = pairs2.unwrap().so();
            for pair2 in pairs2 {
                output.push([pair2[1], pair1[1], pair2[0]]);
            }
        }
        if pair1[0] > expected_io {
            break;
        }
    }
    output
}

#[cfg_attr(debug_assertions, flamer::flame)]
pub fn EQ_TRANS(ts: &TripleStore) -> Vec<[u64; 3]> {
    let pairs = ts.elem().get(NodeDictionary::prop_idx_to_idx(
        NodeDictionary::owlsameAs as u64,
    ));
    if pairs == None {
        return vec![];
    }
    let pairs1 = pairs.unwrap();
    let pairs2 = pairs.unwrap();
    let mut output = vec![];
    for pair1 in pairs1.so() {
        for pair2 in pairs2.so() {
            if pair1[1] == pair2[0] {
                if pair1[0] != pair2[1] {
                    output.push([pair1[0], NodeDictionary::owlsameAs as u64, pair2[1]]);
                    output.push([pair2[1], NodeDictionary::owlsameAs as u64, pair1[0]]);
                }
            }
            if pair2[0] > pair1[1] {
                break;
            }
        }
    }
    output
}
