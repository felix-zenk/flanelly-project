fn generate_test(name: &str) {
    let input: String = read_to_string(format!("test/{:}/prog.while", name)).unwrap();
    let ast = parser::parse(&input).unwrap();
    let cfg = cfg::ast_to_cfg(&ast);
    let cfg_const_prop: Cfg<MfpAnnot<MultiConstLat>> = mfp(&cfg);
    let cfg_avail_exp: Cfg<MfpAnnot<ExpSetLat>> = mfp(&cfg);
    fs::write(format!("test/{:}/ast.json", name), serde_json::to_string(&ast).unwrap()).unwrap();
    fs::write(format!("test/{:}/cfg.json", name), serde_json::to_string(&cfg).unwrap()).unwrap();
    fs::write(format!("test/{:}/cfg_const_prop.json", name), serde_json::to_string(&cfg_const_prop).unwrap()).unwrap();
    fs::write(format!("test/{:}/cfg_avail_exp.json", name), serde_json::to_string(&cfg_avail_exp).unwrap()).unwrap();
}