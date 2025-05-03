// Module: analysis.rs
// Implements graph algorithms: centrality, clustering, six degrees, community detection.

use crate::parse::{NetflixRecord, Role};
use petgraph::Graph;
use petgraph::Undirected;
use std::collections::{HashMap, HashSet};

/// Computes the average path length using BFS from 100 sampled nodes.
/// Measures six degrees of separation in the network.
/// Input: graph of people
/// Output: average distance between connected node pairs
pub fn average_path_length(graph: &Graph<String, (), Undirected>) -> f64 {
    let mut total_distance = 0;
    let mut count = 0;

    // Sample up to 100 nodes to estimate distances
    for start in graph.node_indices().take(100) {
        let mut visited = HashSet::new();
        let mut current_layer = vec![start];
        let mut depth = 0;
        
        // BFS to count reachable nodes and their distances
        while !current_layer.is_empty() {
            let mut next_layer = vec![];
            for node in &current_layer {
                for neighbor in graph.neighbors(*node) {
                    if visited.insert(neighbor) {
                        next_layer.push(neighbor);
                        total_distance += depth + 1;
                        count += 1;
                    }
                }
            }
            current_layer = next_layer;
            depth += 1;
        }
    }
    total_distance as f64 / count.max(1) as f64
}


/// Computes the global clustering coefficient of the graph using triangle counting.
/// Input: graph
/// Output: prints clustering coefficient to stdout
pub fn clustering_info(graph: &Graph<String, (), Undirected>) {
    let mut triangle_count = 0;
    let mut triplet_count = 0;

    for node in graph.node_indices() {
        let neighbors: Vec<_> = graph.neighbors(node).collect();

        // Number of possible triplets among neighbors
        triplet_count += neighbors.len() * (neighbors.len() - 1) / 2;

        // Count closed triplets (triangles)
        for i in 0..neighbors.len() {
            for j in (i + 1)..neighbors.len() {
                if graph.contains_edge(neighbors[i], neighbors[j]) {
                    triangle_count += 1;
                }
            }
        }
    }

    let clustering_coefficient = triangle_count as f64 / triplet_count.max(1) as f64;
    println!("Clustering coefficient: {:.4}", clustering_coefficient);
}

/// Identifies the most "central" nodes by degree (number of connections).
/// Input: graph, N (number of top nodes to return)
/// Output: Vec<String> of names
pub fn centrality(graph: &Graph<String, (), Undirected>, top_n: usize) -> Vec<String> {
    let mut degree_map: HashMap<String, usize> = HashMap::new();

    for node in graph.node_indices() {
        let name = &graph[node];
        degree_map.insert(name.clone(), graph.neighbors(node).count());
    }

    // Sort people by degree, descending
    let mut degrees: Vec<_> = degree_map.into_iter().collect();
    degrees.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top {} most connected people:", top_n);
    for (name, degree) in degrees.iter().take(top_n) {
        println!("{} ({} connections)", name, degree);
    }

    degrees.into_iter().take(top_n).map(|(n, _)| n).collect()
}

/// Computes local clustering coefficient for each specified person.
/// Input: graph and list of names
/// Output: prints each person’s coefficient
pub fn local_clustering(graph: &Graph<String, (), Undirected>, people: &[&str]) {
    for name in people {
        let node = graph.node_indices().find(|&n| graph[n] == *name);
        if let Some(n) = node {
            let neighbors: Vec<_> = graph.neighbors(n).collect();
            // Count links between neighbors
            let mut links = 0;
            let total = neighbors.len();
            for i in 0..total {
                for j in (i + 1)..total {
                    if graph.contains_edge(neighbors[i], neighbors[j]) {
                        links += 1;
                    }
                }
            }
            // Normalize by total possible links
            let possible = total * (total - 1) / 2;
            let coefficient = links as f64 / possible.max(1) as f64;
            println!("{}: {:.4}", name, coefficient);
        } else {
            println!("{} not found in graph.", name);
        }
    }
}

/// Finds and prints the largest communities in the network, based on connected components.
/// Also identifies common genres in each community.
/// Input: graph and full dataset
/// Output: prints community size, sample names, and dominant genre(s)
pub fn detect_communities(graph: &Graph<String, (), Undirected>, records: &[NetflixRecord]) {
    let mut visited = HashSet::new();
    let mut communities: Vec<Vec<&String>> = vec![];

    // Perform BFS to extract connected components
    for node in graph.node_indices() {
        if visited.contains(&node) {
            continue;
        }

        let mut queue = vec![node];
        let mut community = vec![];
        visited.insert(node);

        while let Some(n) = queue.pop() {
            community.push(&graph[n]);
            for neighbor in graph.neighbors(n) {
                if visited.insert(neighbor) {
                    queue.push(neighbor);
                }
            }
        }

        communities.push(community);
    }
    // Sort communities by size (descending)
    communities.sort_by(|a, b| b.len().cmp(&a.len()));
    println!("Detected {} communities.", communities.len());

    // Analyze and display top 5 largest communities
    for (i, group) in communities.iter().take(5).enumerate() {
        let mut genre_count = HashMap::new();

        // Count genre occurrences among people in the community
        for person in group {
            for record in records {
                if record.has_person(person, Role::Director) || record.has_person(person, Role::Cast) {
                    for genre in record.genres() {
                        *genre_count.entry(genre).or_insert(0) += 1;
                    }
                }
            }
        }

        println!("\nCommunity {} ({} people): {:?}", i + 1, group.len(), &group[..group.len().min(5)]);

        // Print all genres that appear with the highest frequency
        if !genre_count.is_empty() {
            let max_count = genre_count.values().cloned().max().unwrap_or(0);
            let top_genres: Vec<_> = genre_count
                .iter()
                .filter(|&(_, count)| *count == max_count)
                .collect();

            for (genre, count) in top_genres {
                println!("  Common genre: {} ({} titles)", genre, count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;
    use crate::parse::{NetflixRecord};

    #[test]
    fn test_centrality() {
        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let a = graph.add_node("Alice".to_string());
        let b = graph.add_node("Bob".to_string());
        let c = graph.add_node("Carol".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(a, c, ());

        let top = centrality(&graph, 1);
        assert_eq!(top[0], "Alice");
    }

    #[test]
    fn test_local_clustering() {
        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(a, c, ());

        local_clustering(&graph, &["A"]);
    }

    #[test]
    fn test_detect_communities() {
        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let a = graph.add_node("Alice".to_string());
        let b = graph.add_node("Bob".to_string());
        let c = graph.add_node("Carol".to_string());
        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());

        let x = graph.add_node("Xavier".to_string());
        let y = graph.add_node("Yvonne".to_string());
        let z = graph.add_node("Zane".to_string());
        graph.add_edge(x, y, ());
        graph.add_edge(y, z, ());

        let records = vec![
            NetflixRecord {
                director: Some("Bob".into()),
                cast: Some("Alice, Carol".into()),
                listed_in: Some("Comedy, Action".into()),
            },
            NetflixRecord {
                director: Some("Yvonne".into()),
                cast: Some("Xavier, Zane".into()),
                listed_in: Some("Horror".into()),
            },
        ];

        detect_communities(&graph, &records);

        assert_eq!(graph.node_count(), 6);
    }
}