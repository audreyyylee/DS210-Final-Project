// Module: graph_builder.rs
// Constructs an undirected graph from the Netflix dataset, connecting collaborators.
// Nodes = people; edges = worked on same movie or show.

use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use crate::parse::NetflixRecord;

/// Builds a graph where each node is a person and edges represent co-collaboration.
/// Input: slice of NetflixRecords
/// Output: undirected Graph<String, (), Undirected>
pub fn build_graph(records: &[NetflixRecord]) -> Graph<String, (), Undirected> {
    let mut graph = Graph::<String, (), Undirected>::new_undirected();
    let mut node_map: HashMap<String, NodeIndex> = HashMap::new();

    for record in records {
        let mut people: Vec<String> = vec![];
    
        // Collect people from director and cast fields (split by comma)
        if let Some(d) = &record.director {
            people.extend(d.split(",").map(|s| s.trim().to_string()));
        }
        if let Some(c) = &record.cast {
            people.extend(c.split(",").map(|s| s.trim().to_string()));
        }

        // Assign each person a node index (reuse if already present)
        let indices: Vec<NodeIndex> = people.iter().map(|person| {
            *node_map.entry(person.clone())
                .or_insert_with(|| graph.add_node(person.clone()))
        }).collect();

        // Create edges between all unique pairs in the current title
        for i in 0..indices.len() {
            for j in (i + 1)..indices.len() {
                if !graph.contains_edge(indices[i], indices[j]) {
                    graph.add_edge(indices[i], indices[j], ());
                }
            }
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::NetflixRecord;

    #[test]
    fn test_build_graph() {
        let records = vec![
            NetflixRecord {
                director: Some("Alice".into()),
                cast: Some("Bob, Charlie".into()),
                listed_in: Some("Drama".into()),
            },
            NetflixRecord {
                director: Some("David".into()),
                cast: Some("Charlie, Eve".into()),
                listed_in: Some("Action".into()),
            },
        ];

        let graph = build_graph(&records);
        assert!(graph.node_count() >= 5);
        assert!(graph.edge_count() >= 3);
    }
}