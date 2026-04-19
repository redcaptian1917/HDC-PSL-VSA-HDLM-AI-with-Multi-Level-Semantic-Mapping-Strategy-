// ============================================================
// #332 ATOMIC-2020 commonsense if-then parser
//
// ATOMIC-2020 is a knowledge graph of ~1.33M if-then tuples across
// 23 relation types covering social, physical, and event commonsense.
// Format: TSV {head, relation, tail} where head + tail are natural-
// language phrases and relation is one of a fixed set.
//
// Example row:
//   "PersonX starts running\txIntent\tto get fit"
//   → (PersonX starts running) xIntent (to get fit)
//
// This parser normalises relations to canonical predicate names that
// match the rest of the pipeline (facts_tuples + fact_edges).
// ============================================================

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ParsedAtomicTuple {
    pub head: String,
    pub predicate: &'static str,
    pub tail: String,
    /// Original relation name (before canonicalisation) for audit.
    pub raw_relation: String,
}

/// Map ATOMIC-2020 relation tags to canonical predicate names.
/// Returns None for relations we don't capture (the caller skips).
fn predicate_name(rel: &str) -> Option<&'static str> {
    // Social relations (motivation, reaction) — these are internal
    // mental states of the agent.
    match rel {
        "xIntent" | "xWant" => Some("Intends"),
        "xReact" | "oReact" => Some("Reacts"),
        "xNeed" => Some("Needs"),
        "xAttr" => Some("HasAttribute"),
        "xEffect" | "oEffect" => Some("Effect"),

        // Physical / event chains.
        "Causes" | "HinderedBy" => Some("Causes"),
        "isAfter" => Some("IsAfter"),
        "isBefore" => Some("IsBefore"),
        "HasSubEvent" => Some("HasSubevent"),

        // Type / part.
        "IsA" => Some("IsA"),
        "CapableOf" => Some("CapableOf"),
        "UsedFor" => Some("UsedFor"),
        "PartOf" => Some("PartOf"),
        "MadeUpOf" => Some("PartOf"),
        "HasProperty" => Some("HasProperty"),

        _ => None,
    }
}

/// Parse one tab-delimited line. Blank lines / comments (`#`) /
/// malformed rows return Ok(None).
pub fn parse_line(line: &str) -> Result<Option<ParsedAtomicTuple>, &'static str> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return Ok(None);
    }
    let parts: Vec<&str> = trimmed.split('\t').collect();
    if parts.len() < 3 {
        return Ok(None);
    }
    let head = parts[0].trim();
    let relation = parts[1].trim();
    let tail = parts[2].trim();

    // ATOMIC uses "none" as a sentinel for "no filler" on some rows.
    if head.is_empty() || tail.is_empty() || tail == "none" || head == "none" {
        return Ok(None);
    }
    if head.len() > 256 || tail.len() > 256 {
        return Ok(None);
    }

    let Some(pred) = predicate_name(relation) else {
        return Ok(None);
    };

    Ok(Some(ParsedAtomicTuple {
        head: head.to_string(),
        predicate: pred,
        tail: tail.to_string(),
        raw_relation: relation.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_intent_tuple() {
        let line = "PersonX starts running\txIntent\tto get fit";
        let p = parse_line(line).unwrap().unwrap();
        assert_eq!(p.head, "PersonX starts running");
        assert_eq!(p.predicate, "Intends");
        assert_eq!(p.tail, "to get fit");
        assert_eq!(p.raw_relation, "xIntent");
    }

    #[test]
    fn parses_causes_tuple() {
        let line = "rain\tCauses\tflooding";
        let p = parse_line(line).unwrap().unwrap();
        assert_eq!(p.predicate, "Causes");
    }

    #[test]
    fn skips_none_fillers() {
        let line = "X\txIntent\tnone";
        assert!(parse_line(line).unwrap().is_none());
    }

    #[test]
    fn skips_blank_and_comments() {
        assert!(parse_line("").unwrap().is_none());
        assert!(parse_line("# comment line").unwrap().is_none());
    }

    #[test]
    fn skips_unknown_relation() {
        let line = "X\tSomeWeirdRel\tY";
        assert!(parse_line(line).unwrap().is_none());
    }

    #[test]
    fn skips_short_rows() {
        assert!(parse_line("just one col").unwrap().is_none());
        assert!(parse_line("two\tcols").unwrap().is_none());
    }

    #[test]
    fn caps_oversize_phrases() {
        let huge = "x".repeat(300);
        let line = format!("{}\txIntent\t{}", huge, huge);
        assert!(parse_line(&line).unwrap().is_none());
    }
}
