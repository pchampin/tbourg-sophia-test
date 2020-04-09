use inferrust::closure::ClosureGraph;
use inferrust::inferray::*;
use inferrust::rules::*;

use time::precise_time_ns;

use std::fs;
use std::io::BufRead;

use sophia::serializer::nt::*;
use sophia::serializer::*;

fn main() {
    let mut profiles = [
        RuleProfile::RDFS(),
        RuleProfile::RhoDF(),
        RuleProfile::RDFSPlus(),
    ];
    fs::read_dir("inferrust/res/bsbm")
        .unwrap()
        .for_each(|file| {
            println!("file: {:#?}", &file);
            let rep = fs::read_to_string(file.unwrap().path()).unwrap();
            profiles.iter_mut().for_each(|profile| {
                let ts = sophia::parser::turtle::parse_str(&rep);
                let mut i_graph = InfGraph::from(ts);
                println!("graph size: {}", i_graph.size());
                let t0 = precise_time_ns();
                i_graph.process(profile);
                let t1 = precise_time_ns();
                let time = (t1 - t0) as f64 / 1e9;
                println!(
                    "profile: {}, graph size: {}, time: {}",
                    profile.name(),
                    i_graph.size(),
                    time
                );
            })
        });
}
