// ============================================================
// #182 Lean4 proof-carrying inference adapter
//
// Kimina-style: LFI issues a conclusion, ships a proof obligation
// (Lean4 tactic term) to a Lean4 server, gets back {valid, errors}.
// When valid, the conclusion is tagged proof_verified in the
// provenance chain. When not, the conclusion is downgraded from
// Proof tier to Consensus or lower in the CARTA trust ladder.
//
// This module is the wire protocol + adapter. A running Lean4 server
// is out-of-scope for this commit — we target the Kimina REST shape
// so the adapter swaps between localhost:8090 (dev) and a remote
// verifier without further work.
//
// Wire protocol (Kimina-compatible):
//   POST /verify
//   body: { lean_code: "theorem ...", timeout_ms: 5000 }
//   response: { valid: bool, errors: [string], proof_hash: string }
// ============================================================

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub struct ProofRequest {
    pub lean_code: String,
    #[serde(default)]
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ProofResponse {
    pub valid: bool,
    #[serde(default)]
    pub errors: Vec<String>,
    #[serde(default)]
    pub proof_hash: String,
}

/// Client config — pointed at a running Lean4/Kimina server.
/// Default: localhost:8090.
#[derive(Debug, Clone)]
pub struct Lean4Client {
    pub base_url: String,
    pub timeout_ms: u64,
}

impl Default for Lean4Client {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:8090".to_string(),
            timeout_ms: 5_000,
        }
    }
}

impl Lean4Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into(), ..Default::default() }
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    /// Synchronous verify — blocks up to `timeout_ms`. Returns the
    /// server's verdict or a transport error.
    ///
    /// BUG ASSUMPTION: server may be unreachable on every LFI install
    /// (it's an optional dependency). Callers should treat
    /// `Err(LeanClientError::Transport)` as "unknown" — neither
    /// accept nor reject the conclusion based on a missing verifier.
    pub fn verify(&self, lean_code: &str) -> Result<ProofResponse, LeanClientError> {
        if lean_code.is_empty() || lean_code.len() > 65_536 {
            return Err(LeanClientError::InvalidRequest(
                "lean_code must be 1..=65536 chars".into(),
            ));
        }
        let body = ProofRequest {
            lean_code: lean_code.to_string(),
            timeout_ms: self.timeout_ms,
        };
        let url = format!("{}/verify", self.base_url.trim_end_matches('/'));
        match ureq::post(&url)
            .timeout(std::time::Duration::from_millis(self.timeout_ms + 500))
            .send_json(serde_json::to_value(&body)
                .map_err(|e| LeanClientError::InvalidRequest(e.to_string()))?)
        {
            Ok(resp) => resp.into_json::<ProofResponse>()
                .map_err(|e| LeanClientError::MalformedResponse(e.to_string())),
            Err(ureq::Error::Status(code, _)) =>
                Err(LeanClientError::ServerError(code)),
            Err(e) => Err(LeanClientError::Transport(e.to_string())),
        }
    }

    /// Verify and classify the failure, if any. Callers typically want
    /// to know whether the conclusion is fully proven, conditionally
    /// proven (timeout), or rejected.
    pub fn classify(&self, lean_code: &str) -> VerifyVerdict {
        match self.verify(lean_code) {
            Ok(resp) if resp.valid => VerifyVerdict::Proved {
                proof_hash: resp.proof_hash,
            },
            Ok(resp) => VerifyVerdict::Rejected { errors: resp.errors },
            Err(LeanClientError::Transport(e)) => VerifyVerdict::Unreachable { detail: e },
            Err(err) => VerifyVerdict::Error { detail: err.to_string() },
        }
    }
}

/// Rich classification of a verify attempt.
#[derive(Debug, Clone, PartialEq)]
pub enum VerifyVerdict {
    Proved { proof_hash: String },
    Rejected { errors: Vec<String> },
    Unreachable { detail: String },
    Error { detail: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum LeanClientError {
    InvalidRequest(String),
    Transport(String),
    ServerError(u16),
    MalformedResponse(String),
}

impl std::fmt::Display for LeanClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRequest(s) => write!(f, "invalid request: {}", s),
            Self::Transport(s) => write!(f, "transport: {}", s),
            Self::ServerError(c) => write!(f, "server HTTP {}", c),
            Self::MalformedResponse(s) => write!(f, "malformed response: {}", s),
        }
    }
}

impl std::error::Error for LeanClientError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_serialises_with_lean_code_and_timeout() {
        let req = ProofRequest { lean_code: "example: 1 + 1 = 2 := by ring".into(), timeout_ms: 3000 };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("lean_code"));
        assert!(json.contains("timeout_ms"));
        assert!(json.contains("ring"));
    }

    #[test]
    fn response_deserialises_valid_shape() {
        let body = r#"{"valid":true,"errors":[],"proof_hash":"abc123"}"#;
        let r: ProofResponse = serde_json::from_str(body).unwrap();
        assert!(r.valid);
        assert_eq!(r.proof_hash, "abc123");
        assert!(r.errors.is_empty());
    }

    #[test]
    fn response_with_missing_optional_fields() {
        // Server could omit errors / proof_hash on the happy path.
        let body = r#"{"valid":true}"#;
        let r: ProofResponse = serde_json::from_str(body).unwrap();
        assert!(r.valid);
        assert_eq!(r.proof_hash, "");
        assert!(r.errors.is_empty());
    }

    #[test]
    fn verify_rejects_empty_or_oversize_input() {
        let client = Lean4Client::default();
        match client.verify("") {
            Err(LeanClientError::InvalidRequest(_)) => {}
            other => panic!("expected InvalidRequest for empty, got {:?}", other),
        }
        let huge = "x".repeat(65_537);
        match client.verify(&huge) {
            Err(LeanClientError::InvalidRequest(_)) => {}
            other => panic!("expected InvalidRequest for oversize, got {:?}", other),
        }
    }

    #[test]
    fn verify_classifies_unreachable_server() {
        // Point at a port nothing is listening on; verdict should
        // be Unreachable rather than Rejected/Error.
        let client = Lean4Client::new("http://127.0.0.1:1")
            .with_timeout(100);
        let verdict = client.classify("example: True := trivial");
        match verdict {
            VerifyVerdict::Unreachable { .. } => {}
            other => panic!("expected Unreachable, got {:?}", other),
        }
    }

    #[test]
    fn error_display_is_informative() {
        let e = LeanClientError::ServerError(500);
        assert!(format!("{}", e).contains("500"));
        let e = LeanClientError::Transport("connection refused".into());
        assert!(format!("{}", e).contains("connection refused"));
    }
}
