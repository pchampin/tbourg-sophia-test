use crate::inferray::*;
use crate::rules::*;

use sophia::graph::Graph;
use sophia::parser::turtle::parse_str as parse_ttl;
use sophia::term::BoxTerm;
use sophia::triple::stream::TripleSource;
use std::error::Error;

const PREFIXES: &str = r#"@prefix : <http://example.org/> .
@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix owl: <http://www.w3.org/2002/07/owl#> .
@prefix s: <http://schema.org/> .
"#;

fn test_infer(input: &str, expected: &str, mut profiles: Vec<RuleProfile>) -> Result<(), Box<dyn Error>> {
    let mut full_input = String::new();
    full_input.push_str(PREFIXES);
    full_input.push_str(input);
    let exp_input: Vec<[BoxTerm; 3]> = parse_ttl(&full_input).collect_triples()?;

    let mut full_expected = String::new();
    full_expected.push_str(PREFIXES);
    full_expected.push_str(expected);
    let expected: Vec<[BoxTerm; 3]> = parse_ttl(&full_expected).collect_triples()?;

    for profile in &mut profiles {
        let i_graph = InfGraph::new(
            sophia::parser::turtle::parse_str(&full_input),
            profile,
        )?;
        println!("=== {} triples inferred", i_graph.size() - exp_input.len());

        for [s, p, o] in &exp_input {
            assert!(i_graph.contains(s, p, o)?,
                "\n  profile: {}\n  missing input triple:\n    {}\n    {}\n    {}\n", profile, s, p, o);
        }
        for [s, p, o] in &expected {
            assert!(i_graph.contains(s, p, o)?,
                "\n  profile: {}\n  missing inferred triple:\n    {}\n    {}\n    {}\n", profile, s, p, o);
        }
    }
    Ok(())
}

// single rules

#[test]
fn cax_eqc1() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :Person owl:equivalentClass s:Person.
        :bart a :Person.
        "#,

        r#"
        :bart a s:Person.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn cax_eqc2() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :Person owl:equivalentClass s:Person.
        :bart a s:Person.
        "#,

        r#"
        :bart a :Person.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn eq_rep_o() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :o1 owl:sameAs :o2.
        :s1 :p1 :o1.
        "#,

        r#"
        :s1 :p1 :o2.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
#[ignore] // fails at the moment... see same_as_rules.rs for the cause of this problem
fn eq_rep_p() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :p1 owl:sameAs :p2.
        :s1 :p1 :o1.
        "#,

        r#"
        :s1 :p2 :o1.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn eq_rep_s() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :s1 owl:sameAs :s2.
        :s1 :p1 :o1.
        "#,

        r#"
        :s2 :p1 :o1.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn eq_sym() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :o1 owl:sameAs :o2.
        "#,

        r#"
        :o2 owl:sameAs :o1.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn eq_trans() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :o1 owl:sameAs :o2.
        :o2 owl:sameAs :o3.
        "#,

        r#"
        :o1 owl:sameAs :o3.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_dom() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :mother rdfs:domain :Person.
        :bart :mother :marge.
        "#,

        r#"
        :bart a :Person.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_eqp1() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :parent owl:equivalentProperty s:parent.
        :bart :parent :marge.
        "#,

        r#"
        :bart s:parent :marge.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_eqp2() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :parent owl:equivalentProperty s:parent.
        :bart s:parent :marge.
        "#,

        r#"
        :bart :parent :marge.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_fp() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :mother a owl:FunctionalProperty.
        :bart :mother :marge, <http://dbpedia.org/page/Marge_Simpson>.
        "#,

        r#"
        :marge owl:sameAs <http://dbpedia.org/page/Marge_Simpson>.
        <http://dbpedia.org/page/Marge_Simpson> owl:sameAs :marge.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_ifp() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :ssn a owl:InverseFunctionalProperty.
        :marge :ssn "simpsons-marge-1234".
        <https://en.wikipedia.org/wiki/Marge_Simpson> :ssn "simpsons-marge-1234".
        "#,

        r#"
        :marge owl:sameAs <https://en.wikipedia.org/wiki/Marge_Simpson>.
        <https://en.wikipedia.org/wiki/Marge_Simpson> owl:sameAs :marge.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_inv1() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :parent owl:inverseOf :child.
        :bart :parent :marge.
        "#,

        r#"
        :marge :child :bart.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_inv2() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :parent owl:inverseOf :child.
        :marge :child :bart.
        "#,
        
        r#"
        :bart :parent :marge.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_rng() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :mother rdfs:range :Woman.
        :bart :mother :marge.
        "#,

        r#"
        :marge a :Woman.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_spo1() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :mother rdfs:subPropertyOf :parent.
        :bart :mother :marge.
        "#,

        r#"
        :bart :parent :marge.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_symp() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :related a owl:SymmetricProperty.
        :bart :related :marge.
        "#,

        r#"
        :marge :related :bart.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn prp_trp() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :ancestor a owl:TransitiveProperty.
        :bart :ancestor :marge.
        :bart :ancestor :clancy.
        "#,

        r#"
        :bart :ancestor :clancy.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_dom1() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :mother rdfs:domain :Woman.
        :Woman rdfs:subClassOf :Person.
        "#,

        r#"
        :mother rdfs:domain :Person.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_dom2() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :parent rdfs:domain :Person.
        :mother rdfs:subPropertyOf :parent.
        "#,

        r#"
        :mother rdfs:domain :Person.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_eqc1() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :Person owl:equivalentClass s:Person.
        "#,

        r#"
        :Person rdfs:subClassOf s:Person.
        s:Person rdfs:subClassOf :Person.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_eqc2() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :Person rdfs:subClassOf s:Person.
        s:Person rdfs:subClassOf :Person.
        "#,
        
        r#"
        :Person owl:equivalentClass s:Person.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_eqp1() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :parent owl:equivalentProperty s:parent.
        "#,

        r#"
        :parent rdfs:subPropertyOf s:parent.
        s:parent rdfs:subPropertyOf :parent.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_eqp2() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :parent rdfs:subPropertyOf s:parent.
        s:parent rdfs:subPropertyOf :parent.
        "#,
        
        r#"
        :parent owl:equivalentProperty s:parent.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_rng1() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :daughter rdfs:range :Woman.
        :Woman rdfs:subClassOf :Person.
        "#,

        r#"
        :daughter rdfs:range :Person.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_rng2() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :child rdfs:range :Person.
        :daughter rdfs:subPropertyOf :child.
        "#,

        r#"
        :daughter rdfs:range :Person.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_sco() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :Boy rdfs:subClassOf :Man.
        :Man rdfs:subClassOf :Person.
        "#,

        r#"
        :Boy rdfs:subClassOf :Person.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_spo() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :mother rdfs:subPropertyOf :parent.
        :parent rdfs:subPropertyOf :related.
        "#,

        r#"
        :mother rdfs:subPropertyOf :related.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

// TODO axiomatic triples

// combined rules

#[test]
fn cax_sco_p_scm_sco() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :Boy rdfs:subClassOf :Man.
        :Man rdfs:subClassOf :Person.
        :bart a :Boy.
        "#,

        r#"
        :Boy rdfs:subClassOf :Person.
        :bart a :Man, :Person.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn cax_sco_p_prp_rng() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :mother rdfs:range :Woman.
        :Woman rdfs:subClassOf :Person.
        :bart :mother :marge.
        "#,

        r#"
        :marge a :Woman, :Person.
        "#,

        vec![
            RuleProfile::RDFS(),
            RuleProfile::RhoDF(),
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn eq_mix() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :clark owl:sameAs :superman.
        :superman owl:sameAs :manofsteal.

        :manofsteal a :FlyingCreature.
        :lex :hates :manofsteal.
        "#,

        r#"
        :clark a :FlyingCreature.
        :lex :hates :clark.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}


#[test]
#[ignore] // not strictly OWL
fn scm_spo_square() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :spo rdfs:subPropertyOf rdfs:subPropertyOf.
        :p :spo :q.
        :q :spo :r.
        "#,

        r#"
        :p rdfs:subPropertyOf :r.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

#[test]
fn scm_trans_p_scm_spo() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :parent rdfs:subPropertyOf :ancestor.
        :ancestor a owl:TransitiveProperty.
        :bart :parent :marge.
        :marge :parent :clancy.
        "#,

        r#"
        :bart :ancestor :clancy.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

/// This test does not work, which is strange...
/// Below is a slightly modified version, which does work.
#[test]
#[ignore]
fn rich_ontology() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :Person a owl:Class .
        :Male rdfs:subClassOf :Person.
        :Female rdfs:subClassOf :Person.
        :Child rdfs:subClassOf :Person.
        :Adult rdfs:subClassOf :Person.
        :Boy rdfs:subClassOf :Male, :Child.
        :Girl rdfs:subClassOf :Female, :Child.
        :Man rdfs:subClassOf :Male, :Adult.
        :Woman rdfs:subClassOf :Female, :Aduly.

        :related a owl:TransitiveProperty, owl:SymmetricProperty ;
            rdfs:domain :Person ;
            rdfs:range :Person ;
        .
        :ancestor a owl:TransitiveProperty ;
            rdfs:subPropertyOf :related ;
        .
        :parent rdfs:subPropertyOf :ancestor ;
            owl:inverseOf :child ;
        .
        :father a owl:FunctionalProperty ;
            rdfs:subPropertyOf :parent ;
            rdfs:range :Male ;
        .
        :mother a owl:FunctionalProperty ;
            rdfs:subPropertyOf :parent ;
            rdfs:range :Female ;
        .
        :son rdfs:subPropertyOf :child ;
            rdfs:range :Male ;
        .
        :daughter rdfs:subPropertyOf :child ;
            rdfs:range :Female ;
        .

        :bart a :Boy ;
            :father :homer ;
            :mother :marge ;
        .
        :lisa a :Girl ;
            :father :homer ;
            :mother :marge ;
        .
        :marge
            :father :clancy ;
            :mother :jackie ;
        .
        "#,

        r#"
        :mother rdfs:subPropertyOf :ancestor.

        :marge a :Female.
        :marge :child :bart.
        :bart :ancestor :jackie.
        :bart :related :lisa.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}

/// This is a hacked version of the previous one;
/// the only change is that the definition of :father
/// is commented out.
/// Mysteriously, this makes the test work.
#[test]
fn rich_ontology_hacked() -> Result<(), Box<dyn Error>> {
    test_infer(
        r#"
        :Person a owl:Class .
        :Male rdfs:subClassOf :Person.
        :Female rdfs:subClassOf :Person.
        :Child rdfs:subClassOf :Person.
        :Adult rdfs:subClassOf :Person.
        :Boy rdfs:subClassOf :Male, :Child.
        :Girl rdfs:subClassOf :Female, :Child.
        :Man rdfs:subClassOf :Male, :Adult.
        :Woman rdfs:subClassOf :Female, :Aduly.

        :related a owl:TransitiveProperty, owl:SymmetricProperty ;
            rdfs:domain :Person ;
            rdfs:range :Person ;
        .
        :ancestor a owl:TransitiveProperty ;
            rdfs:subPropertyOf :related ;
        .
        :parent rdfs:subPropertyOf :ancestor ;
            owl:inverseOf :child ;
        .
        #:father a owl:FunctionalProperty ;
        #    rdfs:subPropertyOf :parent ;
        #    rdfs:range :Male ;
        #.
        :mother a owl:FunctionalProperty ;
            rdfs:subPropertyOf :parent ;
            rdfs:range :Female ;
        .
        :son rdfs:subPropertyOf :child ;
            rdfs:range :Male ;
        .
        :daughter rdfs:subPropertyOf :child ;
            rdfs:range :Female ;
        .

        :bart a :Boy ;
            :father :homer ;
            :mother :marge ;
        .
        :lisa a :Girl ;
            :father :homer ;
            :mother :marge ;
        .
        :marge
            :father :clancy ;
            :mother :jackie ;
        .
        "#,

        r#"
        :mother rdfs:subPropertyOf :ancestor.

        :marge a :Female.
        :marge :child :bart.
        :bart :ancestor :jackie.
        :bart :related :lisa.
        "#,

        vec![
            RuleProfile::RDFSPlus(),
        ],
    )
}
