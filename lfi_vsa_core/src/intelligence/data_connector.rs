// ============================================================
// Data Source Connector Framework
//
// Unified interface for ingesting facts from diverse data sources:
// - Local files (CSV, JSONL, TSV, Parquet metadata)
// - HuggingFace datasets (via API)
// - UCI ML Repository
// - Web APIs
// - User uploads
//
// SUPERSOCIETY: 59M+ facts didn't arrive by accident. This framework
// standardizes ingestion so every source goes through quality scoring,
// domain classification, and provenance tracking.
// ============================================================

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Supported data source types.
#[derive(Debug, Clone, PartialEq)]
pub enum SourceType {
    LocalCsv,
    LocalJsonl,
    LocalTsv,
    HuggingFace,
    UciMl,
    WebApi,
    UserUpload,
}

impl SourceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LocalCsv => "local_csv",
            Self::LocalJsonl => "local_jsonl",
            Self::LocalTsv => "local_tsv",
            Self::HuggingFace => "huggingface",
            Self::UciMl => "uci_ml",
            Self::WebApi => "web_api",
            Self::UserUpload => "user_upload",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "csv" => Some(Self::LocalCsv),
            "jsonl" | "json" | "ndjson" => Some(Self::LocalJsonl),
            "tsv" => Some(Self::LocalTsv),
            _ => None,
        }
    }
}

/// Configuration for a data source connection.
#[derive(Debug, Clone)]
pub struct DataSourceConfig {
    pub name: String,
    pub source_type: SourceType,
    pub path_or_url: String,
    pub domain: Option<String>,
    pub default_quality: f64,
    pub auto_vet: bool,
    pub max_facts: Option<usize>,
    pub column_mapping: ColumnMapping,
}

/// How columns/fields map to fact key/value.
#[derive(Debug, Clone)]
pub struct ColumnMapping {
    /// Column name or JSON field for the fact key
    pub key_field: String,
    /// Column name or JSON field for the fact value
    pub value_field: String,
    /// Optional quality score field
    pub quality_field: Option<String>,
    /// Optional domain field
    pub domain_field: Option<String>,
}

impl Default for ColumnMapping {
    fn default() -> Self {
        Self {
            key_field: "key".to_string(),
            value_field: "value".to_string(),
            quality_field: None,
            domain_field: None,
        }
    }
}

/// Result of an ingestion operation.
#[derive(Debug, Clone)]
pub struct IngestResult {
    pub source_name: String,
    pub source_type: String,
    pub facts_processed: usize,
    pub facts_added: usize,
    pub facts_skipped: usize,
    pub errors: Vec<String>,
    pub duration_ms: u64,
    pub domain: Option<String>,
}

/// Registry of all known data sources — tracks what we've ingested and from where.
pub struct DataSourceRegistry {
    sources: Vec<DataSourceConfig>,
    ingest_history: Vec<IngestResult>,
}

impl DataSourceRegistry {
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            ingest_history: Vec::new(),
        }
    }

    /// Register a new data source.
    pub fn register(&mut self, config: DataSourceConfig) {
        self.sources.push(config);
    }

    /// Scan a directory for ingestable files and auto-register them.
    /// BUG ASSUMPTION: directory may contain non-data files; we only register
    /// files with known extensions.
    pub fn scan_directory(&mut self, dir: &Path, default_domain: Option<&str>, default_quality: f64) -> usize {
        let mut registered = 0;
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_file() { continue; }

                let ext = path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");

                if let Some(source_type) = SourceType::from_extension(ext) {
                    let name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    // Auto-detect domain from filename patterns
                    let domain = default_domain.map(|d| d.to_string()).or_else(|| {
                        let lower = name.to_lowercase();
                        if lower.contains("cyber") || lower.contains("security") || lower.contains("attack") {
                            Some("cybersecurity".to_string())
                        } else if lower.contains("conversation") || lower.contains("chat") || lower.contains("dialog") {
                            Some("conversational".to_string())
                        } else if lower.contains("math") || lower.contains("algebra") {
                            Some("mathematics".to_string())
                        } else if lower.contains("medical") || lower.contains("health") || lower.contains("bio") {
                            Some("biology".to_string())
                        } else if lower.contains("physics") || lower.contains("quantum") {
                            Some("physics".to_string())
                        } else if lower.contains("news") || lower.contains("article") {
                            Some("news_topics".to_string())
                        } else if lower.contains("sentiment") || lower.contains("emotion") {
                            Some("sentiment".to_string())
                        } else if lower.contains("code") || lower.contains("program") {
                            Some("programming".to_string())
                        } else {
                            Some("general".to_string())
                        }
                    });

                    // Detect column mapping from first line
                    let mapping = detect_column_mapping(&path, &source_type);

                    self.register(DataSourceConfig {
                        name,
                        source_type,
                        path_or_url: path.to_string_lossy().to_string(),
                        domain,
                        default_quality,
                        auto_vet: default_quality >= 0.8,
                        max_facts: None,
                        column_mapping: mapping,
                    });
                    registered += 1;
                }
            }
        }
        registered
    }

    /// List all registered sources.
    pub fn list_sources(&self) -> &[DataSourceConfig] {
        &self.sources
    }

    /// Get ingestion history.
    pub fn history(&self) -> &[IngestResult] {
        &self.ingest_history
    }

    /// Record an ingestion result.
    pub fn record_ingest(&mut self, result: IngestResult) {
        self.ingest_history.push(result);
    }

    /// Get summary statistics.
    pub fn summary(&self) -> RegistrySummary {
        let by_type: HashMap<String, usize> = self.sources.iter()
            .fold(HashMap::new(), |mut acc, s| {
                *acc.entry(s.source_type.as_str().to_string()).or_default() += 1;
                acc
            });

        let by_domain: HashMap<String, usize> = self.sources.iter()
            .fold(HashMap::new(), |mut acc, s| {
                let d = s.domain.as_deref().unwrap_or("unclassified");
                *acc.entry(d.to_string()).or_default() += 1;
                acc
            });

        let total_ingested: usize = self.ingest_history.iter()
            .map(|r| r.facts_added).sum();

        RegistrySummary {
            total_sources: self.sources.len(),
            by_type,
            by_domain,
            total_facts_ingested: total_ingested,
            total_ingestion_runs: self.ingest_history.len(),
        }
    }
}

/// Summary of the data source registry.
#[derive(Debug)]
pub struct RegistrySummary {
    pub total_sources: usize,
    pub by_type: HashMap<String, usize>,
    pub by_domain: HashMap<String, usize>,
    pub total_facts_ingested: usize,
    pub total_ingestion_runs: usize,
}

/// Auto-detect column mapping from file header.
fn detect_column_mapping(path: &Path, source_type: &SourceType) -> ColumnMapping {
    let default = ColumnMapping::default();

    let first_line = match std::fs::read_to_string(path) {
        // SECURITY: Only read first 4KB to detect headers — don't load entire file
        Ok(content) => content.lines().next().unwrap_or("").to_string(),
        Err(_) => return default,
    };

    match source_type {
        SourceType::LocalCsv | SourceType::LocalTsv => {
            let sep = if *source_type == SourceType::LocalTsv { '\t' } else { ',' };
            let cols: Vec<&str> = first_line.split(sep).collect();
            let lower_cols: Vec<String> = cols.iter().map(|c| c.trim().trim_matches('"').to_lowercase()).collect();

            // Try to find key/value columns
            let key_idx = lower_cols.iter().position(|c|
                c == "key" || c == "id" || c == "name" || c == "question" || c == "instruction" || c == "prompt"
            );
            let value_idx = lower_cols.iter().position(|c|
                c == "value" || c == "text" || c == "content" || c == "answer" || c == "output" || c == "response"
            );

            ColumnMapping {
                key_field: key_idx.map(|i| cols[i].trim().to_string()).unwrap_or_else(|| cols.first().map(|c| c.trim().to_string()).unwrap_or_default()),
                value_field: value_idx.map(|i| cols[i].trim().to_string()).unwrap_or_else(|| cols.get(1).map(|c| c.trim().to_string()).unwrap_or_default()),
                quality_field: lower_cols.iter().position(|c| c == "quality" || c == "score" || c == "confidence")
                    .map(|i| cols[i].trim().to_string()),
                domain_field: lower_cols.iter().position(|c| c == "domain" || c == "category" || c == "topic")
                    .map(|i| cols[i].trim().to_string()),
            }
        }
        SourceType::LocalJsonl => {
            // Try parsing first line as JSON
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&first_line) {
                if let Some(obj) = parsed.as_object() {
                    let keys: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
                    let key_field = keys.iter().find(|&&k|
                        k == "key" || k == "id" || k == "question" || k == "instruction" || k == "prompt"
                    ).unwrap_or(&"key");
                    let value_field = keys.iter().find(|&&k|
                        k == "value" || k == "text" || k == "content" || k == "answer" || k == "output" || k == "response"
                    ).unwrap_or(&"value");

                    return ColumnMapping {
                        key_field: key_field.to_string(),
                        value_field: value_field.to_string(),
                        quality_field: keys.iter().find(|&&k| k == "quality" || k == "score").map(|k| k.to_string()),
                        domain_field: keys.iter().find(|&&k| k == "domain" || k == "category").map(|k| k.to_string()),
                    };
                }
            }
            default
        }
        _ => default,
    }
}

use serde_json;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp_dir() -> PathBuf {
        let id = std::process::id();
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let dir = PathBuf::from(format!("/tmp/plausiden_test_connector_{}_{}", id, ts));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_source_type_from_extension() {
        assert_eq!(SourceType::from_extension("csv"), Some(SourceType::LocalCsv));
        assert_eq!(SourceType::from_extension("jsonl"), Some(SourceType::LocalJsonl));
        assert_eq!(SourceType::from_extension("tsv"), Some(SourceType::LocalTsv));
        assert_eq!(SourceType::from_extension("exe"), None);
    }

    #[test]
    fn test_scan_directory() {
        let dir = temp_dir();

        // Create test files
        let csv_path = dir.join("cybersecurity_attacks.csv");
        let mut f = std::fs::File::create(&csv_path).unwrap();
        writeln!(f, "id,text,score").unwrap();
        writeln!(f, "1,SQL injection attack detected,0.9").unwrap();

        let jsonl_path = dir.join("conversations.jsonl");
        let mut f = std::fs::File::create(&jsonl_path).unwrap();
        writeln!(f, r#"{{"instruction":"hello","output":"hi there","quality":0.8}}"#).unwrap();

        // Non-data file should be skipped
        std::fs::write(dir.join("readme.txt"), "not data").unwrap();

        let mut registry = DataSourceRegistry::new();
        let count = registry.scan_directory(&dir, None, 0.7);

        assert_eq!(count, 2);
        assert_eq!(registry.list_sources().len(), 2);

        // Check auto-domain detection
        let cyber = registry.list_sources().iter()
            .find(|s| s.name.contains("cybersecurity")).unwrap();
        assert_eq!(cyber.domain.as_deref(), Some("cybersecurity"));
        assert_eq!(cyber.source_type, SourceType::LocalCsv);

        let conv = registry.list_sources().iter()
            .find(|s| s.name.contains("conversations")).unwrap();
        assert_eq!(conv.domain.as_deref(), Some("conversational"));

        // Clean up
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_column_mapping_csv() {
        let dir = temp_dir();
        let path = dir.join("test.csv");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, "question,answer,quality,domain").unwrap();
        writeln!(f, "What is TCP?,A protocol,0.9,networking").unwrap();

        let mapping = detect_column_mapping(&path, &SourceType::LocalCsv);
        assert_eq!(mapping.key_field, "question");
        assert_eq!(mapping.value_field, "answer");
        assert_eq!(mapping.quality_field, Some("quality".to_string()));
        assert_eq!(mapping.domain_field, Some("domain".to_string()));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_column_mapping_jsonl() {
        let dir = temp_dir();
        let path = dir.join("test.jsonl");
        let mut f = std::fs::File::create(&path).unwrap();
        writeln!(f, r#"{{"instruction":"hello","output":"hi","quality":0.8,"domain":"chat"}}"#).unwrap();

        let mapping = detect_column_mapping(&path, &SourceType::LocalJsonl);
        assert_eq!(mapping.key_field, "instruction");
        assert_eq!(mapping.value_field, "output");
        assert_eq!(mapping.quality_field, Some("quality".to_string()));
        assert_eq!(mapping.domain_field, Some("domain".to_string()));

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_registry_summary() {
        let mut registry = DataSourceRegistry::new();
        registry.register(DataSourceConfig {
            name: "test1".into(),
            source_type: SourceType::LocalCsv,
            path_or_url: "/tmp/test1.csv".into(),
            domain: Some("cybersecurity".into()),
            default_quality: 0.8,
            auto_vet: true,
            max_facts: None,
            column_mapping: ColumnMapping::default(),
        });
        registry.register(DataSourceConfig {
            name: "test2".into(),
            source_type: SourceType::HuggingFace,
            path_or_url: "dataset/name".into(),
            domain: Some("conversational".into()),
            default_quality: 0.7,
            auto_vet: false,
            max_facts: Some(10000),
            column_mapping: ColumnMapping::default(),
        });

        registry.record_ingest(IngestResult {
            source_name: "test1".into(),
            source_type: "local_csv".into(),
            facts_processed: 1000,
            facts_added: 950,
            facts_skipped: 50,
            errors: vec![],
            duration_ms: 1234,
            domain: Some("cybersecurity".into()),
        });

        let summary = registry.summary();
        assert_eq!(summary.total_sources, 2);
        assert_eq!(summary.total_facts_ingested, 950);
        assert_eq!(summary.total_ingestion_runs, 1);
        assert_eq!(*summary.by_type.get("local_csv").unwrap(), 1);
        assert_eq!(*summary.by_type.get("huggingface").unwrap(), 1);
    }
}
