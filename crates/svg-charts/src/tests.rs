#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::sankey::{NodeId, Dependencies, Node, *};

    // #[test]
    // fn test_sort() {
    //     let mut nodes: HashMap<NodeId, Node> = HashMap::new();
    //     nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 100.0, color: None });
    //     nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 10.0, color: None });
    //     nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 1111.0, color: None });
    //     nodes.insert(NodeId(3), Node { label: Some("2".to_string()), value: 1111.0, color: None });
    //     let mut layers: Vec<NodeId> = vec![NodeId(0), NodeId(1), NodeId(2), NodeId(3)];
    //     layers.sort_by(|a, b| nodes[a].value.partial_cmp(&nodes[b].value).unwrap_or(std::cmp::Ordering::Equal));
    //     assert_eq!(layers, vec![NodeId(1), NodeId(0), NodeId(2), NodeId(3)]);
    // }
    //
    #[test]
    fn test_layers_empty() {
        let deps: HashMap<NodeId, Dependencies> = HashMap::new();

        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &HashMap::new(), vec![]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();

        let empty: Vec<Vec<NodeId>> = vec![vec![]];
        assert_eq!(final_root_layers, empty);
    }

    #[test]
    fn test_layers_basic() {
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(1)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![NodeId(0)], outputs: vec![NodeId(2)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![NodeId(1)], outputs: vec![]});

        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &HashMap::new(), vec![NodeId(2)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();

        assert_eq!(final_root_layers, vec![vec![NodeId(0)], vec![NodeId(1)], vec![NodeId(2)]]);
    }
    //
    #[test]
    fn test_layers_sort() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 10.0, color: None });
        nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 1.0, color: None });
        nodes.insert(NodeId(3), Node { label: Some("3".to_string()), value: 111.0, color: None });
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(3), Dependencies { inputs: vec![NodeId(0), NodeId(1), NodeId(2)], outputs: vec![] });
        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &nodes, vec![NodeId(3)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();
        assert_eq!(final_root_layers, vec![vec![NodeId(2), NodeId(1), NodeId(0)], vec![NodeId(3)]]);
    }
    #[test]
    fn test_layers_sort_clone_different_weights() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 1.0, color: None });
        nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 10.0, color: None });
        nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(3), Node { label: Some("3".to_string()), value: 111.0, color: None });
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(3), Dependencies { inputs: vec![NodeId(0), NodeId(1), NodeId(2)], outputs: vec![] });
        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &nodes, vec![NodeId(3)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();
        assert_eq!(final_root_layers, vec![vec![NodeId(0), NodeId(1), NodeId(2)], vec![NodeId(3)]]);
    }
    #[test]
    fn test_layers_long_sort() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 1.0, color: None });
        nodes.insert(NodeId(3), Node { label: Some("3".to_string()), value: 201.0, color: None });
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(1)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![NodeId(0)], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(3), Dependencies { inputs: vec![NodeId(1), NodeId(2)], outputs: vec![] });

        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &nodes, vec![NodeId(3)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();
        assert_eq!(final_root_layers, vec![vec![NodeId(2), NodeId(0)], vec![NodeId(1)], vec![NodeId(3)]]);
    }
    #[test]
    fn test_layers_long_sort_clone_different_weights() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 1.0, color: None });
        nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 1.0, color: None });
        nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(3), Node { label: Some("3".to_string()), value: 102.0, color: None });
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(1)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![NodeId(0)], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(3), Dependencies { inputs: vec![NodeId(1), NodeId(2)], outputs: vec![] });
        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &nodes, vec![NodeId(3)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();
        assert_eq!(final_root_layers, vec![vec![NodeId(0), NodeId(2)], vec![NodeId(1)], vec![NodeId(3)]]);
    }
    #[test]
    fn test_layers_fork_sort() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 50.0, color: None });
        nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 50.0, color: None });
        nodes.insert(NodeId(3), Node { label: Some("3".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(4), Node { label: Some("4".to_string()), value: 400.0, color: None });
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(4)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![], outputs: vec![NodeId(4)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![], outputs: vec![NodeId(4)] });
        deps.insert(NodeId(3), Dependencies { inputs: vec![], outputs: vec![NodeId(4)] });
        deps.insert(NodeId(4), Dependencies { inputs: vec![NodeId(0),NodeId(1),NodeId(2),NodeId(3)], outputs: vec![] });
        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &nodes, vec![NodeId(4)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();
        assert_eq!(final_root_layers, vec![vec![NodeId(1), NodeId(2), NodeId(0), NodeId(3)], vec![NodeId(4)]]);
    }

    #[test]
    fn test_layers_double_chain_fork() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 2.0, color: None });
        nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 2.0, color: None });
        nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 3.0, color: None });
        nodes.insert(NodeId(3), Node { label: Some("3".to_string()), value: 3.0, color: None });
        nodes.insert(NodeId(4), Node { label: Some("4".to_string()), value: 5.0, color: None });
        nodes.insert(NodeId(5), Node { label: Some("5".to_string()), value: 5.0, color: None });
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(1)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![NodeId(0)], outputs: vec![NodeId(5)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![], outputs: vec![NodeId(3)] });
        deps.insert(NodeId(3), Dependencies { inputs: vec![NodeId(2)], outputs: vec![NodeId(5)] });
        deps.insert(NodeId(5), Dependencies { inputs: vec![NodeId(1), NodeId(3)], outputs: vec![] });
        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &nodes, vec![NodeId(5)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();
        assert_eq!(final_root_layers, vec![vec![NodeId(0), NodeId(2)], vec![NodeId(1), NodeId(3)], vec![NodeId(5)]]);
    }
    #[test]
    fn test_layers_nested_fork_sort() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 50.0, color: None });
        nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 150.0, color: None });
        nodes.insert(NodeId(3), Node { label: Some("3".to_string()), value: 50.0, color: None });
        nodes.insert(NodeId(4), Node { label: Some("4".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(5), Node { label: Some("5".to_string()), value: 150.0, color: None });
        nodes.insert(NodeId(6), Node { label: Some("6".to_string()), value: 600.0, color: None });
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(2)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![], outputs: vec![NodeId(2)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![NodeId(0), NodeId(1)], outputs: vec![NodeId(6)] });
        deps.insert(NodeId(3), Dependencies { inputs: vec![], outputs: vec![NodeId(5)] });
        deps.insert(NodeId(4), Dependencies { inputs: vec![], outputs: vec![NodeId(5)] });
        deps.insert(NodeId(5), Dependencies { inputs: vec![NodeId(3), NodeId(4)], outputs: vec![NodeId(6)] });
        deps.insert(NodeId(6), Dependencies { inputs: vec![NodeId(2),NodeId(5)], outputs: vec![] });
        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &nodes, vec![NodeId(6)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();
        assert_eq!(final_root_layers, vec![vec![NodeId(1), NodeId(0), NodeId(3), NodeId(4)], vec![NodeId(2), NodeId(5)], vec![NodeId(6)]]);
    }
    #[test]
    fn test_layers_nested_fork_sort_clone_different_weights() {
        let mut nodes: HashMap<NodeId, Node> = HashMap::new();
        nodes.insert(NodeId(0), Node { label: Some("0".to_string()), value: 100.1, color: None });
        nodes.insert(NodeId(1), Node { label: Some("1".to_string()), value: 50.0, color: None });
        nodes.insert(NodeId(2), Node { label: Some("2".to_string()), value: 150.1, color: None });
        nodes.insert(NodeId(3), Node { label: Some("3".to_string()), value: 50.0, color: None });
        nodes.insert(NodeId(4), Node { label: Some("4".to_string()), value: 100.0, color: None });
        nodes.insert(NodeId(5), Node { label: Some("5".to_string()), value: 150.0, color: None });
        nodes.insert(NodeId(6), Node { label: Some("6".to_string()), value: 600.1, color: None });
        let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
        deps.insert(NodeId(0), Dependencies { inputs: vec![], outputs: vec![NodeId(2)] });
        deps.insert(NodeId(1), Dependencies { inputs: vec![], outputs: vec![NodeId(2)] });
        deps.insert(NodeId(2), Dependencies { inputs: vec![NodeId(0), NodeId(1)], outputs: vec![NodeId(6)] });
        deps.insert(NodeId(3), Dependencies { inputs: vec![], outputs: vec![NodeId(5)] });
        deps.insert(NodeId(4), Dependencies { inputs: vec![], outputs: vec![NodeId(5)] });
        deps.insert(NodeId(5), Dependencies { inputs: vec![NodeId(3), NodeId(4)], outputs: vec![NodeId(6)] });
        deps.insert(NodeId(6), Dependencies { inputs: vec![NodeId(2),NodeId(5)], outputs: vec![] });
        let mut final_root_layers: Vec<Vec<NodeId>>;
        let mut final_leafs_layers: Vec<NodeId>;
        (final_root_layers, final_leafs_layers) = recursive_layers(&deps, &nodes, vec![NodeId(6)]);
        final_root_layers.extend(vec![final_leafs_layers]);
        final_root_layers.reverse();
        assert_eq!(final_root_layers, vec![vec![NodeId(3), NodeId(4), NodeId(1), NodeId(0)], vec![NodeId(5), NodeId(2)], vec![NodeId(6)]]);
    }

    #[test]
    fn test_merge_layers0() {
        let mut layers_a: Vec<Vec<NodeId>> = vec![vec![NodeId(1)], vec![NodeId(2)]];
        if layers_a.len() == 0 {
            layers_a.push(vec![NodeId(0)]);
        } else {
            layers_a[0].insert(0, NodeId(0));
        }
        //layers_a.iter_mut().for_each(|layer| layer.insert(0, NodeId(0)));
        let layers_b: Vec<Vec<NodeId>> = vec![vec![NodeId(8)]];
        let result = merge_layers(layers_a, layers_b);
        assert_eq!(result, vec![vec![NodeId(0), NodeId(1), NodeId(8)], vec![NodeId(2)]]);
    }
    #[test]
    fn test_merge_layers1() {
        let layers_a: Vec<Vec<NodeId>> = vec![vec![NodeId(0), NodeId(1), NodeId(3), NodeId(4)], vec![NodeId(5), NodeId(6)], vec![NodeId(7)]];
        let layers_b: Vec<Vec<NodeId>> = vec![vec![NodeId(8), NodeId(9), NodeId(10), NodeId(11)], vec![NodeId(12), NodeId(13)], vec![NodeId(13)]];
        let result = merge_layers(layers_a, layers_b);
        assert_eq!(result, vec![vec![NodeId(0), NodeId(1), NodeId(3), NodeId(4), NodeId(8), NodeId(9), NodeId(10), NodeId(11)], vec![NodeId(5), NodeId(6), NodeId(12), NodeId(13)], vec![NodeId(7), NodeId(13)]]);
    }
    #[test]
    fn test_merge_layers2() {
        let layers_a: Vec<Vec<NodeId>> = vec![vec![NodeId(0)]];
        let layers_b: Vec<Vec<NodeId>> = vec![vec![NodeId(8), NodeId(9), NodeId(10), NodeId(11)], vec![NodeId(12), NodeId(13)], vec![NodeId(13)]];
        let result = merge_layers(layers_a, layers_b);
        assert_eq!(result, vec![vec![NodeId(0), NodeId(8), NodeId(9), NodeId(10), NodeId(11)], vec![NodeId(12), NodeId(13)], vec![NodeId(13)]]);
    }
    #[test]
    fn test_merge_layers3() {
        let layers_a: Vec<Vec<NodeId>> = vec![vec![NodeId(8), NodeId(9), NodeId(10), NodeId(11)], vec![NodeId(12), NodeId(13)], vec![NodeId(13)]];
        let layers_b: Vec<Vec<NodeId>> = vec![vec![NodeId(0)]];
        let result = merge_layers(layers_a, layers_b);
        assert_eq!(result, vec![vec![NodeId(8), NodeId(9), NodeId(10), NodeId(11), NodeId(0)], vec![NodeId(12), NodeId(13)], vec![NodeId(13)]]);
    }
}