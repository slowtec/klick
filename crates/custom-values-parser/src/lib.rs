use std::{collections::HashSet, sync::LazyLock};

use regex::Regex;
use thiserror::Error;

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

#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum NumberFormat {
    // FIXME refactor to use
    DE,
    US,
}

impl NumberFormat {
    fn parse_number(&self, num_str: &str) -> Result<f64, std::num::ParseFloatError> {
        match self {
            NumberFormat::DE => {
                let normalized = num_str.replace('.', "").replace(',', ".");
                normalized.parse::<f64>()
            }
            NumberFormat::US => {
                let normalized = num_str.replace(',', "");
                normalized.parse::<f64>()
            }
        }
    }
}

static OPTIONAL_REGEXP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*"(?P<source>[^"]+)"\s*(?P<value>[\d.,]+)?\s*(?:"(?P<target>[^"]+)")\s*$"#)
        .unwrap()
});

#[allow(clippy::missing_panics_doc)]
pub fn parse_line(
    line_number: usize,
    line: &str,
    number_format: NumberFormat,
) -> Result<Option<CustomEmission>, String> {
    let Some(captures) = OPTIONAL_REGEXP.captures(line) else {
        if line.trim().is_empty() {
            return Ok(None);
        }
        return Err(format!(
            //"Line \"{}\" does not match expected format, which must be: [\"ID\" \"ID\"] or [\"ID\" NUM \"ID\"]",
            "Zeile \"{line_number}\" ist nicht im erwarteten Format, erwartet war: [\"ID\" \"ID\"] oder [\"ID\" NUM \"ID\"]"
        ));
    };
    let source = captures["source"].to_string();
    let target = captures["target"].to_string();
    let Some(value) = captures.name("value") else {
        let emission = CustomEmission::EdgeUndefined(EdgeUndefined {
            line: line_number,
            source,
            target,
        });
        return Ok(Some(emission));
    };

    let value = number_format.parse_number(value.as_str()).map_err(|err| {
        format!(
            //"The number \"{}\" on line \"{}\" does not match expected format: {}",
            "Die Nummer \"{}\" auf Zeile \"{line_number}\" ist nicht im erwarteten Format: {err}",
            value.as_str(),
        )
    })?;

    Ok(Some(CustomEmission::EdgeDefined(EdgeDefined {
        line: line_number,
        source,
        target,
        value,
    })))
}

pub fn parse_emission(
    input: &str,
    number_format: NumberFormat,
) -> Result<Vec<CustomEmission>, String> {
    let emissions = input
        .lines()
        .enumerate()
        .map(|(line_number, line)| parse_line(line_number + 1, line, number_format))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();
    Ok(emissions)
}

#[derive(Error, Debug)]
pub enum CustomEmissionParserError {
    //#[error("Can't use reserved name {} on line: {} as source name", name, line)]
    #[error("Reservierter Name {} auf Zeile: {} als Quellname", name, line)]
    ReservedNameVoilation { name: String, line: usize },

    #[error(
       //"Edges are not unique: Edge (from line {}) and Edge (from line {}) need to be different",
       "Kantennamen sind nicht eindeutig: Kante (auf Zeile {}) und Kante (auf Zeile {}) müssen sich unterscheiden",
       e1_line,
       e2_line
   )]
    EdgeNotUniqueVoilation { e1_line: usize, e2_line: usize },

    //#[error("Edge-Cycle detected on line: {}", line)]
    #[error("Kanten-Zyklus erkannt auf Zeile: {}", line)]
    InsideEdgeCycleVoilation { line: usize },

    //#[error("Node with emission name (from line {}) collides with with same node source name (on line {})!", e1_line, e2_line)]
    #[error("Knoten mit Emissionsnamen (auf Zeile {}) kollidiert mit Knoten gleichen Quellnamens (auf Zeile {})!", e1_line, e2_line)]
    DuplicatedNodeNameVoilation { e1_line: usize, e2_line: usize },

    #[error(
       //"Node without emission (on line {}) may not link to node with emission value (on line {})!",
       "Knoten ohne Emission (auf Zeile {}) darf nicht mit Knoten mit Emissionswert (auf Zeile {}) verlinkt sein!",
       e1_line,
       e2_line
   )]
    EdgeToLeafVoilation { e1_line: usize, e2_line: usize },

    #[error(
       //"Node with emission (on line {}) may not link to node with emission (on line {})!",
       "Knoten mit Emission (auf Zeile {}) darf nicht mit Knoten mit Emission (auf Zeile {}) verlinkt sein!",
       e1_line,
       e2_line
   )]
    NodeToNodeLinkVoilation { e1_line: usize, e2_line: usize },

    //#[error("Found cycle for {} (one line(s) {})", name, line)]
    #[error("Zyklus für {} erkannt (eine Zeile(n) {})", name, line)]
    EdgeCycleVoilation { name: String, line: usize },

    //#[error("Found nodes which are not connected properly (one line(s) {})", lines)]
    #[error("Nicht korrekt verbundene Knoten gefunden (auf Zeile(n) {})", lines)]
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
