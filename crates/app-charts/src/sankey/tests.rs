use std::{cmp::Ordering, collections::HashMap};

use crate::sankey::{Dependencies, Node, NodeId, *};

#[test]
fn test_sort() {
    let nodes = [(0, 100.0), (1, 10.0), (2, 1111.0), (3, 1111.0)]
        .into_iter()
        .map(|(id, value)| {
            (
                NodeId(id),
                Node {
                    label: Some(id.to_string()),
                    value,
                    color: None,
                    edge_color: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    let mut layers = vec![NodeId(0), NodeId(1), NodeId(2), NodeId(3)];
    layers.sort_by(|a, b| {
        nodes[a]
            .value
            .partial_cmp(&nodes[b].value)
            .unwrap_or(Ordering::Equal)
    });
    assert_eq!(layers, vec![NodeId(1), NodeId(0), NodeId(2), NodeId(3)]);
}

#[test]
fn test_layers_empty() {
    let deps = HashMap::new();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &HashMap::new(), vec![]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();

    let empty = vec![vec![]];
    assert_eq!(final_root_layers, empty);
}

#[test]
fn test_layers_basic() {
    let deps = [
        (0, vec![], vec![1]),
        (1, vec![0], vec![2]),
        (2, vec![1], vec![]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &HashMap::new(), vec![NodeId(2)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();

    assert_eq!(
        final_root_layers,
        vec![vec![NodeId(0)], vec![NodeId(1)], vec![NodeId(2)]]
    );
}

#[test]
fn test_layers_sort() {
    let nodes = [(0, 100.0), (1, 10.0), (2, 1.0), (3, 111.0)]
        .into_iter()
        .map(|(id, value)| {
            (
                NodeId(id),
                Node {
                    label: Some(id.to_string()),
                    value,
                    color: None,
                    edge_color: None,
                },
            )
        })
        .collect();

    let deps = [
        (0, vec![], vec![3]),
        (1, vec![], vec![3]),
        (2, vec![], vec![3]),
        (3, vec![0, 1, 2], vec![]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &nodes, vec![NodeId(3)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    assert_eq!(
        final_root_layers,
        vec![vec![NodeId(0), NodeId(1), NodeId(2)], vec![NodeId(3)]]
    );
}

#[test]
fn test_layers_sort_clone_different_weights() {
    let nodes = [(0, 1.0), (1, 10.0), (2, 100.0), (3, 111.0)]
        .into_iter()
        .map(|(id, value)| {
            (
                NodeId(id),
                Node {
                    label: Some(id.to_string()),
                    value,
                    color: None,
                    edge_color: None,
                },
            )
        })
        .collect();

    let deps = [
        (0, vec![], vec![3]),
        (1, vec![], vec![3]),
        (2, vec![], vec![3]),
        (3, vec![0, 1, 2], vec![]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &nodes, vec![NodeId(3)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    assert_eq!(
        final_root_layers,
        vec![vec![NodeId(2), NodeId(1), NodeId(0)], vec![NodeId(3)]]
    );
}

#[test]
fn test_layers_long_sort() {
    let nodes = [(0, 100.0), (1, 100.0), (2, 1.0), (3, 201.0)]
        .into_iter()
        .map(|(id, value)| {
            (
                NodeId(id),
                Node {
                    label: Some(id.to_string()),
                    value,
                    color: None,
                    edge_color: None,
                },
            )
        })
        .collect();

    let deps = [
        (0, vec![], vec![1]),
        (1, vec![0], vec![3]),
        (2, vec![], vec![3]),
        (3, vec![1, 2], vec![]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &nodes, vec![NodeId(3)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    assert_eq!(
        final_root_layers,
        vec![vec![NodeId(0), NodeId(2)], vec![NodeId(1)], vec![NodeId(3)]]
    );
}

#[test]
fn test_layers_long_sort_clone_different_weights() {
    let nodes = [(0, 1.0), (1, 1.0), (2, 100.0), (3, 102.0)]
        .into_iter()
        .map(|(id, value)| {
            (
                NodeId(id),
                Node {
                    label: Some(id.to_string()),
                    value,
                    color: None,
                    edge_color: None,
                },
            )
        })
        .collect();

    let deps = [
        (0, vec![], vec![1]),
        (1, vec![0], vec![3]),
        (2, vec![], vec![3]),
        (3, vec![1, 2], vec![4]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &nodes, vec![NodeId(3)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    assert_eq!(
        final_root_layers,
        vec![vec![NodeId(2), NodeId(0)], vec![NodeId(1)], vec![NodeId(3)]]
    );
}

#[test]
fn test_layers_fork_sort() {
    let nodes = [(0, 100.0), (1, 50.0), (2, 50.0), (3, 100.0), (4, 400.0)]
        .into_iter()
        .map(|(id, value)| {
            (
                NodeId(id),
                Node {
                    label: Some(id.to_string()),
                    value,
                    color: None,
                    edge_color: None,
                },
            )
        })
        .collect();

    let deps = [
        (0, vec![], vec![4]),
        (1, vec![], vec![4]),
        (2, vec![], vec![4]),
        (3, vec![], vec![4]),
        (4, vec![0, 1, 2, 3], vec![]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &nodes, vec![NodeId(4)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    assert_eq!(
        final_root_layers,
        vec![
            vec![NodeId(0), NodeId(3), NodeId(1), NodeId(2)],
            vec![NodeId(4)]
        ]
    );
}

#[test]
fn test_layers_double_chain_fork() {
    let nodes = [(0, 2.0), (1, 2.0), (2, 3.0), (3, 3.0), (4, 5.0), (5, 5.0)]
        .into_iter()
        .map(|(id, value)| {
            (
                NodeId(id),
                Node {
                    label: Some(id.to_string()),
                    value,
                    color: None,
                    edge_color: None,
                },
            )
        })
        .collect();

    let deps = [
        (0, vec![], vec![1]),
        (1, vec![0], vec![5]),
        (2, vec![], vec![3]),
        (3, vec![2], vec![5]),
        (5, vec![1, 3], vec![]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &nodes, vec![NodeId(5)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    assert_eq!(
        final_root_layers,
        vec![
            vec![NodeId(2), NodeId(0)],
            vec![NodeId(3), NodeId(1)],
            vec![NodeId(5)]
        ]
    );
}

#[test]
fn test_layers_nested_fork_sort() {
    let nodes = [
        (0, 100.0),
        (1, 50.0),
        (2, 150.0),
        (3, 100.0),
        (4, 150.0),
        (5, 600.0),
    ]
    .into_iter()
    .map(|(id, value)| {
        (
            NodeId(id),
            Node {
                label: Some(id.to_string()),
                value,
                color: None,
                edge_color: None,
            },
        )
    })
    .collect();

    let deps = [
        (0, vec![], vec![2]),
        (1, vec![], vec![2]),
        (2, vec![0, 1], vec![6]),
        (3, vec![], vec![5]),
        (4, vec![], vec![5]),
        (5, vec![3, 4], vec![6]),
        (6, vec![2, 5], vec![]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();

    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &nodes, vec![NodeId(6)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    assert_eq!(
        final_root_layers,
        vec![
            vec![NodeId(4), NodeId(3), NodeId(0), NodeId(1)],
            vec![NodeId(5), NodeId(2)],
            vec![NodeId(6)]
        ]
    );
}

#[test]
fn test_layers_nested_fork_sort_clone_different_weights() {
    let nodes = [
        (0, 100.1),
        (1, 50.0),
        (2, 150.1),
        (3, 50.0),
        (4, 100.0),
        (5, 150.0),
        (6, 600.1),
    ]
    .into_iter()
    .map(|(id, value)| {
        (
            NodeId(id),
            Node {
                label: Some(id.to_string()),
                value,
                color: None,
                edge_color: None,
            },
        )
    })
    .collect();

    let deps = [
        (0, vec![], vec![2]),
        (1, vec![], vec![2]),
        (2, vec![0, 1], vec![6]),
        (3, vec![], vec![5]),
        (4, vec![], vec![5]),
        (5, vec![3, 4], vec![6]),
        (6, vec![2, 5], vec![]),
    ]
    .into_iter()
    .map(|(id, inputs, outputs)| {
        let inputs = inputs.into_iter().map(NodeId::new).collect();
        let outputs = outputs.into_iter().map(NodeId::new).collect();
        (NodeId::new(id), Dependencies { inputs, outputs })
    })
    .collect();
    let (mut final_root_layers, final_leafs_layers) =
        recursive_layers(&deps, &nodes, vec![NodeId(6)]);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    assert_eq!(
        final_root_layers,
        vec![
            vec![NodeId(0), NodeId(1), NodeId(4), NodeId(3)],
            vec![NodeId(2), NodeId(5)],
            vec![NodeId(6)]
        ]
    );
}

#[test]
fn test_merge_layers0() {
    let mut layers_a: Vec<Vec<NodeId>> = vec![vec![NodeId(1)], vec![NodeId(2)]];
    if layers_a.is_empty() {
        layers_a.push(vec![NodeId(0)]);
    } else {
        layers_a[0].insert(0, NodeId(0));
    }
    let layers_b: Vec<Vec<NodeId>> = vec![vec![NodeId(8)]];
    let result = merge_layers(layers_a, layers_b);
    assert_eq!(
        result,
        vec![vec![NodeId(0), NodeId(1), NodeId(8)], vec![NodeId(2)]]
    );
}

#[test]
fn test_merge_layers1() {
    let layers_a: Vec<Vec<NodeId>> = vec![
        vec![NodeId(0), NodeId(1), NodeId(3), NodeId(4)],
        vec![NodeId(5), NodeId(6)],
        vec![NodeId(7)],
    ];
    let layers_b: Vec<Vec<NodeId>> = vec![
        vec![NodeId(8), NodeId(9), NodeId(10), NodeId(11)],
        vec![NodeId(12), NodeId(13)],
        vec![NodeId(13)],
    ];
    let result = merge_layers(layers_a, layers_b);
    assert_eq!(
        result,
        vec![
            vec![
                NodeId(0),
                NodeId(1),
                NodeId(3),
                NodeId(4),
                NodeId(8),
                NodeId(9),
                NodeId(10),
                NodeId(11)
            ],
            vec![NodeId(5), NodeId(6), NodeId(12), NodeId(13)],
            vec![NodeId(7), NodeId(13)]
        ]
    );
}

#[test]
fn test_merge_layers2() {
    let layers_a: Vec<Vec<NodeId>> = vec![vec![NodeId(0)]];
    let layers_b: Vec<Vec<NodeId>> = vec![
        vec![NodeId(8), NodeId(9), NodeId(10), NodeId(11)],
        vec![NodeId(12), NodeId(13)],
        vec![NodeId(13)],
    ];
    let result = merge_layers(layers_a, layers_b);
    assert_eq!(
        result,
        vec![
            vec![NodeId(0), NodeId(8), NodeId(9), NodeId(10), NodeId(11)],
            vec![NodeId(12), NodeId(13)],
            vec![NodeId(13)]
        ]
    );
}

#[test]
fn test_merge_layers3() {
    let layers_a: Vec<Vec<NodeId>> = vec![
        vec![NodeId(8), NodeId(9), NodeId(10), NodeId(11)],
        vec![NodeId(12), NodeId(13)],
        vec![NodeId(13)],
    ];
    let layers_b: Vec<Vec<NodeId>> = vec![vec![NodeId(0)]];
    let result = merge_layers(layers_a, layers_b);
    assert_eq!(
        result,
        vec![
            vec![NodeId(8), NodeId(9), NodeId(10), NodeId(11), NodeId(0)],
            vec![NodeId(12), NodeId(13)],
            vec![NodeId(13)]
        ]
    );
}
