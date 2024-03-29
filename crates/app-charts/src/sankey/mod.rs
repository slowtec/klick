use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use leptos::*;

#[cfg(test)]
mod tests;

#[derive(Debug, Default, Clone)]
pub struct Sankey {
    nodes: HashMap<NodeId, Node>,
    edges: HashSet<Edge>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub value: f64,
    pub label: Option<String>,
    pub color: Option<Color>,
}

impl Node {
    const fn new(value: f64, label: Option<String>, color: Option<Color>) -> Self {
        Self {
            value,
            label,
            color,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
    pub height: f64,
}

impl NodePosition {
    const fn new(x: f64, y: f64, height: f64) -> Self {
        Self { x, y, height }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    const fn new(id: usize) -> Self {
        Self(id)
    }
}

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

    pub fn insert_node<S>(&mut self, value: f64, label: S, color: Option<Color>) -> NodeId
    where
        S: Into<String>,
    {
        let id = self.nodes.len();
        let id = NodeId::new(id);
        let label = Some(label.into());
        let node = Node::new(value, label, color);
        self.nodes.insert(id, node);
        id
    }

    pub fn insert_edge(&mut self, source: NodeId, target: NodeId) {
        self.edges.insert(Edge { source, target });
    }

    #[must_use]
    pub fn node_value(&self, id: &NodeId) -> Option<f64> {
        self.nodes.get(id).map(|n| n.value)
    }
}

#[component]
pub fn Chart<F>(
    sankey: Sankey,
    width: f64,
    height: f64,
    number_format: F,
    font_size: f64,
) -> impl IntoView
where
    F: Fn(f64) -> String,
{
    let margin_x = width * 0.08;
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
            font_size = font_size
          />
        </g>
      </svg>
    }
}

#[component]
fn InnerChart<F>(
    sankey: Sankey,
    width: f64,
    height: f64,
    number_format: F,
    font_size: f64,
) -> impl IntoView
where
    F: Fn(f64) -> String,
{
    let node_separation = height / 50.0;
    let node_width = width / 70.0; // TODO: make this configurable
    let deps = dependencies(&sankey.edges);
    let layers = layers(&deps, &sankey.nodes);
    let layer_x_positions = layer_x_positions(layers.len(), width, node_width);
    let scale = scale(&layers, &sankey.nodes, height, node_separation);
    let node_positions = node_positions(
        &layers,
        &layer_x_positions,
        &sankey.nodes,
        &deps,
        scale,
        node_separation,
    );
    let edge_positions = edge_positions(
        &sankey.edges,
        &sankey.nodes,
        &deps,
        &node_positions,
        &layers,
        node_width,
    );

    let nodes_to_svg = node_positions
        .iter()
        .map(|(id, node_position)| {
            let x = node_position.x;
            let y = node_position.y;
            let node_height = node_position.height;
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

            let fill = sankey.nodes[id].color.unwrap_or(Color::new("#555"));

            view! {
              <rect
                x = {x}
                y = {y}
                width = { node_width }
                height = { node_height }
                fill = { fill.as_str() }
              />
              <text
                class = "label"
                x = {x + node_width + font_size/2.0 }
                y = {y + node_height /2.0 }
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

    let edges_to_svg = edge_positions
        .iter()
        .map(|(from_top, from_bottom, to_top, to_bottom, color)| {
            let d = edge_path(from_top, from_bottom, to_top, to_bottom);

            // TODO: use gradient
            let fill = color.unwrap_or(Color::new("#555"));

            view! {
              <path
                d = {d}
                fill = { fill.as_str() }
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
      { edges_to_svg }
      { nodes_to_svg }
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
pub struct Dependencies {
    pub inputs: Vec<NodeId>,
    pub outputs: Vec<NodeId>,
}

fn dependencies(edges: &HashSet<Edge>) -> HashMap<NodeId, Dependencies> {
    let mut deps: HashMap<NodeId, Dependencies> = HashMap::new();
    for Edge { source, target } in edges {
        deps.entry(*source).or_default().outputs.push(*target);
        deps.entry(*target).or_default().inputs.push(*source);
    }
    deps
}

pub fn merge_layers(mut layers: Vec<Vec<NodeId>>, layers_b: Vec<Vec<NodeId>>) -> Vec<Vec<NodeId>> {
    for (i, layer) in layers_b.into_iter().enumerate() {
        if layers.len() <= i {
            layers.push(layer);
        } else {
            layers[i].extend(layer);
        }
    }
    layers
}

// layers returns a list of n layers, where layers[0] is on the right side of the sankey and
// layers[n] on the left side. nodes without inputs are on the left side.
// NOTE the reversed order before return!
pub fn layers(
    deps: &HashMap<NodeId, Dependencies>,
    nodes: &HashMap<NodeId, Node>,
) -> Vec<Vec<NodeId>> {
    let root_layer = deps
        .iter()
        .filter_map(|(node, Dependencies { outputs, .. })| {
            if outputs.is_empty() {
                Some(*node)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let (mut final_root_layers, final_leafs_layers) = recursive_layers(deps, nodes, root_layer);
    final_root_layers.extend(vec![final_leafs_layers]);
    final_root_layers.reverse();
    final_root_layers
}

pub fn recursive_layers(
    deps: &HashMap<NodeId, Dependencies>,
    nodes: &HashMap<NodeId, Node>,
    mut current_layer: Vec<NodeId>,
) -> (Vec<Vec<NodeId>>, Vec<NodeId>) {
    let mut roots = vec![];
    let mut leafs = vec![];

    current_layer.sort_by(|a, b| {
        return nodes[b]
            .value
            .partial_cmp(&nodes[a].value)
            .unwrap_or(Ordering::Equal);
    });
    let mut t_roots = vec![];
    for el in current_layer {
        if deps[&el].inputs.is_empty() {
            leafs.push(el);
            continue;
        }
        let Some(dependency) = deps.get(&el) else {
            continue;
        };
        let next_layer_nodes = dependency.inputs.clone();
        let (mut return_roots, return_leafs) = recursive_layers(deps, nodes, next_layer_nodes);
        if return_roots.is_empty() {
            return_roots.push(vec![el]);
        } else {
            return_roots.insert(0, vec![el]);
        }
        t_roots = merge_layers(t_roots, return_roots);
        leafs.extend(return_leafs);
    }
    roots = merge_layers(roots, t_roots);
    (roots, leafs)
}

fn layer_x_positions(layer_count: usize, width: f64, node_width: f64) -> Vec<f64> {
    let dx = (width - node_width) / ((layer_count - 1) as f64);
    (0..layer_count)
        .map(|i| i as f64 * dx + node_width / 2.0)
        .collect()
}

// compute node_positions: height, x position and y positions
fn node_positions(
    layers: &[Vec<NodeId>],
    layer_positions: &[f64],
    nodes: &HashMap<NodeId, Node>,
    deps: &HashMap<NodeId, Dependencies>,
    scale: f64,
    gap: f64,
) -> HashMap<NodeId, NodePosition> {
    let mut node_positions: HashMap<NodeId, NodePosition> = HashMap::new();
    for (j, layer) in layers.iter().enumerate() {
        let x = layer_positions[j];
        let mut layer_y = 0.0;
        for id in layer {
            let y = if j == 0 {
                layer_y
            } else {
                deps[&id].inputs.iter().fold(f64::INFINITY, |y, successor| {
                    y.min(node_positions[&successor].y)
                })
            };
            let height = nodes[id].value * scale;
            let position = NodePosition::new(x, y, height);
            node_positions.insert(*id, position);
            layer_y += height + gap;
        }
    }
    node_positions
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

#[derive(Debug, Copy, Clone)]
pub struct Color(&'static str);

impl Color {
    #[must_use]
    pub const fn new(c: &'static str) -> Self {
        Self(c)
    }
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}

fn edge_positions(
    edges: &HashSet<Edge>,
    nodes: &HashMap<NodeId, Node>,
    deps: &HashMap<NodeId, Dependencies>,
    node_positions: &HashMap<NodeId, NodePosition>,
    layers: &[Vec<NodeId>],
    node_width: f64,
) -> Vec<(Point, Point, Point, Point, Option<Color>)> {
    let mut total_input_values = HashMap::<NodeId, f64>::new();
    for Edge { source, target } in edges {
        *total_input_values.entry(*target).or_default() += nodes[&source].value;
    }
    layers
        .iter()
        .flat_map(|layer| {
            layer
                .iter()
                .filter_map(|node| {
                    let from = node_positions[node];
                    let Some(edge) = edges.iter().find(|edge| edge.source == *node) else {
                        return None;
                    };
                    let to = node_positions[&edge.target];

                    let scale = to.height / total_input_values[&edge.target];
                    let mut to_y_start = node_positions.get(&edge.target).unwrap().y;
                    if (from.y - to.y).abs() > 1.0 {
                        let prev_nodes = deps[&edge.target]
                            .inputs
                            .iter()
                            .filter(|id| from.y - node_positions[&id].y > 1.0)
                            .collect::<Vec<_>>();
                        to_y_start += prev_nodes
                            .iter()
                            .fold(0.0, |acc, id| acc + node_positions[&id].height);
                    }
                    let to_y_end = to_y_start + nodes[&edge.source].value * scale;
                    let color = nodes[&edge.source].color;
                    let points = (
                        Point::new(from.x + node_width, from.y),
                        Point::new(from.x + node_width, from.y + from.height),
                        Point::new(to.x, to_y_start),
                        Point::new(to.x, to_y_end),
                        color,
                    );
                    Some(points)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
