use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct EngineConfig {
    pub name: String,
    pub signatures: Vec<Signature>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", content = "value")]
pub enum SignatureType {
    #[serde(rename = "path_contains")]
    PathContains(String),
    #[serde(rename = "extension")]
    Extension(String),
    #[serde(rename = "filename")]
    Filename(String),
    #[serde(rename = "filename_starts_with")]
    FilenameStartsWith(String),
    #[serde(rename = "filename_ends_with")]
    FilenameEndsWith(String),
    #[serde(rename = "path_component")]
    PathComponent(String), // e.g., "level*"
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Signature {
    #[serde(flatten)]
    pub r#type: SignatureType,
    #[serde(default = "default_weight")]
    pub weight: f32,
}

fn default_weight() -> f32 {
    1.0
}

#[derive(Debug, Serialize)]
pub struct DetectionResult {
    pub engine: String,
    pub confidence: f32,
    pub matches: Vec<String>,
}

pub fn detect_engine(files: &[String], configs: &[EngineConfig]) -> DetectionResult {
    let mut scores: HashMap<String, f32> = HashMap::new();
    let mut matches: HashMap<String, Vec<String>> = HashMap::new();

    // Initialize scores
    for config in configs {
        scores.insert(config.name.clone(), 0.0);
    }
    scores.insert("Unknown".to_string(), 0.0);

    for file in files {
        let lower_file = file.to_lowercase();
        
        for config in configs {
            for sig in &config.signatures {
                let match_found = match &sig.r#type {
                    SignatureType::PathContains(val) => lower_file.contains(&val.to_lowercase()),
                    SignatureType::Extension(val) => lower_file.ends_with(&format!(".{}", val.to_lowercase())),
                    SignatureType::Filename(val) => lower_file.ends_with(&format!("/{}", val.to_lowercase())) || lower_file == val.to_lowercase(),
                    SignatureType::FilenameStartsWith(val) => {
                         let filename = lower_file.split('/').last().unwrap_or(&lower_file);
                         filename.starts_with(&val.to_lowercase())
                    },
                    SignatureType::FilenameEndsWith(val) => lower_file.ends_with(&val.to_lowercase()),
                     SignatureType::PathComponent(val) => {
                         let val_clean = val.replace("*", "").to_lowercase();
                         let filename = lower_file.split('/').last().unwrap_or(&lower_file);
                         filename.starts_with(&val_clean)
                    }
                };

                if match_found {
                     *scores.get_mut(&config.name).unwrap() += sig.weight;
                     matches.entry(config.name.clone()).or_default().push(file.clone());
                }
            }
        }
    }

    // Determine winner
    let mut best_engine = "Unknown".to_string();
    let mut highest_score = 0.0;

    for (engine, score) in scores {
        // Simple 'greater than' logic.
        // If we have a tie, the first one in the map (randomish) wins, or we could sort.
        // But usually weights should differentiate enough.
        if score > highest_score {
            highest_score = score;
            best_engine = engine;
        }
    }

    // Normalize confidence
    // Now that we have weights, a score of 10.0 (project.godot) is very confident.
    // A score of 0.5 (options.ini) is low confidence.
    let confidence = if highest_score >= 5.0 {
        1.0
    } else if highest_score >= 2.0 {
        0.8 
    } else if highest_score > 0.0 {
        0.5
    } else {
        0.0
    };

    DetectionResult {
        engine: best_engine.clone(),
        confidence,
        matches: matches.remove(&best_engine).unwrap_or_default(),
    }
}

pub fn load_config(path: &str) -> std::io::Result<Vec<EngineConfig>> {
    let content = std::fs::read_to_string(path)?;
    let configs: Vec<EngineConfig> = serde_json::from_str(&content)?;
    Ok(configs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_config() -> Vec<EngineConfig> {
        vec![
            EngineConfig {
                name: "Unity".to_string(),
                signatures: vec![
                    Signature {
                        r#type: SignatureType::Extension("assets".to_string()),
                        weight: 1.5,
                    },
                    Signature {
                         r#type: SignatureType::PathContains("_Data/Managed".to_string()),
                         weight: 2.0,
                    }
                ]
            },
            EngineConfig {
                name: "Godot".to_string(),
                signatures: vec![
                     Signature {
                        r#type: SignatureType::Extension("pck".to_string()),
                        weight: 2.0
                     }
                ]
            }
        ]
    }

    #[test]
    fn test_unity_weighted() {
        let configs = get_test_config();
        let files = vec![
            "Game_Data/Managed/Assembly-CSharp.dll".to_string(), // Matches path_contains (2.0)
            "Game.exe".to_string(),
        ];
        let result = detect_engine(&files, &configs);
        assert_eq!(result.engine, "Unity");
        assert!(result.confidence >= 0.8);
    }
}
