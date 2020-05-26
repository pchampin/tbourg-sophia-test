use crate::closure::{ClosureGraph, Node};

use std::collections::HashSet;

/// Source: https://pdfs.semanticscholar.org/47cc/a59310abee097af31d678d6cb2f8263dee37.pdf?_ga=2.26709177.588007852.1584345117-1155404888.1573749711
/// Figure 4

pub fn graph_tc(g: &ClosureGraph) {
    let mut stack = Vec::new();
    let mut num = 0;

    fn node_tc(v: &Node, stack: &mut Vec<u64>, g: &ClosureGraph, num: &mut usize) {
        v.set_num(*num);
        *num += 1;
        stack.push(v.id);
        v.set_root(v.id);
        let mut adj_comp_roots = HashSet::new();
        for wi in g.edges(v.id) {
            let w = g.node(wi);
            if w.num() == usize::max_value() {
                node_tc(&w, stack, g, num);
                let vroot = g.node(v.root());
                let wroot = g.node(w.root());
                v.set_root(minn(&vroot, &wroot));
                if w.in_comp() {
                    adj_comp_roots.insert(w.root());
                }
            } else if v.num() > w.num() {
                if !w.in_comp() {
                    let vroot = g.node(v.root());
                    let wroot = g.node(w.root());
                    v.set_root(minn(&vroot, &wroot));
                } else {
                    adj_comp_roots.insert(w.root());
                }
            }
        }
        for r in adj_comp_roots.iter() {
            if !g.node(v.root()).tc_contains(*r) {
                let tc_r = g.node(*r).tc_iter();
                let root_v = g.node(v.root());
                root_v.tc_insert(*r);
                root_v.tc_extend(tc_r);
            }
        }
        if v.root() == v.id {
            let top = g.node(*stack.last().unwrap());
            if top.num() > v.num() {
                v.tc_insert(v.id);
            }
            let mut wid = stack.pop().unwrap();
            while wid != v.id {
                let w = g.node(wid);
                w.set_in_comp(true);
                if !w.tc_is_empty() {
                    v.tc_extend(w.tc_iter());
                }
                w.set_root(v.id);
                wid = stack.pop().unwrap();
            }
            v.set_in_comp(true);
        } else {
            let root_v = g.node(v.root());
            root_v.tc_insert(v.id);
            root_v.tc_extend(v.tc_iter());
            v.tc_clear();
        }
    }
    for v in g.iter_nodes() {
        if v.num() == usize::max_value() {
            node_tc(v, &mut stack, g, &mut num);
        }
    }
}

fn minn(a: &Node, b: &Node) -> u64 {
    if a.num() <= b.num() {
        a.id
    } else {
        b.id
    }
}
