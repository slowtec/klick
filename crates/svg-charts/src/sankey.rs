use std::collections::{HashMap, HashSet};

use leptos::*;

#[derive(Debug, Default, Clone)]
pub struct Sankey {
    nodes: HashMap<NodeId, Node>,
    edges: HashSet<Edge>,
}

#[derive(Debug, Clone)]
struct Node {
    value: f64,
    label: Option<String>,
    color: Option<String>,
}

impl Node {
    fn new(value: f64, label: Option<String>, color: Option<String>) -> Self {
        Self {
            value,
            label,
            color,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    source: NodeId,
    target: NodeId,
}

impl Sankey {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    pub fn node<S>(&mut self, value: f64, label: S, color: Option<S>) -> NodeId
    where
        S: Into<String>,
    {
        let id = self.nodes.len();
        let id = NodeId(id);
        let label = Some(label.into());
        let node = Node::new(value, label, color.map(Into::into));
        self.nodes.insert(id, node);
        id
    }

    pub fn edge(&mut self, source: NodeId, target: NodeId) {
        self.edges.insert(Edge { source, target });
    }

    pub fn node_value(&self, id: &NodeId) -> Option<f64> {
        self.nodes.get(id).map(|n| n.value)
    }
}

#[component]
pub fn Chart<F>(sankey: Sankey, width: f64, height: f64, number_format: F) -> impl IntoView
where
    F: Fn(f64) -> String,
{
    let margin_x = width * 0.05;
    let margin_y = height * 0.05;

    view! {
      <svg
        width = format!("{width}px")
        height = format!("{height}px")
        viewBox = format!("0.0 0.0 {width} {height}")
      >
        <g
          transform = format!("translate(0.0,{margin_y})")
        >
          <InnerChart sankey
            width = { width - margin_x * 2.0 }
            height = { height - margin_y * 2.0 }
            number_format
          />
        </g>
      </svg>
    }
}

#[component]
fn InnerChart<F>(sankey: Sankey, width: f64, height: f64, number_format: F) -> impl IntoView
where
    F: Fn(f64) -> String,
{
    let node_separation = height / 50.0;
    let node_width = width / 100.0;

    let deps = dependencies(&sankey.edges);
    let layers = layers(&deps);
    let layer_positions = layer_x_positions(layers.len(), width, node_width);
    let scale = scale(&layers, &sankey.nodes, height, node_separation);
    let node_positions = node_positions(
        &layers,
        &layer_positions,
        &sankey.nodes,
        &deps,
        scale,
        node_separation,
    );
    let edge_positions = edge_positions(&sankey.edges, &sankey.nodes, &node_positions, node_width);

    let nodes = node_positions
        .iter()
        .map(|(id, (x, y, node_height))| {
            let font_size = height / 40.0;
            let value = sankey.nodes[id].value;
            let label = sankey.nodes[id].label.as_ref().map(|label| {
                view! {
                  <tspan>
                    { label }
                  </tspan>
                }
            });

            let value_dx = if label.is_some() {
                font_size / 4.0
            } else {
                0.0
            };

            let fill = sankey.nodes[id]
                .color
                .clone()
                .unwrap_or_else(|| "#555".to_string());

            view! {
              <rect
                x = {*x}
                y = {*y}
                width = { node_width }
                height = { *node_height }
                fill = { fill }
              />
              <text
                class = "label"
                x = {*x + node_width + font_size/2.0 }
                y = {*y + node_height /2.0 }
                fill = "#111"
                text-anchor = "start"
                font-family = "sans-serif"
                font-size = { font_size }
                dominant-baseline = "middle"
              >
                { label }
                <tspan dx= {value_dx}>
                  { number_format(value) }
                </tspan>
              </text>
            }
        })
        .collect::<Vec<_>>();

    let edges = edge_positions
        .iter()
        .map(|(from_top, from_bottom, to_top, to_bottom, color)| {
            let d = edge_path(from_top, from_bottom, to_top, to_bottom);

            // TODO: use gradient
            let fill = color
                .as_ref()
                .map(|c| c.0.clone())
                .unwrap_or_else(|| "#555".to_string());

            view! {
              <path
                d = {d}
                fill = { fill }
                opacity = 0.3
              />
            }
        })
        .collect::<Vec<_>>();

    view! {
      <style>
        "text.label {
          cursor: pointer;
        }
        text.label:hover {
          font-weight: bold;
        }"
      </style>
      { edges }
      { nodes }
    }
}

fn edge_path(from_top: &Point, from_bottom: &Point, to_top: &Point, to_bottom: &Point) -> String {
    let from_top_x = from_top.x;
    let from_top_y = from_top.y;

    let from_bottom_x = from_bottom.x;
    let from_bottom_y = from_bottom.y;

    let to_top_x = to_top.x;
    let to_top_y = to_top.y;

    let to_bottom_x = to_bottom.x;
    let to_bottom_y = to_bottom.y;

    let mid_x = (from_top_x + to_top_x) / 2.0;

    format!("M {from_top_x:.3} {from_top_y:.3} C {mid_x:.3} {from_top_y:.3}, {mid_x:.3} {to_top_y:.3}, {to_top_x:.3} {to_top_y:.3} L {to_bottom_x:.3} {to_bottom_y:.3} C {mid_x:.3} {to_bottom_y:.3}, {mid_x:.3} {from_bottom_y:.3}, {from_bottom_x:.3} {from_bottom_y:.3}  Z")
}

#[derive(Debug, Default)]
struct Dependencies {
    inputs: Vec<NodeId>,
    outputs: Vec<NodeId>,
}

fn dependencies(edges: &HashSet<Edge>) -> HashMap<NodeId, Dependencies> {
    let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();

    for Edge { source, target } in edges {
        deps.entry(*source).or_default().outputs.push(*target);
        deps.entry(*target).or_default().inputs.push(*source);
    }
    deps
}

fn layers(deps: &HashMap<NodeId, Dependencies>) -> Vec<Vec<NodeId>> {
    let output_layer = deps
        .iter()
        .filter_map(|(node, Dependencies { outputs, .. })| {
            if outputs.is_empty() {
                Some(*node)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let mut layers = recursive_layers(deps, output_layer, vec![]);

    // Remove the last layer as we'll redefine it.
    if layers.pop().is_none() {
        // The vector is empty
        return layers;
    };

    // The first input layer should contain all nodes that have no inputs.
    let first_input_layer = deps
        .iter()
        .filter_map(
            |(node, Dependencies { inputs, .. })| {
                if inputs.is_empty() {
                    Some(*node)
                } else {
                    None
                }
            },
        )
        .collect::<Vec<_>>();

    // Remove all node IDs that belong to the first input layer from the layers in between.
    layers
        .iter_mut()
        .for_each(|layer| layer.retain(|id| !first_input_layer.contains(id)));

    // Append the new defined layer.
    layers.push(first_input_layer);
    layers.reverse();
    layers
}

fn recursive_layers(
    deps: &HashMap<NodeId, Dependencies>,
    current_layer: Vec<NodeId>,
    mut layers: Vec<Vec<NodeId>>,
) -> Vec<Vec<NodeId>> {
    if current_layer.is_empty() {
        return layers;
    }
    let next_layer = layer_inputs(deps, &current_layer);
    layers.push(current_layer);
    recursive_layers(deps, next_layer, layers)
}

fn layer_inputs(deps: &HashMap<NodeId, Dependencies>, layer: &[NodeId]) -> Vec<NodeId> {
    layer
        .iter()
        .filter_map(|node| deps.get(node).map(|d| d.inputs.clone()))
        .flatten()
        .collect()
}

fn layer_x_positions(layer_count: usize, width: f64, node_width: f64) -> Vec<f64> {
    let dx = (width - node_width) / ((layer_count - 1) as f64);
    (0..layer_count)
        .map(|i| i as f64 * dx + node_width / 2.0)
        .collect()
}

fn node_positions(
    layers: &[Vec<NodeId>],
    layer_positions: &[f64],
    nodes: &HashMap<NodeId, Node>,
    deps: &HashMap<NodeId, Dependencies>,
    scale: f64,
    gap: f64,
) -> HashMap<NodeId, (f64, f64, f64)> {
    let mut positions = layers
        .iter()
        .enumerate()
        .flat_map(|(i, layer)| {
            let x = layer_positions[i];
            layer
                .iter()
                .map(|id| {
                    let node_y = 0.0;

                    let v = nodes[id].value;

                    let node_height = v * scale;

                    (*id, (x, node_y, node_height))
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_, _>>();

    let mut layer_y = vec![0.0; layers.len()];
    let start_layer = &layers[layers.len() - 1];

    for id in start_layer {
        foo(
            &mut layer_y,
            id,
            layers.len() - 1,
            layers,
            &mut positions,
            deps,
            gap,
        );
    }

    for layer in layers.iter().rev() {
        for id in layer {
            if deps[id].inputs.is_empty() {
                continue;
            };

            let min_y = deps[id]
                .inputs
                .iter()
                .map(|id| positions[id].1)
                .fold(f64::INFINITY, |min, val| if val < min { val } else { min });

            let self_y = positions[id].1;

            if self_y < min_y {
                positions.get_mut(id).unwrap().1 = min_y;
            }
        }
    }
    positions
}

fn foo(
    layer_y: &mut [f64],
    id: &NodeId,
    n: usize,
    layers: &[Vec<NodeId>],
    pos: &mut HashMap<NodeId, (f64, f64, f64)>,
    deps: &HashMap<NodeId, Dependencies>,
    gap: f64,
) {
    pos.entry(*id).or_default().1 = layer_y[n];

    layer_y[n] += pos[id].2 + gap;

    let inputs = &deps[id].inputs;

    if inputs.is_empty() {
        return;
    }

    for id in inputs {
        let (n, _) = layers
            .iter()
            .enumerate()
            .find(|(_, layer)| layer.iter().any(|x| x == id))
            .unwrap();
        foo(layer_y, id, n, layers, pos, deps, gap);
    }
}

fn scale(
    layers: &[Vec<NodeId>],
    nodes: &HashMap<NodeId, Node>,
    height: f64,
    node_separation: f64,
) -> f64 {
    let (max_node_count, layer_val) = layers.iter().fold((usize::MIN, 0.0), |(cnt, val), layer| {
        if layer.len() > cnt {
            (
                layer.len(),
                layer.iter().fold(0.0, |x, id| x + nodes[id].value),
            )
        } else {
            (cnt, val)
        }
    });

    let max_gaps = if max_node_count > 0 {
        max_node_count - 1
    } else {
        0
    } as f64;

    (height - max_gaps * node_separation) / layer_val
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

struct Color(String);

fn edge_positions(
    edges: &HashSet<Edge>,
    nodes: &HashMap<NodeId, Node>,
    node_positions: &HashMap<NodeId, (f64, f64, f64)>,
    node_width: f64,
) -> Vec<(Point, Point, Point, Point, Option<Color>)> {
    let mut total_input_values = HashMap::<NodeId, f64>::new();
    let mut relative_heights = HashMap::<NodeId, f64>::new();

    for Edge { source, target } in edges {
        *total_input_values.entry(*target).or_default() += nodes[&source].value;
    }
    log::debug!("{total_input_values:?}");

    edges
        .iter()
        .map(|edge| {
            let (x_from, y_from, height_from) = node_positions[&edge.source];
            let (x_to, y_to, height_to) = node_positions[&edge.target];

            let scale = height_to / total_input_values[&edge.target];
            let to_y_start = relative_heights
                .get(&edge.target)
                .copied()
                .unwrap_or_default()
                + y_to;
            let to_y_end = relative_heights
                .get(&edge.target)
                .copied()
                .unwrap_or_default()
                + y_to
                + nodes[&edge.source].value * scale;

            // TODO: use gradient
            let color = nodes[&edge.source].color.clone().map(Color);

            let points = (
                Point::new(x_from + node_width, y_from),
                Point::new(x_from + node_width, y_from + height_from),
                Point::new(x_to, to_y_start),
                Point::new(x_to, to_y_end),
                color,
            );

            *relative_heights.entry(edge.target).or_default() += nodes[&edge.source].value * scale;

            points
        })
        .collect()
}
