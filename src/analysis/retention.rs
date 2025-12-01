use crate::script::{Scene, VideoScript};

#[derive(Debug, Clone)]
pub struct DropoffPrediction {
    pub scene_index: usize,
    pub predicted_dropoff_percent: f32,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct RetentionHeatmap {
    pub scene_scores: Vec<SceneRetention>,
    pub overall_retention_score: f32,
    pub critical_moments: Vec<usize>, // Scene indices with predicted high drop-off
}

#[derive(Debug, Clone)]
pub struct SceneRetention {
    pub scene_index: usize,
    pub momentum: f32,        // 0-100
    pub retention_score: f32, // 0-100
}

pub struct RetentionAnalyzer;

impl RetentionAnalyzer {
    /// Calculate momentum for a scene based on pacing and visual density
    pub fn calculate_momentum(scene: &Scene, _fps: u32) -> f32 {
        // Momentum = (visual_layers × 20) + pacing_factor
        let visual_layers = scene.layers.len() as f32;
        let visual_score = (visual_layers * 20.0).min(60.0); // Cap at 60

        // Pacing factor: shorter scenes = higher energy
        let duration_factor = if scene.duration < 5.0 {
            40.0 // Very high momentum
        } else if scene.duration < 10.0 {
            30.0 // High momentum
        } else if scene.duration < 20.0 {
            20.0 // Medium momentum
        } else {
            10.0 // Low momentum (long scenes drag)
        };

        (visual_score + duration_factor).min(100.0)
    }

    /// Predict drop-off points based on momentum drops and scene characteristics
    pub fn predict_dropoff(script: &VideoScript) -> Vec<DropoffPrediction> {
        let mut predictions = Vec::new();

        for (i, scene) in script.scenes.iter().enumerate() {
            let momentum = Self::calculate_momentum(scene, script.metadata.fps);

            // Predict drop-off if momentum is low
            if momentum < 40.0 {
                let dropoff_percent = 100.0 - momentum; // Lower momentum = higher predicted drop-off
                predictions.push(DropoffPrediction {
                    scene_index: i,
                    predicted_dropoff_percent: dropoff_percent,
                    reason: if scene.duration > 15.0 {
                        "Scene is too long with low visual density".to_string()
                    } else if scene.layers.is_empty() {
                        "Scene has no visual elements".to_string()
                    } else {
                        "Low momentum detected".to_string()
                    },
                });
            }

            // Check for momentum drops between scenes
            if i > 0 {
                let prev_momentum =
                    Self::calculate_momentum(&script.scenes[i - 1], script.metadata.fps);
                let momentum_drop = prev_momentum - momentum;

                if momentum_drop > 30.0 {
                    predictions.push(DropoffPrediction {
                        scene_index: i,
                        predicted_dropoff_percent: momentum_drop,
                        reason: format!("Sharp momentum drop from scene {} to {}", i, i + 1),
                    });
                }
            }
        }

        predictions
    }

    /// Generate a retention heatmap for the entire script
    pub fn generate_heatmap(script: &VideoScript) -> RetentionHeatmap {
        let mut scene_scores = Vec::new();
        let mut total_retention = 0.0;

        for (i, scene) in script.scenes.iter().enumerate() {
            let momentum = Self::calculate_momentum(scene, script.metadata.fps);

            // Retention score = momentum adjusted for position in video
            // Earlier scenes get a bonus (viewers are fresh)
            let position_factor = 1.0 - (i as f32 / script.scenes.len().max(1) as f32) * 0.3;
            let retention_score = (momentum * position_factor).min(100.0);

            scene_scores.push(SceneRetention {
                scene_index: i,
                momentum,
                retention_score,
            });

            total_retention += retention_score;
        }

        let overall_retention_score = if !scene_scores.is_empty() {
            total_retention / scene_scores.len() as f32
        } else {
            0.0
        };

        // Identify critical moments (bottom 25% retention)
        let threshold = overall_retention_score * 0.75;
        let critical_moments: Vec<usize> = scene_scores
            .iter()
            .filter(|s| s.retention_score < threshold)
            .map(|s| s.scene_index)
            .collect();

        RetentionHeatmap {
            scene_scores,
            overall_retention_score,
            critical_moments,
        }
    }

    /// Export heatmap to CSV format
    pub fn export_csv(heatmap: &RetentionHeatmap) -> String {
        let mut csv = String::from("Scene,Momentum,Retention Score,Critical\n");

        for scene in &heatmap.scene_scores {
            let is_critical = heatmap.critical_moments.contains(&scene.scene_index);
            csv.push_str(&format!(
                "{},{:.2},{:.2},{}\n",
                scene.scene_index + 1,
                scene.momentum,
                scene.retention_score,
                if is_critical { "YES" } else { "NO" }
            ));
        }

        csv.push_str(&format!(
            "\nOverall Retention Score,{:.2}\n",
            heatmap.overall_retention_score
        ));
        csv
    }

    /// Export heatmap to JSON format
    pub fn export_json(heatmap: &RetentionHeatmap) -> String {
        serde_json::to_string_pretty(heatmap).unwrap_or_default()
    }
}

// Derive Serialize for JSON export
use serde::Serialize;
impl Serialize for RetentionHeatmap {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("RetentionHeatmap", 3)?;
        state.serialize_field("scene_scores", &self.scene_scores)?;
        state.serialize_field("overall_retention_score", &self.overall_retention_score)?;
        state.serialize_field("critical_moments", &self.critical_moments)?;
        state.end()
    }
}

impl Serialize for SceneRetention {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("SceneRetention", 3)?;
        state.serialize_field("scene_index", &self.scene_index)?;
        state.serialize_field("momentum", &self.momentum)?;
        state.serialize_field("retention_score", &self.retention_score)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::script::{Layer, Metadata, Resolution, SceneType};

    fn create_test_scene(duration: f32, layer_count: usize) -> Scene {
        let mut layers = Vec::new();
        for i in 0..layer_count {
            layers.push(Layer::Image {
                source: format!("image_{}.png", i).into(),
                effects: vec![],
                transform: Default::default(),
            });
        }

        Scene {
            id: "test".into(),
            scene_type: SceneType::Body,
            duration,
            layers,
            transition: None,
        }
    }

    #[test]
    fn test_calculate_momentum() {
        // High momentum: short duration, many layers
        let scene1 = create_test_scene(3.0, 3);
        let momentum1 = RetentionAnalyzer::calculate_momentum(&scene1, 30);
        assert!(
            momentum1 > 70.0,
            "Short scene with layers should have high momentum"
        );

        // Low momentum: long duration, few layers
        let scene2 = create_test_scene(25.0, 1);
        let momentum2 = RetentionAnalyzer::calculate_momentum(&scene2, 30);
        assert!(
            momentum2 < 40.0,
            "Long scene with few layers should have low momentum"
        );
    }

    #[test]
    fn test_predict_dropoff() {
        let script = VideoScript {
            metadata: Metadata {
                title: "Test".into(),
                resolution: Resolution::Named("1920x1080".into()),
                fps: 30,
                duration: 30.0,
                description: None,
                citations: vec![],
            },
            scenes: vec![
                create_test_scene(5.0, 3),  // Good momentum
                create_test_scene(20.0, 1), // Bad momentum - expect drop-off
            ],
            audio: None,
        };

        let predictions = RetentionAnalyzer::predict_dropoff(&script);
        assert!(
            !predictions.is_empty(),
            "Should predict drop-off for low-momentum scene"
        );
        assert_eq!(
            predictions[0].scene_index, 1,
            "Drop-off should be in scene 2"
        );
    }

    #[test]
    fn test_generate_heatmap() {
        let script = VideoScript {
            metadata: Metadata {
                title: "Test".into(),
                resolution: Resolution::Named("1920x1080".into()),
                fps: 30,
                duration: 20.0,
                description: None,
                citations: vec![],
            },
            scenes: vec![
                create_test_scene(5.0, 3),
                create_test_scene(5.0, 2),
                create_test_scene(10.0, 1),
            ],
            audio: None,
        };

        let heatmap = RetentionAnalyzer::generate_heatmap(&script);
        assert_eq!(heatmap.scene_scores.len(), 3);
        assert!(heatmap.overall_retention_score > 0.0);
    }

    #[test]
    fn test_export_csv() {
        let heatmap = RetentionHeatmap {
            scene_scores: vec![SceneRetention {
                scene_index: 0,
                momentum: 80.0,
                retention_score: 85.0,
            }],
            overall_retention_score: 85.0,
            critical_moments: vec![],
        };

        let csv = RetentionAnalyzer::export_csv(&heatmap);
        assert!(csv.contains("Scene,Momentum,Retention Score"));
        assert!(csv.contains("1,80.00,85.00"));
    }
}
