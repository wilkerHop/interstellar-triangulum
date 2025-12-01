use crate::analysis::narrative::NarrativeAnalyzer;
use crate::script::VideoScript;

pub struct NarrativeContext;

impl NarrativeContext {
    pub fn run(script: &VideoScript) {
        println!("\n📊 Analyzing Narrative Structure...");
        let report = NarrativeAnalyzer::analyze(script);

        println!("   Score: {}/100", report.score);
        if !report.structure_valid {
            println!("   ❌ Structure Issues:");
            for error in &report.structure_errors {
                println!("      - {}", error);
            }
        } else {
            println!("   ✅ Structure: Valid (Hook -> Body -> Payoff)");
        }

        if !report.pacing_alerts.is_empty() {
            println!("   ⚠️ Pacing Alerts:");
            for alert in &report.pacing_alerts {
                println!("      - {}", alert.message);
            }
        } else {
            println!("   ✅ Pacing: Optimal");
        }

        if !report.retention_warnings.is_empty() {
            println!("   ⚠️ Retention Warnings:");
            for warning in &report.retention_warnings {
                println!("      - {}", warning.message);
            }
        }
    }
}
