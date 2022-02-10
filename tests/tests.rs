use std::{fs::{read_to_string}};

use test_generator::test_resources;

use flanelly::cfg::{Cfg, RawAnnot};

use flanelly::{parser, interpreter::eval, cfg, flow_analysis::mfp::MfpAnnot, flow_analysis::const_prop::MultiConstLat, flow_analysis::mfp::mfp, ast::Prog, flow_analysis::avail_exp::ExpSetLat};

#[test_resources("tests-res/*")]
fn test_parser(name: &str) {
    let input: String = read_to_string(format!("{:}/prog.while", name)).unwrap();
    let expected: Prog = serde_json::from_str(&read_to_string(format!("{:}/ast.json", name)).unwrap()).unwrap();
    let actual = parser::parse(&input).unwrap();
    assert_eq!(expected, actual);
}

#[test_resources("tests-res/*")]
fn test_eval(name: &str) {
    let prog: Prog = serde_json::from_str(&read_to_string(format!("{:}/ast.json", name)).unwrap()).unwrap();
    let cases: Vec<(i32, i32)> = serde_json::from_str(&read_to_string(format!("{:}/eval.json", name)).unwrap()).unwrap();
    cases.iter().for_each(|(x, y)| {
        assert_eq!(eval(&prog, *x), *y);
    });
}

#[test_resources("tests-res/*")]
fn test_ast_to_cfg(name: &str) {
    let input: Prog = serde_json::from_str(&read_to_string(format!("{:}/ast.json", name)).unwrap()).unwrap();
    let expected: Cfg<RawAnnot> = serde_json::from_str(&read_to_string(format!("{:}/cfg.json", name)).unwrap()).unwrap();
    let actual = cfg::ast_to_cfg(&input);
    assert_eq!(expected, actual);
}

#[test_resources("tests-res/*")]
fn test_const_prop(name: &str) {
    let input: Cfg<RawAnnot> = serde_json::from_str(&read_to_string(format!("{:}/cfg.json", name)).unwrap()).unwrap();
    let expected: Cfg<MfpAnnot<MultiConstLat>> = serde_json::from_str(&read_to_string(format!("{:}/cfg_const_prop.json", name)).unwrap()).unwrap();
    let actual: Cfg<MfpAnnot<MultiConstLat>> = mfp(&input);
    assert_eq!(expected, actual);
}

#[test_resources("tests-res/*")]
fn test_avail_exp(name: &str) {
    let input: Cfg<RawAnnot> = serde_json::from_str(&read_to_string(format!("{:}/cfg.json", name)).unwrap()).unwrap();
    let expected: Cfg<MfpAnnot<ExpSetLat>> = serde_json::from_str(&read_to_string(format!("{:}/cfg_avail_exp.json", name)).unwrap()).unwrap();
    let actual: Cfg<MfpAnnot<ExpSetLat>> = mfp(&input);
    assert_eq!(expected, actual);
}