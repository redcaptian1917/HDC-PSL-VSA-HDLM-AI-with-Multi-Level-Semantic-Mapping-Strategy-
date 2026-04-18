// ============================================================
// Speech-Act Classifier — Layer 5 prototype matcher (#345)
//
// Per LFI_SUPERSOCIETY_ARCHITECTURE.md §Layer-5:
//   "Speech-act classification: incoming utterance hypervector
//    classified via prototype matching into speech acts (question,
//    statement, request, clarification, acknowledgment,
//    refusal-of-premise, etc.)"
//
// Post-LLM. No ML classifier. Just prototype hypervectors bundled
// from labeled example utterances, cosine similarity at query time.
// The labels come from the rule-based intent decomposer (#329) that
// populates source='dialogue_tuples_v1' facts via
// scripts/ingest_conversational_tuples.py.
//
// Capability bound (spec Substrate I): at D=10,000, ~400 items can be
// safely bundled into one prototype before interference degrades
// retrieval. We sample N per intent (default 400) to stay inside that
// bound. More than that and the trimmed-mean / tier-weighted voted
// bundle (Critical Fix C2) would be needed; that's follow-up work.
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdc::role_binding::concept_vector;
use crate::persistence::BrainDb;
use std::collections::HashMap;

/// Bag-of-words hypervector from the first N tokens.
///
/// Whole-text hashing via concept_vector(text) would produce orthogonal
/// vectors for "what is water" vs "what is fire" — their common "what is"
/// prefix is lost. Bundling per-word vectors preserves repeated leading
/// tokens, which is exactly the signal speech-act classification depends
/// on ("what is" → define, "how do" → how_to, "why does" → why).
///
/// The first 6 tokens carry most of the speech-act signal; beyond that,
/// topical content dominates and swamps the intent cue.
fn bag_of_words_prefix(text: &str, max_tokens: usize) -> Option<BipolarVector> {
    let mut hvs: Vec<BipolarVector> = text
        .split_whitespace()
        .take(max_tokens)
        .map(|w| w.to_lowercase())
        .filter(|w| !w.is_empty())
        .map(|w| concept_vector(&w))
        .collect();
    if hvs.is_empty() {
        return None;
    }
    // Stable clone-free view for bundle().
    let refs: Vec<&BipolarVector> = hvs.iter().collect();
    BipolarVector::bundle(&refs).ok().or_else(|| hvs.pop())
}

/// Number of leading tokens bundled into the bag-of-words vector.
const PREFIX_TOKENS: usize = 6;

/// Speech-act labels matching the intent decomposer's output. Keep
/// in sync with scripts/ingest_conversational_tuples.py::extract_intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpeechAct {
    Define,
    Explain,
    HowTo,
    Why,
    WhQuestion,
    WhoQuestion,
    Compare,
    Enumerate,
    Generate,
    Summarize,
    Translate,
    Fix,
    Improve,
    Analyze,
    Question,
    Statement,
    Unknown,
}

impl SpeechAct {
    /// Parse the string label used by the dialogue-tuple ingester.
    pub fn from_label(s: &str) -> Self {
        match s {
            "define" => SpeechAct::Define,
            "explain" => SpeechAct::Explain,
            "how_to" => SpeechAct::HowTo,
            "why" => SpeechAct::Why,
            "wh_question" => SpeechAct::WhQuestion,
            "who_question" => SpeechAct::WhoQuestion,
            "compare" => SpeechAct::Compare,
            "enumerate" => SpeechAct::Enumerate,
            "generate" => SpeechAct::Generate,
            "summarize" => SpeechAct::Summarize,
            "translate" => SpeechAct::Translate,
            "fix" => SpeechAct::Fix,
            "improve" => SpeechAct::Improve,
            "analyze" => SpeechAct::Analyze,
            "question" => SpeechAct::Question,
            "statement" => SpeechAct::Statement,
            _ => SpeechAct::Unknown,
        }
    }

    pub fn as_label(self) -> &'static str {
        match self {
            SpeechAct::Define => "define",
            SpeechAct::Explain => "explain",
            SpeechAct::HowTo => "how_to",
            SpeechAct::Why => "why",
            SpeechAct::WhQuestion => "wh_question",
            SpeechAct::WhoQuestion => "who_question",
            SpeechAct::Compare => "compare",
            SpeechAct::Enumerate => "enumerate",
            SpeechAct::Generate => "generate",
            SpeechAct::Summarize => "summarize",
            SpeechAct::Translate => "translate",
            SpeechAct::Fix => "fix",
            SpeechAct::Improve => "improve",
            SpeechAct::Analyze => "analyze",
            SpeechAct::Question => "question",
            SpeechAct::Statement => "statement",
            SpeechAct::Unknown => "unknown",
        }
    }
}

/// A prototype-based speech-act classifier. Holds one BipolarVector
/// per SpeechAct, each built by bundling a sample of labeled user
/// utterances from the dialogue_tuples_v1 corpus.
pub struct SpeechActClassifier {
    prototypes: HashMap<SpeechAct, BipolarVector>,
    per_intent_samples: usize,
}

impl SpeechActClassifier {
    /// Build prototype hypervectors from the dialogue tuples in brain.db.
    ///
    /// Query: for each intent label L, sample up to `per_intent` user
    /// utterances from `convo_X` rows where (convo_X, user_intent, L)
    /// and join to (convo_X, has_utterance_user, <text>). Vectorize
    /// each utterance via concept_vector and bundle them.
    ///
    /// Returns a classifier containing one prototype per observed
    /// intent. Missing intents fall back to random-but-deterministic
    /// concept_vector("speech_act::<label>") so classify never panics.
    pub fn build_from_db(db: &BrainDb, per_intent: usize) -> Self {
        let mut prototypes: HashMap<SpeechAct, BipolarVector> = HashMap::new();
        let conn = db.conn.lock().unwrap_or_else(|e| e.into_inner());

        let known_labels = [
            "define", "explain", "how_to", "why", "wh_question", "who_question",
            "compare", "enumerate", "generate", "summarize", "translate",
            "fix", "improve", "analyze", "question", "statement",
        ];

        for label in known_labels {
            // Gather up to per_intent user utterances for this intent.
            let sql = "SELECT u.value FROM facts i \
                       JOIN facts u ON u.key = replace(i.key, '::user_intent', '::has_utterance_user') \
                       WHERE i.source='dialogue_tuples_v1' \
                         AND i.key LIKE '%::user_intent' \
                         AND i.value = ?1 \
                         AND u.source='dialogue_tuples_v1' \
                       LIMIT ?2";
            let mut stmt = match conn.prepare(sql) {
                Ok(s) => s,
                Err(_) => continue,
            };
            let rows: Vec<String> = stmt
                .query_map(
                    rusqlite::params![label, per_intent as i64],
                    |row| row.get::<_, String>(0),
                )
                .map(|iter| iter.filter_map(|r| r.ok()).collect())
                .unwrap_or_default();

            if rows.is_empty() {
                // No examples — seed a deterministic prototype so the
                // classifier is still total.
                prototypes.insert(
                    SpeechAct::from_label(label),
                    concept_vector(&format!("speech_act::{}", label)),
                );
                continue;
            }

            // Vectorize each utterance via bag-of-words over its prefix,
            // then bundle. This preserves the leading-token signal that
            // separates speech acts.
            let hvs: Vec<BipolarVector> = rows
                .iter()
                .filter_map(|t| bag_of_words_prefix(t, PREFIX_TOKENS))
                .collect();
            if hvs.is_empty() {
                prototypes.insert(
                    SpeechAct::from_label(label),
                    concept_vector(&format!("speech_act::{}", label)),
                );
                continue;
            }
            let refs: Vec<&BipolarVector> = hvs.iter().collect();
            let proto = match BipolarVector::bundle(&refs) {
                Ok(v) => v,
                Err(_) => concept_vector(&format!("speech_act::{}", label)),
            };
            prototypes.insert(SpeechAct::from_label(label), proto);
        }

        Self { prototypes, per_intent_samples: per_intent }
    }

    /// Build a classifier with deterministic placeholder prototypes —
    /// used when the DB hasn't been ingested yet (fresh installs, tests).
    /// Every intent gets a stable prototype derived from its label.
    pub fn placeholder() -> Self {
        let mut prototypes = HashMap::new();
        let labels = [
            "define", "explain", "how_to", "why", "wh_question", "who_question",
            "compare", "enumerate", "generate", "summarize", "translate",
            "fix", "improve", "analyze", "question", "statement",
        ];
        for l in labels {
            prototypes.insert(
                SpeechAct::from_label(l),
                concept_vector(&format!("speech_act::{}", l)),
            );
        }
        Self { prototypes, per_intent_samples: 0 }
    }

    /// Number of labeled examples bundled per intent.
    pub fn sample_size(&self) -> usize { self.per_intent_samples }

    /// Total prototypes. 16 if all intents seen, less otherwise (but
    /// build_from_db always fills 16 via placeholder fallback).
    pub fn prototype_count(&self) -> usize { self.prototypes.len() }

    /// Classify an input text by cosine against every prototype.
    /// Returns (predicted_act, similarity_score). Score is in [-1, 1];
    /// ≥ 0.30 is a confident classification at D=10,000 bundled from
    /// ≥ 100 examples per intent (empirically — random baseline ~0).
    pub fn classify(&self, text: &str) -> (SpeechAct, f64) {
        let probe = match bag_of_words_prefix(text, PREFIX_TOKENS) {
            Some(v) => v,
            None => return (SpeechAct::Unknown, 0.0),
        };
        let mut best = (SpeechAct::Unknown, f64::NEG_INFINITY);
        for (&act, proto) in &self.prototypes {
            let sim = probe.similarity(proto).unwrap_or(f64::NEG_INFINITY);
            if sim > best.1 {
                best = (act, sim);
            }
        }
        best
    }

    /// Classify and require confidence above `threshold`, else Unknown.
    /// Use to gate routing on classifications the model isn't sure about.
    pub fn classify_or_unknown(&self, text: &str, threshold: f64) -> (SpeechAct, f64) {
        let (act, score) = self.classify(text);
        if score >= threshold { (act, score) } else { (SpeechAct::Unknown, score) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_roundtrip() {
        for l in ["define", "explain", "how_to", "why", "question", "statement", "fix"] {
            let act = SpeechAct::from_label(l);
            assert_eq!(act.as_label(), l, "roundtrip failed for {}", l);
        }
    }

    #[test]
    fn placeholder_classifier_is_total() {
        // A placeholder classifier has no DB — every intent gets a
        // seed prototype so classify() always returns something.
        let clf = SpeechActClassifier::placeholder();
        assert_eq!(clf.prototype_count(), 16);
        let (_act, score) = clf.classify("what is water");
        assert!(score.is_finite(), "classify returned non-finite score");
    }

    #[test]
    fn unknown_returned_below_threshold() {
        let clf = SpeechActClassifier::placeholder();
        // With placeholder prototypes (random concept_vectors per label),
        // an arbitrary query's similarity to all of them is near zero.
        // Raising threshold to 0.5 should force Unknown.
        let (act, _score) = clf.classify_or_unknown("foo bar baz", 0.5);
        assert_eq!(act, SpeechAct::Unknown);
    }

    #[test]
    fn prototypes_are_distinct() {
        // Different intents must produce different prototypes, else the
        // classifier is degenerate. With placeholder mode this is a
        // property test on concept_vector's namespace separation.
        let clf = SpeechActClassifier::placeholder();
        let p_define = clf.prototypes[&SpeechAct::Define].clone();
        let p_fix = clf.prototypes[&SpeechAct::Fix].clone();
        assert_ne!(p_define.data, p_fix.data,
            "define and fix prototypes collapsed to same hypervector");
    }
}
