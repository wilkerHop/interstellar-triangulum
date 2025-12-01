use crate::analysis::credibility::CredibilityAnalyzer;
use crate::script::VideoScript;

pub struct CredibilityContext;

impl CredibilityContext {
    pub fn run(script: &VideoScript) {
        println!("\n🛡️ Analyzing Credibility...");
        let cred_report = CredibilityAnalyzer::analyze(script);

        println!("   Score: {}/100", cred_report.score);
        if cred_report.claims.is_empty() {
            println!("   ✅ No claims detected.");
        } else {
            println!("   🔍 Detected {} claims:", cred_report.claims.len());
            for claim in &cred_report.claims {
                let status = if claim.verified {
                    "✅ Verified"
                } else {
                    "❌ Unverified"
                };
                println!("      - [{}] \"{}\" ({})", status, claim.text, claim.reason);
            }
        }

        if !cred_report.citations.is_empty() {
            println!("   📚 Citations:");
            for citation in &cred_report.citations {
                println!("      - {}", citation);
            }
        } else if !cred_report.claims.is_empty() {
            println!("   ⚠️ No citations provided for detected claims.");
        }
    }
}
