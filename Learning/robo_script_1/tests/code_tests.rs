use robo_script_1::*;

#[test]
fn test_wrap_it(){
    assert_eq!(String::from("<span style=\"color: pink\">FF</span>"), wrap_it("FF"));
    assert_eq!(String::from("<span style=\"color: red\">L</span>"), wrap_it("L"));
    assert_eq!(String::from("<span style=\"color: green\">RRR</span>"), wrap_it("RRR"));
    assert_eq!(String::from("<span style=\"color: orange\">0</span>"), wrap_it("0"));
    assert_eq!(String::from("<span style=\"color: orange\">9</span>"), wrap_it("9"));
    assert_eq!(String::from("<span style=\"color: orange\">9</span>"), wrap_it("9"));
    assert_eq!(String::from(""), wrap_it(")"));
    assert_ne!(String::from("<span style=\"color: \">(</span>"), wrap_it("("));
}


#[test]
fn test_highlight() {
    let query1 = "F3RF5LF7";
    let query2 = "FFFR345F2LL";
    let query3 = "RRRRR(F45L3)F2";
    let res1 = String::from("<span style=\"color: pink\">F</span><span style=\"color: orange\">3</span><span style=\"color: green\">R</span><span style=\"color: pink\">F</span><span style=\"color: orange\">5</span><span style=\"color: red\">L</span><span style=\"color: pink\">F</span><span style=\"color: orange\">7</span>");
    let res2 = String::from("<span style=\"color: pink\">FFF</span><span style=\"color: green\">R</span><span style=\"color: orange\">345</span><span style=\"color: pink\">F</span><span style=\"color: orange\">2</span><span style=\"color: red\">LL</span>");

    assert_eq!(res1, highlight(query1));
    assert_eq!(res2, highlight(query2));
}