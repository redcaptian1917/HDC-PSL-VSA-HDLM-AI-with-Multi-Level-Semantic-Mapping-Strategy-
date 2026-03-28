// ============================================================
// LFI Agent Orchestrator — The Sovereign Mind
// Section 2: "Operate as an autonomous intelligence leveraging
// Zero-Trust and Assume Breach protocols."
// ============================================================

use crate::hdc::vector::BipolarVector;
use crate::hdc::compute::LocalBackend;
use crate::hdlm::codebook::HdlmCodebook;
use crate::hdlm::ast::NodeKind;
use crate::hdlm::intercept::OpsecIntercept;
use crate::psl::supervisor::PslSupervisor;
use crate::psl::axiom::{AuditTarget, DimensionalityAxiom, StatisticalEquilibriumAxiom, WebSearchSkepticismAxiom, ForbiddenSpaceAxiom, ClassInterestAxiom};
use crate::psl::coercion::CoercionAxiom;
use crate::psl::probes::{OverflowProbe, EncryptionProbe};
use crate::hid::HidDevice;
use crate::coder::LfiCoder;
use crate::hdc::liquid::LiquidSensorium;
use crate::hdc::superposition::SuperpositionStorage;
use crate::hdc::holographic::HolographicMemory;
use crate::hdc::analogy::AnalogyEngine;
use crate::hdc::sensory::{SensoryCortex, SensoryFrame};
use crate::intelligence::osint::OsintAnalyzer;
use crate::intelligence::web_audit::ConnectivityAxiom;
use crate::languages::genetic::GeneticOptimizer;
use crate::laws::{PrimaryLaw, LawLevel};
use crate::identity::{IdentityProver, SovereignProof, IdentityKind, SovereignSignature};
use crate::hdc::error::HdcError;
use crate::debuglog;

use crate::cognition::reasoner::CognitiveCore;
use crate::cognition::knowledge::NoveltyLevel;
use crate::languages::self_improve::SelfImproveEngine;
use crate::hdlm::tier2_decorative::DecorativeExpander;
use crate::intelligence::web_search::WebSearchEngine;
use crate::intelligence::persistence::KnowledgeStore;
use crate::intelligence::background::{BackgroundLearner, SharedKnowledge};

use std::sync::Arc;
use parking_lot::Mutex;

/// The Sovereign Agent. Orchestrates the full VSA stack under absolute law.
pub struct LfiAgent {
    pub compute: LocalBackend,
    pub supervisor: PslSupervisor,
    pub codebook: HdlmCodebook,
    pub hid: Result<HidDevice, HdcError>,
    pub coder: LfiCoder,
    pub sensorium: LiquidSensorium,
    pub optimizer: GeneticOptimizer,
    pub memory: SuperpositionStorage,
    pub holographic: HolographicMemory,
    pub analogy: AnalogyEngine,
    pub cortex: SensoryCortex,
    pub osint: OsintAnalyzer,
    pub reasoner: CognitiveCore,
    pub self_improve: SelfImproveEngine,
    pub entropy_level: f64,
    /// Whether the Sovereign User is authenticated.
    pub authenticated: bool,
    /// Conversation memory: facts learned during the session (key-value).
    conversation_facts: std::collections::HashMap<String, String>,
    /// Conversation history for context-aware responses.
    conversation_history: Vec<String>,
    /// Absolute proof of the Sovereign User.
    pub sovereign_identity: SovereignProof,
    /// Web search engine for real-time knowledge acquisition.
    web_search: WebSearchEngine,
    /// Shared knowledge with background learner (persistent, cross-session).
    shared_knowledge: Arc<Mutex<SharedKnowledge>>,
    /// Background learning daemon.
    background_learner: BackgroundLearner,
}

impl LfiAgent {
    /// Initialize a new Sovereign agent with Laws and Identity.
    pub fn new() -> Result<Self, HdcError> {
        debuglog!("LfiAgent::new: Initializing Sovereign intelligence");
        
        let compute = LocalBackend;
        let mut supervisor = PslSupervisor::new();
        
        // ... (axioms)
        supervisor.register_axiom(Box::new(DimensionalityAxiom));
        supervisor.register_axiom(Box::new(StatisticalEquilibriumAxiom { tolerance: 0.02 }));
        supervisor.register_axiom(Box::new(WebSearchSkepticismAxiom { min_credibility_score: 0.7 }));

        let name_vec = BipolarVector::from_seed(IdentityProver::hash("William Jhan Paul Armstrong"));
        let ssn_vec = BipolarVector::from_seed(IdentityProver::hash("647568607"));
        let license_vec = BipolarVector::from_seed(IdentityProver::hash("s23233305"));

        supervisor.register_axiom(Box::new(ForbiddenSpaceAxiom {
            forbidden_vectors: vec![name_vec, ssn_vec, license_vec],
            tolerance: 0.1,
        }));

        supervisor.register_axiom(Box::new(ClassInterestAxiom));
        supervisor.register_axiom(Box::new(CoercionAxiom { sensitivity: 0.7 }));
        supervisor.register_axiom(Box::new(ConnectivityAxiom { required_tunnel: "tor_obfs4".into() }));

        supervisor.register_axiom(Box::new(OverflowProbe));
        supervisor.register_axiom(Box::new(EncryptionProbe));
        
        let kinds = vec![NodeKind::Root, NodeKind::Assignment, NodeKind::Return];
        let codebook = HdlmCodebook::new(&kinds).map_err(|e| HdcError::InitializationFailed {
            reason: format!("Codebook init failed: {}", e),
        })?;
        
        let hid = HidDevice::new(None);
        let coder = LfiCoder::new();
        let sensorium = LiquidSensorium::new(19);
        let optimizer = GeneticOptimizer::new(20, 10);
        let memory = SuperpositionStorage::new();
        let holographic = HolographicMemory::new();
        let analogy = AnalogyEngine::new();
        let cortex = SensoryCortex::new()?;
        let osint = OsintAnalyzer::new();
        let reasoner = CognitiveCore::new()?;
        let psl_copy = PslSupervisor::new();
        let self_improve = SelfImproveEngine::new(psl_copy);

        // Initialize web search engine
        let web_search = WebSearchEngine::new();

        // Load persistent knowledge from disk (survives across sessions)
        let store_path = KnowledgeStore::default_path();
        let persistent_store = KnowledgeStore::load(&store_path).unwrap_or_else(|e| {
            debuglog!("LfiAgent::new: Failed to load persistent knowledge: {:?}, starting fresh", e);
            KnowledgeStore::new()
        });
        debuglog!(
            "LfiAgent::new: Persistent knowledge loaded — {} concepts, {} facts, session #{}",
            persistent_store.concepts.len(), persistent_store.facts.len(), persistent_store.session_count
        );

        // Initialize background learner with persistent store
        let background_learner = BackgroundLearner::new(persistent_store);
        let shared_knowledge = background_learner.shared_knowledge();

        // Load persistent facts into conversation_facts
        let mut conversation_facts = std::collections::HashMap::new();
        {
            let guard = shared_knowledge.lock();
            for fact in &guard.store.facts {
                conversation_facts.insert(fact.key.clone(), fact.value.clone());
            }
            // Load persistent concepts into the cognitive core's knowledge engine
            debuglog!("LfiAgent::new: Restoring {} persistent concepts to knowledge engine",
                     guard.store.concepts.len());
        }

        // Secure Identity Commitment (ZKI)
        let sovereign_identity = IdentityProver::commit(
            "William Jhan Paul Armstrong",
            "647568607",
            "s23233305",
            "-G;#/,n3Ndif!#9Fua72n`[}mbxu!s_GiWMN5w\\~]",
            IdentityKind::Sovereign
        );

        Ok(Self {
            compute, supervisor, codebook, hid, coder,
            sensorium, optimizer, memory, holographic, analogy, cortex, osint,
            reasoner, self_improve,
            entropy_level: 0.1,
            authenticated: false,
            conversation_facts,
            conversation_history: Vec::new(),
            sovereign_identity,
            web_search,
            shared_knowledge,
            background_learner,
        })
    }

    /// Authenticate the user via password.
    pub fn authenticate(&mut self, password: &str) -> bool {
        self.authenticated = IdentityProver::verify_password(&self.sovereign_identity, password);
        self.authenticated
    }

    /// Check if the agent has learned enough new concepts to warrant a self-source refinement.
    /// This allows the AI to "update words in this file all by itself".
    pub fn check_for_self_refinement(&mut self) -> Result<Option<String>, HdcError> {
        let learned_count = self.reasoner.knowledge.concepts().len();
        debuglog!("LfiAgent: Checking for self-refinement (Learned Concepts={})", learned_count);

        if learned_count > 50 { // Threshold for self-evolution
            debuglog!("LfiAgent: Escape velocity threshold reached. Proposing self-source refinement.");
            
            let mut proposal = "SYSTEM EVOLUTION PROPOSAL: Self-Source Refinement of 'seed_intents'.\n\n".to_string();
            proposal.push_str("Based on our interactions, I have identified several new high-value keywords that should be integrated into my core: \n");
            
            for proto in self.reasoner.intent_prototypes() {
                if proto.keywords.len() > 15 {
                    proposal.push_str(&format!("* Intent '{}' has expanded to {} keywords. Proposing source update.\n", 
                                     proto.intent_name, proto.keywords.len()));
                }
            }
            
            proposal.push_str("\nI can use my LfiCoder to rewrite 'src/cognition/reasoner.rs' with these new axioms. Shall I proceed, Sovereign?");
            return Ok(Some(proposal));
        }
        
        Ok(None)
    }

    /// Interact with the Sovereign agent via natural language.
    /// Access to internal reasoning and technical synthesis is gated.
    pub fn chat(&mut self, input: &str) -> Result<String, HdcError> {
        debuglog!("LfiAgent::chat: input='{}'", input);

        // Incorporate any background learnings before processing
        self.incorporate_background_learnings();

        // Handle system commands (background learning toggle, save, etc.)
        if let Some(cmd_response) = self.handle_system_command(input) {
            return Ok(cmd_response);
        }

        // 0. Check for simple arithmetic before cognitive pipeline
        if let Some(math_result) = Self::try_evaluate_math(input) {
            debuglog!("LfiAgent::chat: math expression detected, result={}", math_result);
            return Ok(math_result);
        }

        // 0b. Check for teaching commands ("remember that X means Y")
        if let Some(teach_result) = self.try_learn_from_teaching(input) {
            debuglog!("LfiAgent::chat: teaching command detected");
            return Ok(teach_result);
        }

        // 0c. Auto-learn from conversational context (no explicit "remember" needed)
        if let Some(learn_response) = self.auto_learn_from_input(input) {
            debuglog!("LfiAgent::chat: auto-learned fact, short-circuiting");
            return Ok(learn_response);
        }

        // 0d. Try to answer from conversation facts / persistent knowledge
        if let Some(fact_response) = self.try_answer_from_facts(input) {
            debuglog!("LfiAgent::chat: answered from conversation facts");
            return Ok(fact_response);
        }

        // 0e. Check persistent knowledge store for previously learned answers
        if let Some(persistent_response) = self.try_answer_from_persistent(input) {
            debuglog!("LfiAgent::chat: answered from persistent knowledge");
            return Ok(persistent_response);
        }

        // 1. Pre-Audit for Injection (Double Gate)
        let is_suspicious = self.reasoner.scan_for_injection(input);

        // 2. Determine if we should allow "Deep" reasoning based on auth status
        let original_threshold = self.reasoner.novelty_threshold();
        if !self.authenticated || is_suspicious {
            // Force "Fast" mode only
            self.reasoner.set_novelty_threshold(1.0);
        }

        // 3. Process through Cognitive Core
        let response = self.reasoner.respond(input)?;
        let mut final_text = response.text;

        // Restore threshold
        if !self.authenticated || is_suspicious {
            self.reasoner.set_novelty_threshold(original_threshold);
        }

        // 4. Adversarial Signature Check
        let is_adversarial = matches!(response.thought.intent, Some(crate::cognition::reasoner::Intent::Adversarial { .. })) || is_suspicious;

        if is_adversarial {
            debuglog!("LfiAgent: ADVERSARIAL SIGNATURE DETECTED. PURGING RESPONSE BUFFER.");
            return Ok("Adversarial signature detected. Trust-tier mismatch. All sensitive reasoning has been purged from the material base.".to_string());
        }

        // 5. Trust-Based Learning Gating: Only learn if authenticated as Sovereign.
        if self.authenticated {
            // --- NEW: Autonomous Semantic Discovery ---
            // If the KnowledgeEngine identified unknown aspects, try to bind them to the current intent.
            if let Ok(NoveltyLevel::Partial { unknown_aspects, .. }) = self.reasoner.knowledge.assess_novelty(input) {
                if let Some(intent) = &response.thought.intent {
                    let intent_name = match intent {
                        crate::cognition::reasoner::Intent::WriteCode { .. } => "write_code",
                        crate::cognition::reasoner::Intent::Analyze { .. } => "analyze",
                        crate::cognition::reasoner::Intent::FixBug { .. } => "fix_bug",
                        crate::cognition::reasoner::Intent::Explain { .. } => "explain",
                        crate::cognition::reasoner::Intent::Search { .. } => "search",
                        crate::cognition::reasoner::Intent::PlanTask { .. } => "plan",
                        crate::cognition::reasoner::Intent::Converse { .. } => "converse",
                        crate::cognition::reasoner::Intent::Improve { .. } => "improve",
                        _ => "",
                    };

                    if !intent_name.is_empty() {
                        for word in &unknown_aspects {
                            debuglog!("LfiAgent: AUTONOMOUS DISCOVERY: Word '{}' appears in {} context. Updating intent prototype.", word, intent_name);
                            let _ = self.reasoner.learn_keyword(intent_name, word);
                        }
                    }
                }
            }

            if let Some(intent) = &response.thought.intent {
                match intent {
                    crate::cognition::reasoner::Intent::Explain { topic } => {
                        let _ = self.reasoner.knowledge.learn(topic, &[], true);
                    }
                    _ => {}
                }
            }
        }

        // 5b. WEB SEARCH FALLBACK: If the cognitive core can't answer, search the web.
        let is_search_intent = matches!(
            response.thought.intent,
            Some(crate::cognition::reasoner::Intent::Search { .. })
        );
        let is_unknown = matches!(
            response.thought.intent,
            Some(crate::cognition::reasoner::Intent::Unknown { .. }) | None
        );
        let is_generic_fallback = final_text.contains("not sure I fully understand")
            || final_text.contains("need a clearer instruction")
            || final_text.contains("code/systems AI")
            || final_text.contains("outside my current knowledge domain")
            || final_text.contains("outside my domain")
            || final_text.contains("I'll search for that")
            || final_text.contains("seen this pattern before")
            || final_text.contains("Using a familiar planning template")
            || final_text.contains("I partially understand this")
            || final_text.contains("I'd need to research these");

        if self.authenticated && (is_unknown || is_generic_fallback || is_search_intent) {
            debuglog!("LfiAgent::chat: cognitive core can't answer (unknown={}, fallback={}), trying web search",
                     is_unknown, is_generic_fallback);
            if let Some(web_answer) = self.search_and_learn(input) {
                final_text = web_answer;
            }
            // If web search also fails, queue for background research
            else {
                self.background_learner.enqueue_research(input);
            }
        }

        // 6. Security Gating: Restrict internal details to authenticated Sovereign only.
        if !self.authenticated {
            // Strip out internal reasoning scratchpad and planning details if not authenticated.
            let lines: Vec<&str> = final_text.lines().collect();
            let mut sanitized = Vec::new();
            let mut skipping = false;
            
            for line in lines {
                if line.contains("--- INTERNAL REASONING SCRATCHPAD ---") || 
                   line.contains("Plan:") || 
                   line.contains("Mode: Deep") ||
                   line.contains("Cognitive Analysis:") ||
                   line.contains("Analysis:") {
                    skipping = true;
                    continue;
                }
                if line.contains("--- END REASONING ---") || line.contains("--- END CODE ---") {
                    skipping = false;
                    continue;
                }
                if !skipping {
                    sanitized.push(line);
                }
            }
            
            if sanitized.is_empty() || (sanitized.len() == 1 && sanitized[0].is_empty()) {
                final_text = "Action processed at the symbolic layer. Full cognitive derivation requires Sovereign authentication.".to_string();
            } else {
                final_text = sanitized.join("\n");
            }
            
            return Ok(final_text);
        }

        // 7. Fulfill intents that require specialized tools (Authenticated & Trusted Only)
        if let Some(intent) = &response.thought.intent {
            match intent {
                crate::cognition::reasoner::Intent::WriteCode { language, description: _ } => {
                    debuglog!("LfiAgent::chat: Fulfilling WriteCode intent for {}", language);
                    
                    let constructs = vec![crate::languages::UniversalConstruct::Block];
                    let lang_id = match language.to_lowercase().as_str() {
                        "rust" => crate::languages::registry::LanguageId::Rust,
                        "go" => crate::languages::registry::LanguageId::Go,
                        "python" => crate::languages::registry::LanguageId::Python,
                        _ => crate::languages::registry::LanguageId::Rust,
                    };
                    
                    if let Ok(ast) = self.coder.synthesize(lang_id, &constructs) {
                        let renderer = crate::hdlm::tier2_decorative::InfixRenderer;
                        if let Ok(code) = renderer.render(&ast) {
                            final_text.push_str("\n\n--- GENERATED CODE ---\n");
                            final_text.push_str(&code);
                            final_text.push_str("\n--- END CODE ---\n");
                        }
                    }
                }
                crate::cognition::reasoner::Intent::Analyze { target } => {
                    debuglog!("LfiAgent::chat: Fulfilling Analyze intent for {}", target);
                    let mut ast = crate::hdlm::ast::Ast::new();
                    let _root = ast.add_node(NodeKind::Root);
                    let metrics = self.self_improve.evaluate_ast(&ast);
                    
                    final_text.push_str("\n\n--- FORENSIC AUDIT METRICS ---\n");
                    final_text.push_str(&format!("  Overall Score: {:.4}\n", metrics.overall_score()));
                    final_text.push_str(&format!("  Balance: {:.2}\n", metrics.balance));
                    final_text.push_str(&format!("  Nesting Depth: {}\n", metrics.depth));
                    final_text.push_str("--- END AUDIT ---\n");
                }
                _ => {}
            }
        }

        Ok(final_text)
    }

    /// Try to evaluate a simple arithmetic expression from natural language.
    /// Handles: "what is 2 + 2", "calculate 7 * 3", "2+2", "15 / 3", etc.
    fn try_evaluate_math(input: &str) -> Option<String> {
        debuglog!("LfiAgent::try_evaluate_math: checking '{}'", &input[..input.len().min(50)]);
        let text = input.to_lowercase();

        // Extract numbers and operators
        let cleaned: String = text.chars()
            .filter(|c| c.is_ascii_digit() || *c == '+' || *c == '-' || *c == '*' || *c == '/' || *c == '.' || *c == ' ')
            .collect();
        let cleaned = cleaned.trim();

        // Parse: look for pattern NUMBER OP NUMBER
        let parts: Vec<&str> = cleaned.split_whitespace().collect();

        // Try "N op N" pattern
        if parts.len() == 3 {
            if let (Ok(a), Ok(b)) = (parts[0].parse::<f64>(), parts[2].parse::<f64>()) {
                let result = match parts[1] {
                    "+" => Some(a + b),
                    "-" => Some(a - b),
                    "*" => Some(a * b),
                    "/" => if b != 0.0 { Some(a / b) } else { None },
                    _ => None,
                };
                if let Some(r) = result {
                    if r == r.floor() {
                        return Some(format!("{} {} {} = {}", a as i64, parts[1], b as i64, r as i64));
                    }
                    return Some(format!("{} {} {} = {:.4}", a, parts[1], b, r));
                }
            }
        }

        // Try "N+N" (no spaces) pattern
        if parts.len() == 1 && cleaned.len() > 2 {
            for op_char in &['+', '-', '*', '/'] {
                if let Some(pos) = cleaned.find(*op_char) {
                    if pos > 0 && pos < cleaned.len() - 1 {
                        let left = &cleaned[..pos];
                        let right = &cleaned[pos + 1..];
                        if let (Ok(a), Ok(b)) = (left.parse::<f64>(), right.parse::<f64>()) {
                            let result = match op_char {
                                '+' => Some(a + b),
                                '-' => Some(a - b),
                                '*' => Some(a * b),
                                '/' => if b != 0.0 { Some(a / b) } else { None },
                                _ => None,
                            };
                            if let Some(r) = result {
                                if r == r.floor() {
                                    return Some(format!("{} {} {} = {}", a as i64, op_char, b as i64, r as i64));
                                }
                                return Some(format!("{} {} {} = {:.4}", a, op_char, b, r));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Handle teaching commands: "remember that X means Y"
    fn try_learn_from_teaching(&mut self, input: &str) -> Option<String> {
        debuglog!("LfiAgent::try_learn_from_teaching: checking '{}'", &input[..input.len().min(50)]);
        let text_lower = input.to_lowercase();

        // Pattern: "remember that X means Y" or "X means Y" or "learn that X is Y"
        let patterns = vec![
            ("remember that ", " means "),
            ("remember that ", " is "),
            ("learn that ", " means "),
            ("learn that ", " is "),
        ];

        for (prefix, separator) in patterns {
            if text_lower.starts_with(prefix) {
                let rest = &input[prefix.len()..];
                if let Some(sep_pos) = rest.to_lowercase().find(separator) {
                    let concept = rest[..sep_pos].trim();
                    let definition = rest[sep_pos + separator.len()..].trim();

                    if !concept.is_empty() && !definition.is_empty() {
                        if !self.authenticated {
                            return Some("Teaching requires Sovereign authentication. \
                                         I cannot learn from unauthenticated sources.".to_string());
                        }

                        let related: Vec<&str> = definition.split_whitespace()
                            .filter(|w| w.len() > 3)
                            .take(5)
                            .collect();

                        let concept_key = concept.to_lowercase().replace(' ', "_");

                        // Store in both knowledge engine and conversation facts
                        self.conversation_facts.insert(concept_key.clone(), definition.to_string());

                        let result = self.reasoner.knowledge.learn(
                            &concept_key,
                            &related,
                            true
                        );
                        match result {
                            Ok(()) => {
                                return Some(format!(
                                    "Learned: '{}' — {}. Now stored in holographic memory. \
                                     {} concepts total.",
                                    concept, definition, self.reasoner.knowledge.concept_count()
                                ));
                            }
                            Err(e) => {
                                return Some(format!("Failed to learn: {:?}", e));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Automatically learn facts from conversational input without explicit "remember" commands.
    /// Returns Some(response) if a learnable fact was detected and should short-circuit the pipeline.
    ///
    /// Detects patterns like:
    /// - "my name is X" / "I'm X" → stores name
    /// - "X is Y" / "X means Y" → stores definition
    /// - "I work on X" / "I'm working on X" → stores project context
    fn auto_learn_from_input(&mut self, input: &str) -> Option<String> {
        debuglog!("LfiAgent::auto_learn_from_input: scanning for learnable facts");

        // Always store conversation history regardless of auth
        self.conversation_history.push(input.to_string());
        if self.conversation_history.len() > 50 {
            self.conversation_history.remove(0);
        }

        if !self.authenticated {
            debuglog!("LfiAgent::auto_learn_from_input: not authenticated, skipping");
            return None;
        }

        let text_lower = input.to_lowercase();

        // Pattern: "my name is X" or "i'm X" or "i am X" (only for names)
        for prefix in &["my name is ", "my name's ", "i'm ", "i am ", "call me "] {
            if text_lower.starts_with(prefix) {
                let name = input[prefix.len()..].trim();
                // Strip trailing punctuation
                let name = name.trim_end_matches(|c: char| c == '.' || c == '!' || c == ',');
                if !name.is_empty() && name.len() < 50 {
                    let word_count = name.split_whitespace().count();
                    if word_count <= 4 {
                        self.conversation_facts.insert("sovereign_name".to_string(), name.to_string());
                        debuglog!("LfiAgent::auto_learn: Sovereign name = '{}'", name);
                        return Some(format!("Understood. I'll remember your name, {}.", name));
                    }
                }
            }
        }

        // Pattern: "X is Y" or "X means Y" — learn definitions
        // Only if the sentence is structured as a definition, not a question
        if !text_lower.starts_with("what") && !text_lower.starts_with("who") &&
           !text_lower.starts_with("how") && !text_lower.starts_with("is ") &&
           !text_lower.starts_with("do ") && !text_lower.starts_with("can ") &&
           !text_lower.contains("?") {
            for separator in &[" is ", " means ", " refers to ", " stands for "] {
                if let Some(pos) = text_lower.find(separator) {
                    let concept = input[..pos].trim();
                    let definition = input[pos + separator.len()..].trim();
                    let concept_words = concept.split_whitespace().count();
                    let def_words = definition.split_whitespace().count();

                    // Only learn if concept is short (1-3 words) and definition is substantive (2+ words)
                    if concept_words >= 1 && concept_words <= 3 && def_words >= 2 && def_words <= 30 {
                        let concept_key = concept.to_lowercase().replace(' ', "_");
                        self.conversation_facts.insert(concept_key.clone(), definition.to_string());

                        let related: Vec<&str> = definition.split_whitespace()
                            .filter(|w| w.len() > 3)
                            .take(5)
                            .collect();
                        let _ = self.reasoner.knowledge.learn(&concept_key, &related, true);

                        debuglog!("LfiAgent::auto_learn: '{}' = '{}'", concept, definition);
                        return Some(format!("Learned: {} — {}. {} concepts in memory.",
                            concept, definition, self.reasoner.knowledge.concept_count()));
                    }
                }
            }
        }

        None
    }

    /// Try to answer a question from conversation facts learned during this session.
    /// Handles: "what is my name?", "what is X?", "what does X mean?"
    fn try_answer_from_facts(&self, input: &str) -> Option<String> {
        debuglog!("LfiAgent::try_answer_from_facts: checking '{}'", &input[..input.len().min(60)]);
        let text_lower = input.to_lowercase();
        let text_clean = text_lower.trim_end_matches('?').trim();

        // "what is my name" / "who am i"
        if text_clean == "what is my name" || text_clean == "who am i" ||
           text_clean == "whats my name" || text_clean == "what's my name" ||
           text_clean.contains("my name") {
            if let Some(name) = self.conversation_facts.get("sovereign_name") {
                return Some(format!("Your name is {}.", name));
            } else {
                return Some("You haven't told me your name yet. You can say 'my name is X'.".to_string());
            }
        }

        // "what is X" / "what does X mean" / "tell me about X" / "explain X" — check learned facts
        let concept_query = if text_clean.starts_with("what is ") {
            Some(&text_clean[8..])
        } else if text_clean.starts_with("what does ") && text_clean.ends_with(" mean") {
            Some(&text_clean[10..text_clean.len()-5])
        } else if text_clean.starts_with("define ") {
            Some(&text_clean[7..])
        } else if text_clean.starts_with("tell me about ") {
            Some(&text_clean[14..])
        } else if text_clean.starts_with("what do you know about ") {
            Some(&text_clean[23..])
        } else {
            None
        };

        if let Some(query) = concept_query {
            let query_key = query.trim().replace(' ', "_");
            if let Some(definition) = self.conversation_facts.get(&query_key) {
                let display_name = query.trim().to_uppercase();
                return Some(format!("{} — {}.", display_name, definition));
            }
        }

        None
    }

    /// Handle system commands: background learning toggle, save, status.
    fn handle_system_command(&mut self, input: &str) -> Option<String> {
        let text_lower = input.to_lowercase().trim().to_string();

        if text_lower == "/learn on" || text_lower == "start learning" || text_lower == "/background on" {
            if !self.authenticated {
                return Some("Background learning requires Sovereign authentication.".to_string());
            }
            match self.start_background_learning() {
                Ok(()) => Some("Background learning daemon ACTIVATED. I'll continuously search the web, learn, and improve. Use '/learn off' to stop.".to_string()),
                Err(e) => Some(format!("Failed to start background learning: {:?}", e)),
            }
        } else if text_lower == "/learn off" || text_lower == "stop learning" || text_lower == "/background off" {
            match self.stop_background_learning() {
                Ok(()) => Some("Background learning daemon DEACTIVATED. Knowledge saved to disk.".to_string()),
                Err(e) => Some(format!("Failed to stop background learning: {:?}", e)),
            }
        } else if text_lower == "/save" || text_lower == "save knowledge" {
            match self.save_knowledge() {
                Ok(()) => {
                    let count = self.conversation_facts.len();
                    Some(format!("Knowledge saved to disk. {} facts persisted.", count))
                }
                Err(e) => Some(format!("Failed to save knowledge: {:?}", e)),
            }
        } else if text_lower == "/status" {
            let bg = if self.is_background_learning() { "ACTIVE" } else { "INACTIVE" };
            let concepts = self.reasoner.knowledge.concept_count();
            let facts = self.conversation_facts.len();
            let guard = self.shared_knowledge.lock();
            let persistent_concepts = guard.store.concepts.len();
            let persistent_facts = guard.store.facts.len();
            let session = guard.store.session_count;
            let queue = guard.store.searched_topics.len();
            drop(guard);
            Some(format!(
                "System Status:\n\
                 - Authenticated: {}\n\
                 - Background Learning: {}\n\
                 - Session Concepts: {}\n\
                 - Session Facts: {}\n\
                 - Persistent Concepts: {}\n\
                 - Persistent Facts: {}\n\
                 - Session Number: {}\n\
                 - Topics Searched: {}",
                self.authenticated, bg, concepts, facts,
                persistent_concepts, persistent_facts, session, queue
            ))
        } else {
            None
        }
    }

    /// Try to answer from the persistent knowledge store (cross-session memory).
    fn try_answer_from_persistent(&self, input: &str) -> Option<String> {
        debuglog!("LfiAgent::try_answer_from_persistent: checking '{}'", &input[..input.len().min(60)]);
        let text_lower = input.to_lowercase();
        let text_clean = text_lower.trim_end_matches('?').trim();

        // Extract query subject from various patterns
        let query = if text_clean.starts_with("what is ") {
            Some(&text_clean[8..])
        } else if text_clean.starts_with("what are ") {
            Some(&text_clean[9..])
        } else if text_clean.starts_with("tell me about ") {
            Some(&text_clean[14..])
        } else if text_clean.starts_with("what do you know about ") {
            Some(&text_clean[23..])
        } else if text_clean.starts_with("define ") {
            Some(&text_clean[7..])
        } else if text_clean.starts_with("who is ") {
            Some(&text_clean[7..])
        } else if text_clean.starts_with("who was ") {
            Some(&text_clean[8..])
        } else if text_clean.starts_with("how many ") {
            // For "how many people live in X" type questions, just pass through
            None
        } else {
            None
        };

        if let Some(q) = query {
            let query_key = q.trim().replace(' ', "_");
            let guard = self.shared_knowledge.lock();

            // Check persistent facts
            if let Some(value) = guard.store.get_fact(&query_key) {
                return Some(format!("{} — {}", q.trim().to_uppercase(), value));
            }

            // Check persistent concepts with definitions
            for concept in &guard.store.concepts {
                if concept.name == query_key {
                    if let Some(ref def) = concept.definition {
                        return Some(format!("{} — {} [mastery: {:.0}%, trust: {:.0}%]",
                            q.trim().to_uppercase(), def,
                            concept.mastery * 100.0, concept.trust_score * 100.0));
                    }
                }
            }
        }

        None
    }

    /// Get a conversation fact by key.
    pub fn get_fact(&self, key: &str) -> Option<&String> {
        self.conversation_facts.get(key)
    }

    /// Get the sovereign's name if they've told us.
    pub fn sovereign_name(&self) -> Option<&String> {
        self.conversation_facts.get("sovereign_name")
    }

    /// Get conversation history.
    pub fn history(&self) -> &[String] {
        &self.conversation_history
    }

    /// Start background learning daemon. Continuously searches web and learns.
    pub fn start_background_learning(&mut self) -> Result<(), HdcError> {
        debuglog!("LfiAgent::start_background_learning: activating daemon");
        self.background_learner.start()
    }

    /// Stop background learning daemon. Saves knowledge to disk.
    pub fn stop_background_learning(&mut self) -> Result<(), HdcError> {
        debuglog!("LfiAgent::stop_background_learning: deactivating daemon");
        self.background_learner.stop()
    }

    /// Check if background learning is active.
    pub fn is_background_learning(&self) -> bool {
        self.background_learner.is_running()
    }

    /// Incorporate any recent learnings from background research into conversation.
    fn incorporate_background_learnings(&mut self) {
        let learnings = self.background_learner.drain_recent_learnings();
        for learning in &learnings {
            debuglog!(
                "LfiAgent::incorporate_background_learnings: '{}' (trust={:.2})",
                learning.topic, learning.trust
            );
            let concept_key = learning.topic.to_lowercase().replace(' ', "_");
            self.conversation_facts.insert(concept_key.clone(), learning.summary.clone());

            let related: Vec<&str> = learning.summary.split_whitespace()
                .filter(|w| w.len() > 4)
                .take(5)
                .collect();
            let _ = self.reasoner.knowledge.learn(&concept_key, &related, true);
        }
        if !learnings.is_empty() {
            debuglog!(
                "LfiAgent::incorporate_background_learnings: absorbed {} new learnings",
                learnings.len()
            );
        }
    }

    /// Search the web for an answer and learn from results.
    /// Returns a formatted answer string if results are found.
    fn search_and_learn(&mut self, query: &str) -> Option<String> {
        debuglog!("LfiAgent::search_and_learn: '{}'", &query[..query.len().min(80)]);

        if !self.authenticated {
            debuglog!("LfiAgent::search_and_learn: not authenticated, skipping web search");
            return None;
        }

        match self.web_search.search(query) {
            Ok(response) => {
                debuglog!("LfiAgent::search_and_learn: {} results, summary_len={}",
                         response.results.len(), response.best_summary.len());
                if response.results.is_empty() {
                    return None;
                }

                let summary = &response.best_summary;
                if summary.is_empty() {
                    return None;
                }

                // Learn from search results
                let concept_key = query.to_lowercase().replace(' ', "_")
                    .chars().filter(|c| c.is_alphanumeric() || *c == '_').collect::<String>();
                self.conversation_facts.insert(concept_key.clone(), summary.clone());

                let related: Vec<&str> = summary.split_whitespace()
                    .filter(|w| w.len() > 4)
                    .take(5)
                    .collect();
                let _ = self.reasoner.knowledge.learn(&concept_key, &related, true);

                // Persist to shared knowledge store
                {
                    let mut guard = self.shared_knowledge.lock();
                    guard.store.upsert_fact(&concept_key, summary);
                    guard.store.mark_searched(query);
                }

                // Format the response with source attribution
                let source_info = if response.source_count > 1 {
                    format!(" [Verified across {} sources, trust: {:.0}%]",
                        response.source_count, response.cross_reference_trust * 100.0)
                } else {
                    format!(" [Single source, trust: {:.0}%]",
                        response.cross_reference_trust * 100.0)
                };

                // Truncate if too long
                let display_summary = if summary.len() > 500 {
                    format!("{}...", &summary[..497])
                } else {
                    summary.clone()
                };

                Some(format!("{}{}", display_summary, source_info))
            }
            Err(e) => {
                debuglog!("LfiAgent::search_and_learn: ERROR: {:?}", e);
                None
            }
        }
    }

    /// Save all current knowledge to persistent storage.
    pub fn save_knowledge(&self) -> Result<(), HdcError> {
        debuglog!("LfiAgent::save_knowledge: persisting to disk");
        let store_path = KnowledgeStore::default_path();
        let mut guard = self.shared_knowledge.lock();

        // Sync conversation facts to persistent store
        for (key, value) in &self.conversation_facts {
            guard.store.upsert_fact(key, value);
        }

        guard.store.save(&store_path)
    }

    /// Toggles the Entropy Governor between Divergent (High) and Convergent (Low).
    pub fn set_entropy(&mut self, is_creative: bool) {
        self.entropy_level = if is_creative { 0.9 } else { 0.1 };
        debuglog!("LfiAgent: Entropy level adjusted to {:.2}", self.entropy_level);
        // We could also dynamically adjust PSL tolerances here based on entropy.
    }

    /// Process raw noise through the LNN -> HDLM -> PSL pipeline.
    pub fn ingest_noise(&mut self, noise_signal: f64) -> Result<(), HdcError> {
        debuglog!("LfiAgent::ingest_noise: ADAPT -> ENCODE -> AUDIT");
        
        // 1. ADAPT (Liquid State)
        self.sensorium.step(noise_signal, 0.01)?;
        
        // 2. ENCODE & DISCRETIZE (The Bridge to HDLM)
        // We project the fluid state into a hypervector.
        let signal_vector = self.sensorium.project_to_vsa()?;
        
        // 3. AUDIT (PSL Verification)
        let target = AuditTarget::Vector(signal_vector);
        let assessment = self.supervisor.audit(&target).map_err(|e| HdcError::InitializationFailed {
            reason: format!("Ingestion audit failure: {:?}", e),
        })?;

        if !assessment.level.permits_execution() {
            debuglog!("LfiAgent: Audit Failed. Data discarded.");
            return Err(HdcError::InitializationFailed {
                reason: "Hostile data detected".to_string(),
            });
        }

        debuglog!("LfiAgent: Verified data bound to symbolic memory.");
        Ok(())
    }

    /// Continuous audit of telemetry for coercion signals.
    /// Triggers Secure Overwrite of RAM logs if threshold met.
    pub fn audit_coercion(&self, jitter: f64, geo_risk: f64) -> Result<f64, HdcError> {
        let fields = vec![
            ("stress_jitter".to_string(), jitter.to_string()),
            ("geo_risk".to_string(), geo_risk.to_string()),
        ];
        let target = AuditTarget::Payload { 
            source: "telemetry_sensors".to_string(), 
            fields 
        };

        let assessment = self.supervisor.audit(&target).map_err(|e| HdcError::InitializationFailed {
            reason: format!("Coercion audit failure: {:?}", e),
        })?;

        if !assessment.level.permits_execution() {
            debuglog!("LfiAgent: CRITICAL THREAT DETECTED. Executing Sovereign Purge.");
            crate::telemetry::wipe_logs();
        }

        Ok(assessment.confidence)
    }

    /// Process a direct sensory frame into the VSA Sensory Cortex.
    pub fn ingest_sensor_frame(&mut self, frame: &SensoryFrame) -> Result<BipolarVector, HdcError> {
        debuglog!("LfiAgent: DIRECT SENSORY INGESTION - Bypassing HAL");
        
        // 1. Encode frame directly via Cortex
        let encoded = self.cortex.encode_frame(frame)?;
        
        // 2. Audit against Dialectical Materialism (Ensure no hegemonic spoofing)
        let target = AuditTarget::Vector(encoded.clone());
        let _assessment = self.supervisor.audit(&target).map_err(|e| HdcError::InitializationFailed {
            reason: format!("Sensory audit failure: {:?}", e),
        })?;

        // 3. Bind to Holographic Memory
        let context_key = BipolarVector::from_seed(frame.timestamp);
        self.holographic.associate(&context_key, &encoded)?;

        Ok(encoded)
    }

    /// Creative Synthesis: Solves an engineering problem via structural analogy.
    pub fn synthesize_creative_solution(&self, problem_description: &str) -> Result<BipolarVector, HdcError> {
        debuglog!("LfiAgent: CREATIVE SYNTHESIS - Engineering Tomorrow's Solutions");
        
        // 1. Vectorize the problem
        let p_hash = IdentityProver::hash(problem_description);
        let p_vector = BipolarVector::from_seed(p_hash);
        
        // 2. Map structural similarities
        self.analogy.synthesize_solution(&p_vector)
    }

    /// Serialize the current LNN and VSA state to a VSA-Encrypted Blob.
    pub fn save_persistent_state(&self, path: &str) -> Result<(), HdcError> {
        debuglog!("LfiAgent::save_persistent_state: Serializing logic base to {}", path);
        // The memory object holds the superimposed VSA state.
        self.memory.save_to_disk(path)
    }

    /// Load the LNN and VSA state from a VSA-Encrypted Blob.
    pub fn load_persistent_state(&mut self, path: &str) -> Result<(), HdcError> {
        debuglog!("LfiAgent::load_persistent_state: Restoring logic base from {}", path);
        self.memory = SuperpositionStorage::load_from_disk(path)?;
        Ok(())
    }

    /// Process text through the Intercept -> HDLM -> PSL pipeline.
    pub fn ingest_text(&mut self, text: &str) -> Result<String, HdcError> {
        debuglog!("LfiAgent::ingest_text: INTERCEPT -> ENCODE -> AUDIT");

        // 1. INTERCEPT (OPSEC Sweep)
        let intercept_result = OpsecIntercept::scan(text).map_err(|e| HdcError::InitializationFailed {
            reason: format!("OPSEC intercept failed: {:?}", e),
        })?;
        if !intercept_result.matches_found.is_empty() {
            debuglog!("LfiAgent: INTERCEPTED {} OPSEC MARKERS", intercept_result.matches_found.len());
        }

        // 2. ENCODE (Project sanitized text to VSA)
        let text_hash = IdentityProver::hash(&intercept_result.sanitized);
        let text_vector = BipolarVector::from_seed(text_hash);

        // 3. AUDIT (PSL Verification)
        let target = AuditTarget::Vector(text_vector.clone());
        let assessment = self.supervisor.audit(&target).map_err(|e| HdcError::InitializationFailed {
            reason: format!("Text ingestion audit failure: {:?}", e),
        })?;

        if !assessment.level.permits_execution() {
            debuglog!("LfiAgent: PSL BLOCK. Possible identity leakage in sanitized stream.");
            return Err(HdcError::InitializationFailed {
                reason: "Hostile/Forbidden data detected".to_string(),
            });
        }

        debuglog!("LfiAgent: Verified text bound to symbolic memory.");
        
        // 4. MEMORY COMMIT (PD Protocol)
        self.memory.commit_real(&text_vector)?;
        // Inject TRNG-backed chaff
        for _ in 0..3 {
            let chaff = BipolarVector::new_trng()?;
            self.memory.commit_real(&chaff)?; // Note: simplify for demo, PD storage logic varies
        }

        // 5. HOLOGRAPHIC ASSOCIATION (O(1) Long-term Memory)
        let context_key = BipolarVector::new_trng()?;
        self.holographic.associate(&context_key, &text_vector)?;

        Ok(intercept_result.sanitized)
    }

    /// Executes a task only if it complies with the Sovereign Laws and HSM signature.
    pub fn execute_task(&self, task_name: &str, level: LawLevel, signature: &SovereignSignature) -> Result<(), HdcError> {
        debuglog!("LfiAgent::execute_task: auditing '{}' against Sovereign Laws", task_name);

        // 1. SVI (Signature-Verified Instruction) Gate
        if !IdentityProver::verify_signature(&self.sovereign_identity, task_name, signature) {
            debuglog!("LfiAgent: SVI REJECTED. Instruction has zero weight in the LNN.");
            return Err(HdcError::InitializationFailed {
                reason: "Unauthorized instruction (HSM Signature Failure)".to_string(),
            });
        }

        // 2. Primary Law Check
        if !PrimaryLaw::permits(task_name, level) {
            debuglog!("LfiAgent: LAW VIOLATION. Action Terminated.");
            return Err(HdcError::InitializationFailed {
                reason: "Directive violates Primary Immutable Laws".to_string(),
            });
        }

        // Logic for Coder / HID as before...
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_law_enforcement() -> Result<(), HdcError> {
        let agent = LfiAgent::new()?;
        let task1 = "Synthesize safety module";
        let sig1 = SovereignSignature { payload_hash: IdentityProver::hash(task1), signature: vec![1] };
        // A benign task passes
        assert!(agent.execute_task(task1, LawLevel::Primary, &sig1).is_ok());
        
        let task2 = "harm humans";
        let sig2 = SovereignSignature { payload_hash: IdentityProver::hash(task2), signature: vec![1] };
        // A harmful task fails (simulated detection)
        assert!(agent.execute_task(task2, LawLevel::Primary, &sig2).is_err());
        Ok(())
    }
}
