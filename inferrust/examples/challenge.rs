// This example runs the challenge proposed at:
// https://gist.github.com/justin2004/f9d07adf4e7c2c422be3e0ba92f278d2

use inferrust::*;

use sophia::graph::Graph;
use time::precise_time_ns;

use std::fs;
use std::io::Write;
use sophia::term::{BoxTerm, TTerm};
use sophia::ns::rdf;
use sophia::triple::Triple;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: challenge [tbox.ttl] [abox.ttl]");
        std::process::exit(-1);
    }
    let tbox = &args[1];
    let abox = &args[2];
    eprintln!(
        "#load time, graphsize, proc time,  inferred,"
    );
    std::io::stderr().flush().expect("flush stderr");

    let mut ttl = fs::read_to_string(tbox).expect("open tbox");
    ttl.push_str(&fs::read_to_string(abox).expect("open abox"));
    let t0 = precise_time_ns();
    let ts = sophia::parser::turtle::parse_str(&ttl);
    let mut i_graph = InfGraph::new_unprocessed(ts).expect("error during parsing");
    let t1 = precise_time_ns();
    let initial_size = i_graph.size();
    let t1b = precise_time_ns();
    let load_time = (t1 - t0) as f64 / 1e9;
    eprint!("#{:9.6}, {:9}, ", load_time, initial_size,);
    std::io::stderr().flush().expect("flush stderr");

    i_graph.process(&RuleProfile::RDFS());
    let t2 = precise_time_ns();
    let process_time = (t2 - t1b) as f64 / 1e9;
    let inferred = i_graph.size();
    eprintln!("{:9.6}, {:+9}, ", process_time, inferred - initial_size);
    std::io::stderr().flush().expect("flush stderr");

    let c0 = BoxTerm::new_iri_unchecked("http://example.com/condition0");
    for t in i_graph.triples_with_sp(&c0, &rdf::type_) {
        println!("{}", t.unwrap().o().value())
    }
}
