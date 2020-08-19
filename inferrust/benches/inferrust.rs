use criterion::{criterion_group, criterion_main, Criterion};

use inferrust::inferray::*;
use inferrust::rules::*;

// retrieve the mirror at http://swat.cse.lehigh.edu/onto/univ-bench-dl.owl
// and converted to N-Triples
const UOBM_ONTO: &str = "benches/univ-bench-dl.nt";

pub fn uobm_total(c: &mut Criterion) {
    let data = std::fs::read_to_string(UOBM_ONTO).expect("open file");
    c.bench_function("uobm_total", |b| {
        b.iter(|| {
            let mut graph = InfGraph::from(sophia::parser::nt::parse_str(&data));
            graph.process(&mut RuleProfile::RDFSPlus());
        })
    });
}

pub fn uobm_load(c: &mut Criterion) {
    let data = std::fs::read_to_string(UOBM_ONTO).expect("open file");
    c.bench_function("uobm_load", |b| {
        b.iter(|| {
            let mut graph = InfGraph::from(sophia::parser::nt::parse_str(&data));
            assert_eq!(graph.size(), 711);
        })
    });
}

criterion_group!(benches, uobm_total, uobm_load);
criterion_main!(benches);
