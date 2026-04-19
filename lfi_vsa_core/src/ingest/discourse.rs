// ============================================================
// #334 Penn Discourse Treebank / RST ingestor
//
// Both PDT and RST-DT ship as tab / pipe-delimited files of the form:
//   arg1 | relation | arg2 [ | source ]
// where arg1 and arg2 are text spans (sentences / clauses) and
// relation is one of a shared label set: Contrast, Cause, Result,
// Elaboration, Enablement, Condition, Temporal-Before,
// Temporal-After, Concession, Conjunction, etc.
//
// This parser canonicalises relation names to the same predicate
// vocabulary used by ATOMIC / CauseNet / Wikidata so downstream
// consumers see a unified namespace.
// ============================================================

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParsedDiscourseTuple {
    pub arg1: String,
    pub predicate: &'static str,
    pub arg2: String,
    pub raw_relation: String,
    pub source_corpus: Option<String>,
}

fn predicate_name(rel: &str) -> Option<&'static str> {
    // Case-insensitive match on the raw label.
    let r = rel.trim().to_ascii_lowercase();
    Some(match r.as_str() {
        "cause" | "result" | "nonvolitional-cause" | "volitional-cause"
            | "nonvolitional-result" | "volitional-result"
            | "cause.reason" | "cause.result" => "Causes",
        "contrast" | "concession" | "antithesis" | "otherwise"
            | "comparison.contrast" | "comparison.concession"
            => "Contrast",
        "elaboration" | "elaboration-general-specific"
            | "elaboration-set-member" | "elaboration-object-attribute"
            | "expansion.elaboration" | "expansion.restatement"
            => "Elaboration",
        "enablement" | "purpose" | "contingency.condition"
            => "Enables",
        "condition" | "hypothetical" | "contingency.cause"
            => "Condition",
        "temporal-before" | "temporal.asynchronous.precedence"
            | "temporal.before"
            => "IsBefore",
        "temporal-after" | "temporal.asynchronous.succession"
            | "temporal.after"
            => "IsAfter",
        "conjunction" | "list" | "joint" | "expansion.conjunction"
            => "Conjunction",
        "background" => "Background",
        "summary" => "Summary",
        "evidence" | "justify" => "Evidence",
        "manner" | "circumstance" => "Manner",
        _ => return None,
    })
}

/// Parse one PIPE-delimited line (` | ` separator) OR one TSV line
/// (`\t` separator) — both are common in discourse corpus exports.
pub fn parse_line(line: &str) -> Option<ParsedDiscourseTuple> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    // Try pipe-delimited first, then TSV.
    let parts: Vec<&str> = if trimmed.contains(" | ") {
        trimmed.split(" | ").collect()
    } else {
        trimmed.split('\t').collect()
    };

    if parts.len() < 3 { return None; }

    let arg1 = parts[0].trim();
    let relation = parts[1].trim();
    let arg2 = parts[2].trim();
    let source_corpus = parts.get(3)
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if arg1.is_empty() || arg2.is_empty()
        || arg1.len() > 512 || arg2.len() > 512 {
        return None;
    }

    let pred = predicate_name(relation)?;
    Some(ParsedDiscourseTuple {
        arg1: arg1.to_string(),
        predicate: pred,
        arg2: arg2.to_string(),
        raw_relation: relation.to_string(),
        source_corpus,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pipe_delimited_cause() {
        let line = "The rain started. | Cause | The ground became wet.";
        let p = parse_line(line).unwrap();
        assert_eq!(p.predicate, "Causes");
        assert_eq!(p.arg1, "The rain started.");
        assert_eq!(p.arg2, "The ground became wet.");
    }

    #[test]
    fn parses_tsv_contrast() {
        let line = "She studied hard.\tContrast\tShe still failed.";
        let p = parse_line(line).unwrap();
        assert_eq!(p.predicate, "Contrast");
    }

    #[test]
    fn captures_source_corpus_when_present() {
        let line = "X | Elaboration | Y | pdt";
        let p = parse_line(line).unwrap();
        assert_eq!(p.source_corpus.as_deref(), Some("pdt"));
    }

    #[test]
    fn case_insensitive_relation_match() {
        let line = "A | CAUSE | B";
        let p = parse_line(line).unwrap();
        assert_eq!(p.predicate, "Causes");
    }

    #[test]
    fn maps_compound_labels() {
        let line = "A | Comparison.Contrast | B";
        let p = parse_line(line).unwrap();
        assert_eq!(p.predicate, "Contrast");
    }

    #[test]
    fn skips_blank_and_comments() {
        assert!(parse_line("").is_none());
        assert!(parse_line("# comment").is_none());
    }

    #[test]
    fn skips_unknown_relation() {
        assert!(parse_line("A | FooBar | B").is_none());
    }
}
