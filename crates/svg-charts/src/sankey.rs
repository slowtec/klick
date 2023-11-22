use std::collections::HashMap;

use leptos::*;

#[derive(Debug, Default, Clone)]
pub struct Sankey {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
struct Node {
    value: f64,
    label: Option<String>,
}

impl Node {
    fn new(value: f64, label: Option<String>) -> Self {
        Self { value, label }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(usize);

#[derive(Debug, Clone, Copy)]
struct Edge {
    source: NodeId,
    target: NodeId,
}

impl Sankey {
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn node<S>(&mut self, value: f64, label: Option<S>) -> NodeId
    where
        S: Into<String>,
    {
        let id = self.nodes.len();
        self.nodes.push(Node::new(value, label.map(Into::into)));
        NodeId(id)
    }

    pub fn edge(&mut self, source: NodeId, target: NodeId) {
        self.edges.push(Edge { source, target });
    }
}

#[component]
pub fn Chart(sankey: Sankey, width: f64, height: f64) -> impl IntoView {
    let node_separation = height / 30.0;
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
    let edge_positions = edge_positions(
        &sankey.edges,
        &sankey.nodes,
        scale,
        &node_positions,
        node_width,
    );

    let nodes = node_positions
        .iter()
        .map(|(id, (x, y, node_height))| {

           let font_size = height / 40.0;
            let value = sankey.nodes[id.0].value;
            let label = sankey.nodes[id.0].label.as_ref().map(|label|
              view! {
                <tspan>
                  { label }
                </tspan>
              }
            );

            let value_dx = if label.is_some(){ font_size/4.0 } else { 0.0 };

            view! {
              <rect
                x = {*x}
                y = {*y}
                width = { node_width }
                height = { *node_height }
                fill = "#555"
              />
              <text 
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
                  { format!("{value:.1}") }
                </tspan>
              </text>
            }
        })
        .collect::<Vec<_>>();

    let edges = edge_positions.iter().map(|(from_top,from_bottom,to_top,to_bottom)|{

      let from_top_x = from_top.x;
      let from_top_y = from_top.y;

      let from_bottom_x = from_bottom.x;
      let from_bottom_y = from_bottom.y;

      let to_top_x = to_top.x;
      let to_top_y = to_top.y;

      let to_bottom_x = to_bottom.x;
      let to_bottom_y = to_bottom.y;

      let mid_x = (from_top_x + to_top_x) / 2.0;

      let d = format!("M {from_top_x:.3} {from_top_y:.3} C {mid_x:.3} {from_top_y:.3}, {mid_x:.3} {to_top_y:.3}, {to_top_x:.3} {to_top_y:.3} L {to_bottom_x:.3} {to_bottom_y:.3} C {mid_x:.3} {to_bottom_y:.3}, {mid_x:.3} {from_bottom_y:.3}, {from_bottom_x:.3} {from_bottom_y:.3}  Z");
      view! {
        <path
          d = {d}
          fill = "rgba(15,15,15,0.5)"
        />
      }
    }).collect::<Vec<_>>();

    view! {
      <svg viewBox = format!("0.0 0.0 {width} {height}") >
        { edges }
        { nodes }
      </svg>
    }
}

#[derive(Debug, Default)]
struct Dependencies {
    inputs: Vec<NodeId>,
    outputs: Vec<NodeId>,
}

fn dependencies(edges: &[Edge]) -> HashMap<NodeId, Dependencies> {
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
        .filter_map(|node| deps.get(node).map(|d| d.inputs.to_vec()))
        .flatten()
        .collect()
}

fn layer_x_positions(layer_count: usize, width: f64, node_width: f64) -> Vec<f64> {
    let dx = (width - node_width) / (layer_count as f64);
    (0..layer_count)
        .into_iter()
        .map(|i| i as f64 * dx + node_width / 2.0)
        .collect()
}

fn node_positions(
    layers: &[Vec<NodeId>],
    layer_positions: &[f64],
    nodes: &[Node],
    deps: &HashMap<NodeId, Dependencies>,
    scale: f64,
    gap: f64,
) -> HashMap<NodeId, (f64, f64, f64)> {
    let mut positions = layers
        .iter()
        .enumerate()
        .flat_map(|(i, layer)| {

            let x = layer_positions[i];
            let mut y = 0.0;
            layer
                .iter()
                .map(|id| {
                    let gap_count = gap_count(deps, id);

                    let node_y = y;

                    let v = if deps[id].inputs.is_empty() {
                        nodes[id.0].value
                    } else {
                        input_value(deps, nodes, id)
                    };

                    let node_height = v * scale + gap_count as f64 * gap;

                    y += node_height + gap;
                    (*id, (x, node_y, node_height))
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<_,_>>();

  for layer in layers.iter().rev() {
    for id in layer {

      if deps[id].inputs.is_empty() {
        continue;
      };
    
      let min_y = deps[id].inputs.iter().map(|id| positions[id].1)
          .fold(f64::INFINITY, |min, val| if val < min { val } else { min });

      let self_y = positions[id].1;

      if self_y < min_y {
        positions.get_mut(id).unwrap().1 = min_y;
      }

    }
  }
  positions
}

fn gap_count(deps: &HashMap<NodeId, Dependencies>, id: &NodeId) -> usize {
    let input_count = deps[id].inputs.len();

    let gaps = if input_count > 0 { input_count - 1 } else { 0 };

    gaps + deps[id]
        .inputs
        .iter()
        .map(|id| gap_count(deps, id))
        .fold(0, |x, v| x + v)
}

fn input_value(deps: &HashMap<NodeId, Dependencies>, nodes: &[Node], id: &NodeId) -> f64 {
    deps[id]
        .inputs
        .iter()
        .map(|id| nodes[id.0].value)
        .fold(0.0, |x, v| x + v)
}

fn scale(layers: &[Vec<NodeId>], nodes: &[Node], height: f64, node_separation: f64) -> f64 {
    layers
        .iter()
        .map(|layer| {
            let total_value = layer.iter().fold(0.0, |x, id| x + nodes[id.0].value);
            let count = layer.len() as f64;
            let scale = (height - node_separation * count) / total_value;
            scale
        })
        .fold(f64::INFINITY, |min, val| if val < min { val } else { min })
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

fn edge_positions(
    edges: &[Edge],
    nodes: &[Node],
    _scale: f64,
    node_positions: &HashMap<NodeId, (f64, f64, f64)>,
    node_width: f64,
) -> Vec<(Point, Point, Point, Point)> {
    let mut total_input_values = vec![0.0; nodes.len()];
    let mut relative_heights = vec![0.0; nodes.len()];

    for Edge { source, target } in edges {
        total_input_values[target.0] += nodes[source.0].value;
    }

    edges
        .iter()
        .map(|edge| {
            let (x_from, y_from, height_from) = node_positions[&edge.source];
            let (x_to, y_to, height_to) = node_positions[&edge.target];

            let scale = height_to / total_input_values[edge.target.0];
            let to_y_start = relative_heights[edge.target.0] + y_to;
            let to_y_end =
                relative_heights[edge.target.0] + y_to + nodes[edge.source.0].value * scale;

            let points = (
                Point::new(x_from + node_width, y_from),
                Point::new(x_from + node_width, y_from + height_from),
                Point::new(x_to, to_y_start),
                Point::new(x_to, to_y_end),
            );

            relative_heights[edge.target.0] += nodes[edge.source.0].value * scale;

            points
        })
        .collect()
}
