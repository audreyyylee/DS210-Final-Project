// Module: parse.rs
// Loads and structures Netflix dataset. Includes helper logic for people-role lookup and genre extraction.

use std::fs::File;
use csv::ReaderBuilder;
use serde::Deserialize;

/// Struct representing a Netflix title.
/// Fields include cast, director, and genre list.
#[derive(Debug, Deserialize)]
pub struct NetflixRecord {
    pub director: Option<String>,
    pub cast: Option<String>,
    pub listed_in: Option<String>,
}

/// Enum identifying whether a person appears as a Cast member or a Director.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Role {
    Cast,
    Director,
}

impl NetflixRecord {
    /// Checks if a person appears in this title in the given role.
    /// Inputs:
    /// - `person`: name to search
    /// - `role`: Role::Cast or Role::Director
    /// Output: true if person is found in that role
    pub fn has_person(&self, person: &str, role: Role) -> bool {
        match role {
            Role::Director => self
                .director
                .as_ref()
                .map_or(false, |d| d.contains(person)),
            Role::Cast => self
                .cast
                .as_ref()
                .map_or(false, |c| c.contains(person)),
        }
    }

    /// Returns a list of genres associated with the title.
    /// Output: Vec<String> split from listed_in field
    pub fn genres(&self) -> Vec<String> {
        self.listed_in
            .as_ref()
            .map(|g| g.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }
}

/// Reads a CSV file and parses each row into a NetflixRecord struct.
/// Skips invalid rows during parsing.
pub fn load_data(path: &str) -> Vec<NetflixRecord> {
    let file = File::open(path).expect("Cannot open file");
    let rdr = ReaderBuilder::new()
        .flexible(true)
        .from_reader(file);

    rdr.into_deserialize()
        .filter_map(Result::ok)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_person_cast() {
        let record = NetflixRecord {
            director: Some("Jane Smith".to_string()),
            cast: Some("Alice Johnson, Bob Lee".to_string()),
            listed_in: Some("Dramas".to_string()),
        };
        assert!(record.has_person("Alice Johnson", Role::Cast));
        assert!(!record.has_person("Jane Smith", Role::Cast));
    }
}