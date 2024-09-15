use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[cfg(test)]
mod tests;

#[derive(Debug, Default, Clone)]
pub struct SankeyData {
    nodes: HashMap<NodeId, Node>,
    edges: HashSet<Edge>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub value: f64,
    pub label: Option<String>,
    pub color: Option<Color>,
    pub edge_color: Option<Color>,
}

impl Node {
    #[must_use]
    fn new(
        value: f64,
        label: Option<String>,
        color: Option<Color>,
        edge_color: Option<Color>,
    ) -> Self {
        let color = Some(color.unwrap_or(Color::new("#555")));
        let edge_color = Some(edge_color.unwrap_or(Color::new("grey")));
        Self {
            value,
            label,
            color,
            edge_color,
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
    #[must_use]
    const fn new(x: f64, y: f64, height: f64) -> Self {
        Self { x, y, height }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

impl NodeId {
    #[must_use]
    const fn new(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    source: NodeId,
    target: NodeId,
}

impl SankeyData {
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashSet::new(),
        }
    }

    pub fn insert_node<S>(
        &mut self,
        value: f64,
        label: S,
        color: Option<Color>,
        edge_color: Option<Color>,
    ) -> NodeId
    where
        S: Into<String>,
    {
        let id = self.nodes.len();
        let id = NodeId::new(id);
        let label = Some(label.into());
        let node = Node::new(value, label, color, edge_color);
        self.nodes.insert(id, node);
        id
    }

    // virtual nodes are not rendered but help to compute edge paths with no intersections
    pub fn insert_virtual_nodes(&mut self) {
        // STEP 1. let max_count i.e. find longest consecutive node<->edge count
        let deps: HashMap<NodeId, Dependencies> = dependencies(&self.edges);
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

        let max_count: u64 = root_layer
            .iter()
            .map(|el| count_nodes(&deps, el))
            .max()
            .unwrap_or(0);

        // STEP 2.
        root_layer
            .iter()
            .for_each(|el| travel_and_expand(self, &deps, el, max_count, 0));
    }

    pub fn insert_edge(&mut self, source: NodeId, target: NodeId) {
        self.edges.insert(Edge { source, target });
    }

    #[must_use]
    pub fn node_value(&self, id: &NodeId) -> Option<f64> {
        self.nodes.get(id).map(|n| n.value)
    }
}

// From all root nodes, travel left until leaf found, if count < max_count,
// fill with virtual nodes
fn travel_and_expand(
    s: &mut SankeyData,
    deps: &HashMap<NodeId, Dependencies>,
    node: &NodeId,
    max_count: u64,
    count: u64,
) {
    let Dependencies {
        inputs: node_inputs,
        ..
    } = &deps[node];
    if node_inputs.is_empty() {
        return;
    }
    for before_node in node_inputs {
        let Dependencies {
            inputs: before_node_inputs,
            ..
        } = &deps[before_node];
        if !before_node_inputs.is_empty() {
            travel_and_expand(s, deps, before_node, max_count, count + 1);
            continue;
        }
        let count = count + 1;
        if count >= max_count {
            continue;
        };
        let patch = max_count - count;
        if patch < 1 {
            continue;
        }
        s.edges.remove(&Edge {
            source: *before_node,
            target: *node,
        });
        let value = s.nodes[before_node].value;
        let id = s.nodes.len();
        let new_node_id = NodeId::new(id);
        let edge_color = s.nodes[before_node].edge_color;
        // hiding the node with edge_color painting
        let new_node = Node::new(value, None, edge_color, edge_color);
        s.nodes.insert(new_node_id, new_node);
        s.edges.insert(Edge {
            source: *before_node,
            target: new_node_id,
        });
        s.edges.insert(Edge {
            source: new_node_id,
            target: *node,
        });
    }
}

#[must_use]
fn count_nodes(deps: &HashMap<NodeId, Dependencies>, node: &NodeId) -> u64 {
    let Dependencies { inputs, .. } = &deps[node];
    if inputs.is_empty() {
        return 0;
    }
    inputs
        .iter()
        .map(|el| count_nodes(deps, el) + 1)
        .max()
        .unwrap_or(0)
}

pub fn sankey_chart<F>(
    sankey_data: SankeyData,
    width: f64,
    height: f64,
    number_format: F,
    font_size: f64,
    aria_label: Option<String>,
) -> ::svg::Document
where
    F: Fn(f64) -> String,
{
    let margin_x = width * 0.1;
    let margin_y = height * 0.05;

    let inner_chart = inner_chart(
        sankey_data,
        width - margin_x * 2.0,
        height - margin_y * 2.0,
        number_format,
        font_size,
    );

    let mut doc = ::svg::Document::new()
        .set("width", format!("{width}px"))
        .set("height", format!("{height}px"))
        .set("viewBox", (0.0, 0.0, width, height))
        .set("role", "img");

    if let Some(label) = aria_label {
        doc = doc.set("aria_label", label);
    }
    doc.add(inner_chart.set("transform", format!("translate(0.0,{margin_y})")))
}

fn inner_chart<F>(
    mut sankey_data: SankeyData,
    width: f64,
    height: f64,
    number_format: F,
    font_size: f64,
) -> ::svg::node::element::Group
where
    F: Fn(f64) -> String,
{
    let node_separation = height / 50.0;
    let node_width = width / 70.0; // TODO: make this configurable
    sankey_data.insert_virtual_nodes();
    let deps = dependencies(&sankey_data.edges);
    let layers = layers(&deps, &sankey_data.nodes);
    let layer_x_positions = layer_x_positions(layers.len(), width, node_width);
    let scale = scale(&layers, &sankey_data.nodes, height, node_separation);
    let node_positions = node_positions(
        &layers,
        &layer_x_positions,
        &sankey_data.nodes,
        &deps,
        scale,
        node_separation,
    );
    let edge_positions = edge_positions(
        &sankey_data.edges,
        &sankey_data.nodes,
        &deps,
        &node_positions,
        &layers,
        node_width,
    );

    let (svg_nodes, svg_labels): (Vec<_>, Vec<_>) = node_positions
        .iter()
        .map(|(id, node_position)| {
            let x = node_position.x;
            let y = node_position.y;
            let node_height = node_position.height;
            let value = sankey_data.nodes[id].value;
            let label_tspan = sankey_data.nodes[id].label.as_ref().map(|label| {
                ::svg::node::element::TSpan::new(format!("{label} { }", number_format(value)))
            });

            let fill: Color = sankey_data.nodes[id].color.unwrap_or(Color::new("magenta"));

            let node = ::svg::node::element::Rectangle::new()
                .set("x", format!("{x:.10}"))
                .set("y", format!("{y:.10}"))
                .set("width", format!("{node_width:.10}"))
                .set("height", format!("{node_height:.10}"))
                .set("fill", fill.as_str())
                .set("stroke", fill.as_str())
                .set("stroke-miterlimit", 0)
                .set("stroke-opacity", 1)
                .set("stroke-width", 1)
                .set("stroke-dashoffset", 0);

            let mut label = ::svg::node::element::Text::new("");
            label.get_children_mut().clear();
            if let Some(label_tspan) = label_tspan {
                label = label.add(label_tspan)
            }
            label = label
                .set("class", "label")
                .set("x", x + node_width + font_size / 2.0)
                .set("y", y + node_height / 2.0)
                .set("fill", "#111")
                .set("text-anchor", "start")
                .set("font-family", "sans-serif")
                .set("font-size", font_size)
                .set("dominant-baseline", "middle");
            (node, label)
        })
        .unzip();

    let svg_edges = edge_positions
        .iter()
        .map(|(from_top, from_bottom, to_top, to_bottom, edge_color)| {
            let d = edge_path(from_top, from_bottom, to_top, to_bottom);

            // TODO: use gradient
            let fill = edge_color.unwrap_or(Color::new("purple"));

            ::svg::node::element::Path::new()
                .set("d", d)
                .set("fill", fill.as_str())
                .set("stroke", fill.as_str())
                .set("stroke-miterlimit", 0)
                .set("stroke-opacity", 1)
                .set("stroke-width", 1)
                .set("stroke-dashoffset", 0)
        })
        .collect::<Vec<_>>();

    let mut edges = ::svg::node::element::Group::new();
    let mut nodes = ::svg::node::element::Group::new();
    let mut labels = ::svg::node::element::Group::new();

    for c in svg_edges {
        edges = edges.add(c);
    }
    for c in svg_nodes {
        nodes = nodes.add(c);
    }
    for c in svg_labels {
        labels = labels.add(c);
    }
    let style = ::svg::node::element::Style::new(
        "text.label {
           cursor: pointer;
         }
         text.label:hover {
           font-weight: bold;
         }",
    );
    ::svg::node::element::Group::new()
        .add(style)
        .add(edges)
        .add(nodes)
        .add(labels)
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

    format!("M {from_top_x:.10} {from_top_y:.10} C {mid_x:.10} {from_top_y:.10}, {mid_x:.10} {to_top_y:.10}, {to_top_x:.10} {to_top_y:.10} L {to_bottom_x:.10} {to_bottom_y:.10} C {mid_x:.10} {to_bottom_y:.10}, {mid_x:.10} {from_bottom_y:.10}, {from_bottom_x:.10} {from_bottom_y:.10}  Z")
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
        nodes[b]
            .value
            .partial_cmp(&nodes[a].value)
            .unwrap_or(Ordering::Equal)
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
                deps[id].inputs.iter().fold(f64::INFINITY, |y, successor| {
                    y.min(node_positions[successor].y)
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
    #[must_use]
    const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

// computes 2 edge paths between 2 nodes => one on top, one on bottom
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
        *total_input_values.entry(*target).or_default() += nodes[source].value;
    }
    layers
        .iter()
        .flat_map(|layer| {
            layer
                .iter()
                .filter_map(|node| {
                    let from = node_positions[node];
                    let edge = edges.iter().find(|edge| edge.source == *node)?;
                    let to = node_positions[&edge.target];
                    let scale = to.height / total_input_values[&edge.target];
                    let mut to_y_start = node_positions.get(&edge.target).unwrap().y;
                    if (from.y - to.y).abs() > 1.0 {
                        let prev_nodes = deps[&edge.target]
                            .inputs
                            .iter()
                            .filter(|id| from.y - node_positions[id].y > 1.0)
                            .collect::<Vec<_>>();
                        to_y_start += prev_nodes
                            .iter()
                            .fold(0.0, |acc, id| acc + node_positions[id].height);
                    }
                    let to_y_end = to_y_start + nodes[&edge.source].value * scale;
                    let points = (
                        Point::new(from.x + node_width, from.y),
                        Point::new(from.x + node_width, from.y + from.height),
                        Point::new(to.x, to_y_start),
                        Point::new(to.x, to_y_end),
                        nodes[&edge.source].edge_color,
                    );
                    Some(points)
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
