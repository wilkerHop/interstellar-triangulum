use crate::script::VideoScript;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Claim {
    pub text: String,
    pub scene_index: usize,
    pub verified: bool,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct CredibilityReport {
    pub score: u32,
    pub claims: Vec<Claim>,
    pub citations: Vec<String>,
}

pub struct CredibilityAnalyzer;

impl CredibilityAnalyzer {
    pub fn analyze(script: &VideoScript) -> CredibilityReport {
        let claims = Self::detect_claims(script);
        let citations = script.metadata.citations.clone();

        let verified_claims = claims.iter().filter(|c| c.verified).count();
        let total_claims = claims.len();

        let score = if total_claims == 0 {
            100
        } else {
            let base_score = (verified_claims as f32 / total_claims as f32) * 100.0;
            // Penalize for unverified claims
            let unverified_count = total_claims - verified_claims;
            (base_score as i32 - (unverified_count as i32 * 10)).max(0) as u32
        };

        CredibilityReport {
            score,
            claims,
            citations,
        }
    }

    fn detect_claims(script: &VideoScript) -> Vec<Claim> {
        let mut claims = Vec::new();

        // Regex patterns for claim detection
        let stat_regex = Regex::new(r"\d+%|\d+ out of \d+|\d+x faster").unwrap();
        let superlative_regex =
            Regex::new(r"(?i)\b(best|fastest|first|only|proven|guaranteed)\b").unwrap();
        let absolute_regex = Regex::new(r"(?i)\b(always|never|everyone|nobody)\b").unwrap();

        for (i, scene) in script.scenes.iter().enumerate() {
            for layer in &scene.layers {
                if let crate::script::Layer::Text { content, .. } = layer {
                    let mut is_claim = false;
                    let mut reason = String::new();

                    if stat_regex.is_match(content) {
                        is_claim = true;
                        reason = "Contains statistics".to_string();
                    } else if superlative_regex.is_match(content) {
                        is_claim = true;
                        reason = "Uses superlative language".to_string();
                    } else if absolute_regex.is_match(content) {
                        is_claim = true;
                        reason = "Uses absolute terms".to_string();
                    }

                    if is_claim {
                        // Check if this claim is covered by any citation
                        // This is a naive check: if we have citations, we assume claims are verified for now
                        // A more advanced version would link specific citations to specific claims
                        let verified = !script.metadata.citations.is_empty();

                        claims.push(Claim {
                            text: content.clone(),
                            scene_index: i,
                            verified,
                            reason,
                        });
                    }
                }
            }
        }

        claims
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::script::{Layer, Metadata, Resolution, Scene, SceneType};

    fn create_test_script(citations: Vec<String>, text: &str) -> VideoScript {
        VideoScript {
            metadata: Metadata {
                title: "Test".into(),
                resolution: Resolution::Named("1920x1080".into()),
                fps: 30,
                duration: 0.0,
                description: None,
                citations,
            },
            scenes: vec![Scene {
                id: "test".into(),
                duration: 5.0,
                scene_type: SceneType::Body,
                layers: vec![Layer::Text {
                    content: text.into(),
                    font: "font.ttf".into(),
                    font_size: 24.0,
                    color: crate::script::Color {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 255,
                    },
                    position: crate::script::Position { x: 0, y: 0 },
                    effects: vec![],
                }],
                transition: None,
            }],
            audio: None,
        }
    }

    #[test]
    fn test_claim_detection_stats() {
        let script = create_test_script(vec![], "Rust is 10x faster than Python");
        let report = CredibilityAnalyzer::analyze(&script);
        assert_eq!(report.claims.len(), 1);
        assert_eq!(report.claims[0].reason, "Contains statistics");
        assert!(!report.claims[0].verified); // No citations
        assert!(report.score < 100);
    }

    #[test]
    fn test_claim_detection_superlative() {
        let script = create_test_script(vec![], "The best programming language");
        let report = CredibilityAnalyzer::analyze(&script);
        assert_eq!(report.claims.len(), 1);
        assert_eq!(report.claims[0].reason, "Uses superlative language");
    }

    #[test]
    fn test_claim_verification() {
        let script =
            create_test_script(vec!["Benchmark results 2024".into()], "Rust is 10x faster");
        let report = CredibilityAnalyzer::analyze(&script);
        assert_eq!(report.claims.len(), 1);
        assert!(report.claims[0].verified); // Has citations
        assert_eq!(report.score, 100);
    }
}
