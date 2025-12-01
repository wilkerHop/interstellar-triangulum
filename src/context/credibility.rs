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
        } else {
            println!("   ⚠️  No citations provided");
        }

        println!("\n   ✅ Quality Checklist:");
        for item in &cred_report.checklist {
            let icon = if item.passed { "✓" } else { "❌" };
            println!("      {} [{}] {}", icon, item.category, item.message);
        }
    }
}
