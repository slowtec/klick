use super::*;

fn internal_node_names() -> Vec<String> {
    [
        "N2oEmissions",
        "TotalEmissions",
        "DirectEmissions",
        "Ch4Emissions",
        "IndirectEmissions",
        "OperatingMaterials",
        "OtherIndirectEmissions",
    ]
    .iter()
    .map(|e| (*e).to_string())
    .collect()
}

#[test]
fn empty() {
    let input = "";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<CustomEmission> = vec![];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn empty_spaces() {
    let input = "  ";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<CustomEmission> = vec![];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn empty_spaces__() {
    assert_eq!(true, true);
}

#[test]
fn empty_spaces_tab() {
    let input = " \t ";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<CustomEmission> = vec![];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_undefined() {
    let input = "\"asdf1\" \"asdf2\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<CustomEmission> = vec![CustomEmission::EdgeUndefined(EdgeUndefined {
        line: 1,
        source: "asdf1".to_string(),
        target: "asdf2".to_string(),
    })];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_undefined_umlauts() {
    let input = "\"H₂ Generator\"   \"fällmittel\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<CustomEmission> = vec![CustomEmission::EdgeUndefined(EdgeUndefined {
        line: 1,
        source: "H₂ Generator".to_string(),
        target: "fällmittel".to_string(),
    })];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_undefined_other() {
    let input = "\"H₂ Generator\"   \"Midtbø\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<CustomEmission> = vec![CustomEmission::EdgeUndefined(EdgeUndefined {
        line: 1,
        source: "H₂ Generator".to_string(),
        target: "Midtbø".to_string(),
    })];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_defined() {
    let input = "\"asdf1\" 1,1 \"asdf2\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<CustomEmission> = vec![CustomEmission::EdgeDefined(EdgeDefined {
        line: 1,
        source: "asdf1".to_string(),
        target: "asdf2".to_string(),
        value: 1.1,
    })];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_defined_umlauts() {
    let input = "\"H₂ Generator\" 1,1 \"fällmittel\"";
    let output = parse_emission(input, NumberFormat::DE).unwrap();
    let q: Vec<CustomEmission> = vec![CustomEmission::EdgeDefined(EdgeDefined {
        line: 1,
        source: "H₂ Generator".to_string(),
        target: "fällmittel".to_string(),
        value: 1.1,
    })];
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn fail1() {
    let input = "\"asdf1\" aaa";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn fail2() {
    let input = "\"asdf1\" 1,1 aaa";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn fail3() {
    let input = "\"asdf1\" 1,1";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn fail4() {
    let input = "a 1,1 a";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn fail5() {
    let input = "a a";
    let output = parse_emission(input, NumberFormat::DE);
    assert!(output.is_err());
}

#[test]
fn edge_defined_de_floats_de() {
    let floats = vec![
        "1",
        "1,",
        "1,",
        "1,1",
        "2,",
        "1,0",
        "1,000",
        "1,0000",
        "0,5",
        "10,00",
        "1000",
        "1000,00",
        "1000,1",
        "1200,000",
        "100000,23",
        "1,0000",
        "0,0001",
        "2345,67",
        "1234567,89",
        "1000000,000",
        "10000,0001",
        "1,2345",
        "0,000001",
        "999,999",
    ];
    for f in floats {
        let input = format!("\"a\" {f} \"b\"");
        let number_format = NumberFormat::DE;
        // println!("input {input}");
        let output = parse_emission(&input, NumberFormat::DE).unwrap();
        let v = number_format.parse_number(f).unwrap();
        let edges = vec![CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "a".to_string(),
            target: "b".to_string(),
            value: v,
        })];
        let q: Vec<CustomEmission> = edges;
        assert_eq!(output.as_slice(), q.as_slice());
    }
}

#[test]
fn edge_defined_us() {
    let input = "\"asdf1\" 1.1 \"asdf2\"";
    let output = parse_emission(input, NumberFormat::US).unwrap();
    let edges = vec![CustomEmission::EdgeDefined(EdgeDefined {
        line: 1,
        source: "asdf1".to_string(),
        target: "asdf2".to_string(),
        value: 1.1,
    })];
    let q: Vec<CustomEmission> = edges;
    assert_eq!(output.as_slice(), q.as_slice());
}

#[test]
fn edge_defined_de_floats_us() {
    let floats = vec![
        "1",
        ".1",
        "1.",
        "1.1",
        "2.",
        "1.0",
        "1.000",
        "1.0000",
        "0.5",
        "10.00",
        "1.000",
        "1000.00",
        "1000.1",
        "1200.000",
        "100000.23",
        "1.0000",
        "0.0001",
        "2345.67",
        "1234.56789",
        "1000000.000",
        "10000.0001",
        "1.2345",
        "0.000001",
        "999.999",
    ];
    for f in floats {
        let input = format!("\"a\" {f} \"b\"");
        // println!("input {input}");
        let number_format = NumberFormat::US;
        let output = parse_emission(&input, number_format).unwrap();
        let v = number_format.parse_number(f).unwrap();

        let edges = vec![CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "a".to_string(),
            target: "b".to_string(),
            value: v,
        })];
        let q: Vec<CustomEmission> = edges;
        assert_eq!(output.as_slice(), q.as_slice());
    }
}

#[test]
fn check_graph_empty() {
    let edges = &[];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    assert!(r.is_ok());
}

#[test]
fn check_graph_cycle_edge_defined() {
    let edges = &[CustomEmission::EdgeDefined(EdgeDefined {
        line: 1,
        source: "a".to_string(),
        target: "a".to_string(),
        value: 1.1,
    })];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    match r {
        Err(CustomEmissionParserError::InsideEdgeCycleVoilation { line }) => {
            assert_eq!(line, 1);
        }
        _ => panic!("Expected InsideEdgeCycleVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_cycle_edge_undefined() {
    let edges = &[CustomEmission::EdgeUndefined(EdgeUndefined {
        line: 1,
        source: "a".to_string(),
        target: "a".to_string(),
    })];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    match r {
        Err(CustomEmissionParserError::InsideEdgeCycleVoilation { line }) => {
            assert_eq!(line, 1);
        }
        _ => panic!("Expected InsideEdgeCycleVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_cycle_two_nodes() {
    let edges = &[
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 1,
            source: "a".to_string(),
            target: "b".to_string(),
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 2,
            source: "b".to_string(),
            target: "a".to_string(),
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 3,
            source: "b".to_string(),
            target: "TotalEmissions".to_string(),
        }),
    ];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    let node_name = "b".to_string();
    match r {
        Err(CustomEmissionParserError::EdgeCycleVoilation { name, line }) => {
            assert_eq!(name, node_name);
            assert_eq!(line, 2);
        }
        _ => panic!("Expected EdgeCycleVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_cycle_many_nodes() {
    let edges = &[
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 1,
            source: "a".to_string(),
            target: "b".to_string(),
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 2,
            source: "b".to_string(),
            target: "c".to_string(),
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 3,
            source: "b".to_string(),
            target: "a".to_string(),
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 4,
            source: "c".to_string(),
            target: "TotalEmissions".to_string(),
        }),
    ];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    let node_name = "b".to_string();
    match r {
        Err(CustomEmissionParserError::EdgeCycleVoilation { name, line }) => {
            assert_eq!(name, node_name);
            assert_eq!(line, 3);
        }
        _ => panic!("Expected EdgeCycleVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_cycle_many_nodes_two() {
    let edges = &[
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "a".to_string(),
            target: "b".to_string(),
            value: 1.1,
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 2,
            source: "c".to_string(),
            target: "b".to_string(),
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 3,
            source: "b".to_string(),
            target: "c".to_string(),
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 4,
            source: "c".to_string(),
            target: "TotalEmissions".to_string(),
        }),
    ];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    let node_name = "c".to_string();
    match r {
        Err(CustomEmissionParserError::EdgeCycleVoilation { name, line }) => {
            assert_eq!(name, node_name);
            assert_eq!(line, 2);
        }
        _ => panic!("Expected EdgeCycleVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_conflict_source_name_not_unique() {
    let edges = &[
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "a".to_string(),
            target: "b".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 2,
            source: "a".to_string(),
            target: "b".to_string(),
            value: 1.2,
        }),
    ];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    match r {
        Err(CustomEmissionParserError::EdgeNotUniqueVoilation { e1_line, e2_line }) => {
            assert_eq!(e1_line, 1);
            assert_eq!(e2_line, 2);
        }
        _ => panic!("Expected EdgeNotUniqueVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_too_many_values() {
    let edges = &[
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "a".to_string(),
            target: "b".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 2,
            source: "b".to_string(),
            target: "c".to_string(),
            value: 1.2,
        }),
    ];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    match r {
        Err(CustomEmissionParserError::NodeToNodeLinkVoilation { e1_line, e2_line }) => {
            assert_eq!(e1_line, 1);
            assert_eq!(e2_line, 2);
        }
        _ => panic!("Expected NodeToNodeLinkVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_unconnected_edge() {
    let edges = &[CustomEmission::EdgeUndefined(EdgeUndefined {
        line: 1,
        source: "a".to_string(),
        target: "b".to_string(),
    })];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    let found_lines = "1".to_string();
    match r {
        Err(CustomEmissionParserError::DetachedNodesVoilation { lines }) => {
            assert_eq!(lines, found_lines);
        }
        _ => panic!("Expected DetachedNodesVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_unconnected_edges() {
    let edges = &[
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "foo".to_string(),
            target: "TotalEmissions".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 2,
            source: "bar".to_string(),
            target: "unconnected".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 3,
            source: "batz".to_string(),
            target: "TotalEmissions".to_string(),
            value: 1.2,
        }),
    ];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    let found_lines = "2".to_string();
    match r {
        Err(CustomEmissionParserError::DetachedNodesVoilation { lines }) => {
            assert_eq!(lines, found_lines);
        }
        _ => panic!("Expected DetachedNodesVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_emission_node_links_emission_node() {
    let edges = &[
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "foo".to_string(),
            target: "TotalEmissions".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 2,
            source: "MyCrazyEmission".to_string(),
            target: "foo".to_string(),
            value: 1.2,
        }),
    ];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    match r {
        Err(CustomEmissionParserError::NodeToNodeLinkVoilation { e1_line, e2_line }) => {
            assert_eq!(e1_line, 2);
            assert_eq!(e2_line, 1);
        }
        _ => panic!("Expected NodeToNodeLinkVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_proper_edge() {
    let q = &[CustomEmission::EdgeDefined(EdgeDefined {
        line: 1,
        source: "a".to_string(),
        target: "TotalEmissions".to_string(),
        value: 1.2,
    })];
    let r = check_graph(q, internal_node_names());
    assert!(r.is_ok());
}

#[test]
fn check_graph_using_reserved_names() {
    let edges = &[
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "TotalEmissions".to_string(),
            target: "a".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "DirectEmissions".to_string(),
            target: "a".to_string(),
            value: 1.2,
        }),
    ];
    let q = edges;
    let r = check_graph(q, internal_node_names());
    let node_name = "TotalEmissions".to_string();
    match r {
        Err(CustomEmissionParserError::ReservedNameVoilation { name, line }) => {
            assert_eq!(name, node_name);
            assert_eq!(line, 1);
        }
        _ => panic!("Expected ReservedNameVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_duplicate_name() {
    let q = &[
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "a".to_string(),
            target: "TotalEmissions".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 2,
            source: "a".to_string(),
            target: "DirectEmissions".to_string(),
            value: 1.2,
        }),
    ];
    let r = check_graph(q, internal_node_names());
    match r {
        Err(CustomEmissionParserError::DuplicatedNodeNameVoilation { e1_line, e2_line }) => {
            assert_eq!(e1_line, 1);
            assert_eq!(e2_line, 2);
        }
        _ => panic!("Expected DuplicatedNodeNameVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_edge_to_leaf() {
    let q = &[
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 1,
            source: "a".to_string(),
            target: "b".to_string(),
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 2,
            source: "b".to_string(),
            target: "DirectEmissions".to_string(),
            value: 1.2,
        }),
    ];
    let r = check_graph(q, internal_node_names());
    match r {
        Err(CustomEmissionParserError::EdgeToLeafVoilation { e1_line, e2_line }) => {
            assert_eq!(e1_line, 1);
            assert_eq!(e2_line, 2);
        }
        _ => panic!("Expected EdgeToLeafVoilation, but got {r:?}"),
    }
}

#[test]
fn check_graph_example() {
    let q = &[
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 1,
            source: "H₂ Generator".to_string(),
            target: "OtherIndirectEmissions".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 2,
            source: "Fällmittel (AI)".to_string(),
            target: "fällmittel".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 3,
            source: "Fällmittel (Eisen(II)-sulfat)".to_string(),
            target: "fällmittel".to_string(),
            value: 1.2,
        }),
        CustomEmission::EdgeUndefined(EdgeUndefined {
            line: 4,
            source: "fällmittel".to_string(),
            target: "TotalEmissions".to_string(),
        }),
        CustomEmission::EdgeDefined(EdgeDefined {
            line: 5,
            source: "Flüssiggas".to_string(),
            target: "DirectEmissions".to_string(),
            value: 1.2,
        }),
    ];
    let r = check_graph(q, internal_node_names());
    println!("{r:?}");
    assert!(r.is_ok());
}
