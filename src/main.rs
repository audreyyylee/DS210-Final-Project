// Module: main.rs
// Orchestrates the loading, analysis, and output of Netflix collaboration data.

mod parse;
mod graph_builder;
mod analysis;

use crate::parse::load_data;
use crate::graph_builder::build_graph;
use crate::analysis::{
    average_path_length, clustering_info, centrality,
    local_clustering, detect_communities
};

fn main() {
    // Load and parse CSV into a vector of NetflixRecord structs
    let records = load_data("netflix_titles.csv");
    // Construct undirected graph from cast and director relationships
    let graph = build_graph(&records);

    // Compute average path length to understand network connectedness
    println!("Average path length (six degrees): {:.2}", average_path_length(&graph));

    // Global clustering coefficient
    clustering_info(&graph);

    // Get top 5 most central individuals by number of connections
    let top_people = centrality(&graph, 5);

    // Analyze how clustered each top person’s neighbors are
    println!("\nLocal clustering coefficients:"); 
    let top_refs: Vec<&str> = top_people.iter().map(String::as_str).collect();
    local_clustering(&graph, &top_refs);

    // Detect and analyze connected communities of collaborators
    println!("\nCommunity detection:"); 
    detect_communities(&graph, &records);
}