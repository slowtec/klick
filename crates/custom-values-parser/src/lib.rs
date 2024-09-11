use klick_presenter::Lng;
use std::collections::HashSet;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Clone)]
pub struct EdgeDefined {
    pub line: usize,
    pub source: String,
    pub target: String,
    pub value: f64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EdgeUndefined {
    pub line: usize,
    pub source: String,
    pub target: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CustomEmission {
    EdgeDefined(EdgeDefined),
    EdgeUndefined(EdgeUndefined),
}

impl CustomEmission {
    #[must_use]
    pub const fn line(&self) -> usize {
        match self {
            Self::EdgeUndefined(e) => e.line,
            Self::EdgeDefined(e) => e.line,
        }
    }

    #[must_use]
    pub const fn target(&self) -> &String {
        match self {
            Self::EdgeUndefined(e) => &e.target,
            Self::EdgeDefined(e) => &e.target,
        }
    }

    #[must_use]
    pub const fn source(&self) -> &String {
        match self {
            Self::EdgeUndefined(e) => &e.source,
            Self::EdgeDefined(e) => &e.source,
        }
    }
}

fn parse_line(line_number: usize, line: &str, lang: Lng) -> Result<Option<CustomEmission>, String> {
    let trimmed = line.trim();

    if trimmed.is_empty() {
        return Ok(None);
    }

    if !trimmed.starts_with('"') || !trimmed.ends_with('"') || trimmed.len() < 2 {
        return Err(
            match lang {
                Lng::De => format!("Zeile \"{line_number}\" ist nicht im erwarteten Format, erwartet war: [\"ID\" \"ID\"] oder [\"ID\" NUM \"ID\"]"),
                Lng::En => format!("Line \"{line_number}\" is not formatted correctly, expected was: [\"ID\" \"ID\"] oder [\"ID\" NUM \"ID\"]")
            }
        );
    }

    let trimmed = &trimmed[1..trimmed.len() - 1];

    let mut parts = trimmed
        .split('"')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                None
            } else {
                Some(part)
            }
        })
        .collect::<Vec<&str>>();

    if parts.len() < 2 || parts.len() > 3 {
        return Err(
            match lang {
                Lng::De => format!("Zeile \"{line_number}\" ist nicht im erwarteten Format, erwartet war: [\"ID\" \"ID\"] oder [\"ID\" NUM \"ID\"]"),
                Lng::En => format!("Line \"{line_number}\" is not formatted correctly, expected was: [\"ID\" \"ID\"] oder [\"ID\" NUM \"ID\"]")
            }
        );
    }

    let source = parts.remove(0).to_string();
    let target = if parts.len() == 2 {
        parts.remove(1).to_string()
    } else {
        parts.remove(0).to_string()
    };

    let Some(value_str) = parts.first() else {
        return Ok(Some(CustomEmission::EdgeUndefined(EdgeUndefined {
            line: line_number,
            source,
            target,
        })));
    };

    let normalized_value = match lang {
        Lng::De => value_str.replace('.', "").replace(',', "."),
        Lng::En => value_str.replace(',', ""),
    };

    let value = normalized_value.parse::<f64>().map_err(|err| {
        match lang {
            Lng::De => format!("Die Nummer \"{value_str}\" auf Zeile \"{line_number}\" ist nicht im erwarteten Format: {err}"),
            Lng::En => format!("The number \"{value_str}\" on line \"{line_number}\" was not expected: {err}"),
        }
    })?;

    if value < 0.0 {
        return Err(match lang {
            Lng::De => format!(
                "Negative Nummer \"{value_str}\" auf Zeile \"{line_number}\" macht keinen Sinn!"
            ),
            Lng::En => {
                format!("Negative number \"{value_str}\" on line \"{line_number}\" is insane!")
            }
        });
    }

    Ok(Some(CustomEmission::EdgeDefined(EdgeDefined {
        line: line_number,
        source,
        target,
        value,
    })))
}

pub fn parse_emission(input: &str, lang: Lng) -> Result<Vec<CustomEmission>, String> {
    let emissions = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| parse_line(line_number + 1, line, lang))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(emissions)
}

impl CustomEmissionParserError {
    pub fn format_error(&self, lang: Lng) -> String {
        match self {
            Self::ReservedNameVoilation{name, line} => {
                match lang {
                    Lng::De => format!("Reservierter Name {} auf Zeile: {} als Quellname", name, line),
                    Lng::En => format!("Can't use reserved name {} on line: {} as source name", name, line)
                }
            }
            Self::EdgeNotUniqueVoilation { e1_line , e2_line } => {
                match lang {
                    Lng::De => format!("Kantennamen sind nicht eindeutig: Kante (auf Zeile {}) und Kante (auf Zeile {}) müssen sich unterscheiden", e1_line, e2_line),
                    Lng::En => format!("Edges are not unique: Edge (from line {}) and Edge (from line {}) need to be different", e1_line, e2_line)
                }
            }
            Self::InsideEdgeCycleVoilation { line } => {
                match lang {
                    Lng::De => format!("Kanten-Zyklus erkannt auf Zeile: {}", line),
                    Lng::En => format!("Edge-Cycle detected on line: {}", line)
                }
            }
            Self::DuplicatedNodeNameVoilation { e1_line, e2_line } => {
                match lang {
                    Lng::De => format!("Knoten mit Emissionsnamen (auf Zeile {}) kollidiert mit Knoten gleichen Quellnamens (auf Zeile {})!", e1_line, e2_line),
                    Lng::En => format!("Node with emission name (from line {}) collides with with same node source name (on line {})!", e1_line, e2_line)
                }
            }
            Self::EdgeToLeafVoilation { e1_line, e2_line } => {
                match lang {
                    Lng::De => format!("Knoten ohne Emission (auf Zeile {}) darf nicht mit Knoten mit Emissionswert (auf Zeile {}) verlinkt sein!", e1_line, e2_line),
                    Lng::En => format!("Node without emission (on line {}) may not link to node with emission value (on line {})!", e1_line, e2_line)
                }
            }
            Self::NodeToNodeLinkVoilation { e1_line, e2_line } => {
                match lang {
                    Lng::De => format!("Knoten mit Emission (auf Zeile {}) darf nicht mit Knoten mit Emission (auf Zeile {}) verlinkt sein!", e1_line, e2_line),
                    Lng::En => format!("Node with emission (on line {}) may not link to node with emission (on line {})!", e1_line, e2_line)
                }
            }
            Self::EdgeCycleVoilation { name, line } => {
                match lang {
                    Lng::De => format!("Zyklus für {} erkannt (eine Zeile(n) {})", name, line),
                    Lng::En => format!("Found cycle for {} (one line(s) {})", name, line)
                }
            }
            Self::DetachedNodesVoilation { lines } => {
                match lang {
                    Lng::De => format!("Nicht korrekt verbundene Knoten gefunden (auf Zeile(n) {})", lines),
                    Lng::En => format!("Found nodes which are not connected properly (one line(s) {})", lines)
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum CustomEmissionParserError {
    ReservedNameVoilation { name: String, line: usize },
    EdgeNotUniqueVoilation { e1_line: usize, e2_line: usize },
    InsideEdgeCycleVoilation { line: usize },
    DuplicatedNodeNameVoilation { e1_line: usize, e2_line: usize },
    EdgeToLeafVoilation { e1_line: usize, e2_line: usize },
    NodeToNodeLinkVoilation { e1_line: usize, e2_line: usize },
    EdgeCycleVoilation { name: String, line: usize },
    DetachedNodesVoilation { lines: String },
}

// these checks are written for like 8 nodes max, so performance isn't the main objective
pub fn check_graph(
    custom_edges: &[CustomEmission],
    all_internal_nodes_names: Vec<String>,
) -> Result<(), CustomEmissionParserError> {
    let mut edges_defined: Vec<EdgeDefined> = vec![];
    let mut edges_undefined: Vec<EdgeUndefined> = vec![];
    for edge in custom_edges {
        match edge {
            CustomEmission::EdgeDefined(e) => edges_defined.push(e.clone()),
            CustomEmission::EdgeUndefined(e) => edges_undefined.push(e.clone()),
        }
    }

    // 1. no use of reserved names in source
    let reserved_names: Vec<String> = all_internal_nodes_names.clone();
    if let Some(edge) = custom_edges
        .iter()
        .find(|edge| reserved_names.iter().any(|f| f == edge.source()))
    {
        return Err(CustomEmissionParserError::ReservedNameVoilation {
            name: edge.source().to_string(),
            line: edge.line(),
        });
    }

    // 2. all edges must be unique
    for i in 0..edges_defined.len() {
        for j in 0..edges_defined.len() {
            if i == j {
                continue;
            }
            let e1 = &edges_defined[i];
            let e2 = &edges_defined[j];
            if e1.source == e2.source && e1.target == e2.target {
                return Err(CustomEmissionParserError::EdgeNotUniqueVoilation {
                    e1_line: e1.line,
                    e2_line: e2.line,
                });
            }
        }
    }
    // 3. no cycle inside edge(s)
    if let Some(edge) = custom_edges.iter().find(|e| e.source() == e.target()) {
        return Err(CustomEmissionParserError::InsideEdgeCycleVoilation { line: edge.line() });
    }

    // 4. ensure that leaf source names are unique
    for i in 0..edges_defined.len() {
        for j in 0..edges_defined.len() {
            if i == j {
                continue;
            }
            let e1 = &edges_defined[i];
            let e2 = &edges_defined[j];
            if e1.source == e2.source {
                return Err(CustomEmissionParserError::DuplicatedNodeNameVoilation {
                    e1_line: e1.line,
                    e2_line: e2.line,
                });
            }
        }
    }

    // 5. connecting edges (EdgeUndefined) must not link to leafs (EdgeDefined)
    for eu in &edges_undefined {
        for ed in &edges_defined {
            if eu.target == ed.source {
                return Err(CustomEmissionParserError::EdgeToLeafVoilation {
                    e1_line: eu.line,
                    e2_line: ed.line,
                });
            }
        }
    }

    // 6. leafs (EdgeDefined) must not link to leafs either (EdgeDefined)
    for i in 0..edges_defined.len() {
        for j in 0..edges_defined.len() {
            if i == j {
                continue;
            }
            let e1 = &edges_defined[i];
            let e2 = &edges_defined[j];
            if e1.target == e2.source {
                return Err(CustomEmissionParserError::NodeToNodeLinkVoilation {
                    e1_line: e1.line,
                    e2_line: e2.line,
                });
            }
        }
    }

    // 7. no cycle between edge(s)
    let mut edges_unvisited: Vec<CustomEmission> = custom_edges.to_owned();
    let mut edges_visited: Vec<CustomEmission> = vec![];
    let mut visited_targets: HashSet<String> = all_internal_nodes_names
        .into_iter()
        .collect::<HashSet<String>>();
    let mut found_one_more = false;
    loop {
        // 1. go over edges_unvisited and take all targets which match visited_targets
        let mut edges_unvisited_new: Vec<CustomEmission> = vec![];
        let mut edges_visited_new: Vec<CustomEmission> = vec![];
        for edge in &edges_unvisited {
            if visited_targets.contains(edge.target()) {
                edges_visited_new.push(edge.clone());
                found_one_more = true;
            } else {
                edges_unvisited_new.push(edge.clone());
            }
        }

        let mut visited_targets_new: HashSet<String> = HashSet::new();
        // 2. check if their source is already in visited_targets:
        for e in &edges_visited_new {
            if let Some(n) = visited_targets.get(e.source()) {
                //   - if they are already, find collision and report Err
                // FIXME find the other nodes in conflict
                return Err(CustomEmissionParserError::EdgeCycleVoilation {
                    name: n.to_string(),
                    line: e.line(),
                });
            }
            //   - if not, add them & and add edges to edges_unvisited (set found_one_more to true)
            visited_targets_new.insert(e.source().clone());
        }
        // update old state with new knowledge
        edges_unvisited = edges_unvisited_new;
        edges_visited.append(&mut edges_visited_new.clone());
        visited_targets.extend(visited_targets_new);

        // 3. if found_one_more is false, exit
        if !found_one_more {
            break;
        }
        found_one_more = false;
    }

    // 8. enforce connections: no loose edges, each must connect to something
    //    if all_edges_list is not empty, report the remainder elements as loose edges
    if !edges_unvisited.is_empty() {
        let lines: String = edges_unvisited.into_iter().fold(String::new(), |sum, e| {
            let comma = if sum.is_empty() { "" } else { ", " };
            sum + format!("{}{}", comma, e.line()).as_str()
        });
        return Err(CustomEmissionParserError::DetachedNodesVoilation { lines });
    }

    Ok(())
}
