// ============================================================
// #333 PropBank / FrameNet / VerbNet semantic role parser
//
// All three corpora share the structure of PREDICATE-ROLE-FILLER
// triples rooted in a sentence:
//   "John broke the window with a rock."
//     predicate = "break.01"
//     ARG0  (agent)     → "John"
//     ARG1  (theme)     → "the window"
//     ARGM-MNR (manner) → "with a rock"
//
// This parser consumes JSON records of the shape:
//   {"sentence": "...", "frame": "break.01",
//    "roles": [{"label": "ARG0", "filler": "John"}, ...]}
// and emits one (frame, role, filler) triple per role entry.
//
// FrameNet uses "Agent / Theme / Instrument" instead of ARG0/ARG1/
// ARG2 — the parser preserves whatever label the corpus uses and
// tags the source corpus so downstream can pick the right mapping.
// ============================================================

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct SemanticFrameRecord {
    pub sentence: String,
    pub frame: String,
    #[serde(default)]
    pub roles: Vec<Role>,
    #[serde(default)]
    pub corpus: Option<String>, // "propbank" | "framenet" | "verbnet"
}

#[derive(Debug, Clone, Deserialize)]
pub struct Role {
    pub label: String,
    pub filler: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParsedRoleTuple {
    pub frame: String,
    pub role_label: String,
    pub filler: String,
    pub sentence: String,
    pub source_corpus: String,
}

/// Parse one record into zero or more (frame, role_label, filler)
/// tuples. Empty / invalid rows return Vec::new().
pub fn parse_record(json: &str) -> Result<Vec<ParsedRoleTuple>, serde_json::Error> {
    let record: SemanticFrameRecord = serde_json::from_str(json.trim())?;

    let sentence = record.sentence.trim();
    let frame = record.frame.trim();
    if sentence.is_empty() || frame.is_empty()
        || sentence.len() > 1024 || frame.len() > 128 {
        return Ok(Vec::new());
    }

    let source = record.corpus.as_deref().unwrap_or("unknown").to_string();
    let mut out = Vec::new();
    for role in record.roles.iter().take(16) {
        let label = role.label.trim();
        let filler = role.filler.trim();
        if label.is_empty() || filler.is_empty()
            || label.len() > 32 || filler.len() > 256 {
            continue;
        }
        out.push(ParsedRoleTuple {
            frame: frame.to_string(),
            role_label: label.to_string(),
            filler: filler.to_string(),
            sentence: sentence.to_string(),
            source_corpus: source.clone(),
        });
    }
    Ok(out)
}

/// Canonicalise PropBank numbered args to FrameNet role names.
/// Best-effort: ARG0 → Agent, ARG1 → Theme, ARG2 → Instrument/Recipient
/// depending on frame. Callers that need strict mapping should look
/// up per-frame in the full role lexicon.
pub fn canonical_role(label: &str) -> &'static str {
    match label.trim() {
        "ARG0" | "A0" => "Agent",
        "ARG1" | "A1" => "Theme",
        "ARG2" | "A2" => "Recipient",
        "ARG3" | "A3" => "Source",
        "ARG4" | "A4" => "Benefactive",
        "ARGM-LOC" => "Location",
        "ARGM-TMP" => "Time",
        "ARGM-MNR" => "Manner",
        "ARGM-CAU" => "Cause",
        "ARGM-PRP" => "Purpose",
        _ => "Other",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_propbank_frame() {
        let json = r#"{
            "sentence": "John broke the window.",
            "frame": "break.01",
            "roles": [
                {"label": "ARG0", "filler": "John"},
                {"label": "ARG1", "filler": "the window"}
            ],
            "corpus": "propbank"
        }"#;
        let out = parse_record(json).unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].role_label, "ARG0");
        assert_eq!(out[0].filler, "John");
        assert_eq!(out[1].role_label, "ARG1");
        assert_eq!(out[0].source_corpus, "propbank");
    }

    #[test]
    fn empty_sentence_yields_no_tuples() {
        let json = r#"{"sentence":"","frame":"x.01","roles":[]}"#;
        assert!(parse_record(json).unwrap().is_empty());
    }

    #[test]
    fn skips_oversize_fillers() {
        let long = "x".repeat(300);
        let json = format!(
            r#"{{"sentence":"S","frame":"f.01","roles":[{{"label":"ARG0","filler":"{}"}}]}}"#,
            long
        );
        assert!(parse_record(&json).unwrap().is_empty());
    }

    #[test]
    fn caps_roles_per_record() {
        // 20 roles in → 16 out (hardcoded cap).
        let roles: Vec<String> = (0..20)
            .map(|i| format!(r#"{{"label":"R{}","filler":"f{}"}}"#, i, i))
            .collect();
        let json = format!(
            r#"{{"sentence":"S","frame":"f.01","roles":[{}]}}"#,
            roles.join(",")
        );
        let out = parse_record(&json).unwrap();
        assert_eq!(out.len(), 16);
    }

    #[test]
    fn canonical_role_mapping_core_args() {
        assert_eq!(canonical_role("ARG0"), "Agent");
        assert_eq!(canonical_role("A1"), "Theme");
        assert_eq!(canonical_role("ARGM-LOC"), "Location");
        assert_eq!(canonical_role("unknown"), "Other");
    }
}
