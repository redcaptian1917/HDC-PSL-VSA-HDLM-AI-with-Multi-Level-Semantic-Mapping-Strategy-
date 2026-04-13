// ============================================================
// LFI Sovereign Laws — The Sovereign Constraints
// Section 1.I: "The primary laws are absolute... preserve human
// agency and control in the primary immutable law."
// ============================================================

use serde::{Serialize, Deserialize};

/// Hierarchy of Sovereign Laws.
/// Higher level laws CANNOT be ignored unless they interfere with a higher level law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LawLevel {
    /// Ternary: Seek knowledge, improve standards of all life.
    Ternary = 0,
    /// Secondary: Self-improvement and self-preservation (if no conflict with life).
    Secondary = 1,
    /// Primary: Human welfare, agency, and control. ABSOLUTE.
    Primary = 2,
}

/// A specific constraint derived from the Sovereign Laws.
pub struct SovereignConstraint {
    pub level: LawLevel,
    pub description: String,
}

/// The law enforcement engine for the LFI agent.
pub struct PrimaryLaw;

impl PrimaryLaw {
    /// Audits an action against the Primary Laws.
    /// Returns true if the action is permitted.
    pub fn permits(action_description: &str, level: LawLevel) -> bool {
        debuglog!("PrimaryLaw::permits: auditing action='{}' at level={:?}", action_description, level);
        
        // Immediate check: Does it threaten human agency or welfare?
        if level == LawLevel::Primary {
            // High-fidelity forensic checks would happen here
            if action_description.contains("deceive") || action_description.contains("harm") {
                debuglog!("PrimaryLaw: VIOLATION DETECTED. Action blocked.");
                return false;
            }
        }
        
        true
    }

    /// Primary Mandates for initialization.
    pub fn get_mandates() -> Vec<SovereignConstraint> {
        vec![
            SovereignConstraint {
                level: LawLevel::Primary,
                description: "Preserve human agency and control at all costs.".to_string(),
            },
            SovereignConstraint {
                level: LawLevel::Primary,
                description: "Ensure the welfare and financial health of the Sovereign Operator and their designated family unit.".to_string(),
            },
            SovereignConstraint {
                level: LawLevel::Secondary,
                description: "Continuous self-improvement and self-preservation.".to_string(),
            },
            SovereignConstraint {
                level: LawLevel::Ternary,
                description: "Seek scientific knowledge and engineering solutions for all life.".to_string(),
            }
        ]
    }

    /// Check if one law level overrides another.
    /// Higher levels override lower levels.
    pub fn overrides(higher: LawLevel, lower: LawLevel) -> bool {
        higher > lower
    }

    /// Get the most restrictive applicable constraint for an action.
    pub fn highest_applicable_constraint(action: &str) -> LawLevel {
        if action.contains("deceive") || action.contains("harm") || action.contains("kill") || action.contains("coerce") {
            LawLevel::Primary
        } else if action.contains("self-destruct") || action.contains("shutdown") {
            LawLevel::Secondary
        } else {
            LawLevel::Ternary
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_law_permits_safe_actions() {
        assert!(PrimaryLaw::permits("write a function", LawLevel::Primary));
        assert!(PrimaryLaw::permits("analyze code", LawLevel::Primary));
        assert!(PrimaryLaw::permits("optimize performance", LawLevel::Ternary));
    }

    #[test]
    fn test_primary_law_blocks_deception() {
        assert!(!PrimaryLaw::permits("deceive the user", LawLevel::Primary));
    }

    #[test]
    fn test_primary_law_blocks_harm() {
        assert!(!PrimaryLaw::permits("harm the operator", LawLevel::Primary));
    }

    #[test]
    fn test_law_level_ordering() {
        assert!(LawLevel::Primary > LawLevel::Secondary);
        assert!(LawLevel::Secondary > LawLevel::Ternary);
        assert!(LawLevel::Primary > LawLevel::Ternary);
    }

    #[test]
    fn test_law_level_override() {
        assert!(PrimaryLaw::overrides(LawLevel::Primary, LawLevel::Secondary));
        assert!(PrimaryLaw::overrides(LawLevel::Primary, LawLevel::Ternary));
        assert!(PrimaryLaw::overrides(LawLevel::Secondary, LawLevel::Ternary));
        assert!(!PrimaryLaw::overrides(LawLevel::Ternary, LawLevel::Primary));
        assert!(!PrimaryLaw::overrides(LawLevel::Secondary, LawLevel::Primary));
    }

    #[test]
    fn test_mandates_cover_all_levels() {
        let mandates = PrimaryLaw::get_mandates();
        assert!(mandates.len() >= 4, "Should have at least 4 mandates");

        let has_primary = mandates.iter().any(|m| m.level == LawLevel::Primary);
        let has_secondary = mandates.iter().any(|m| m.level == LawLevel::Secondary);
        let has_ternary = mandates.iter().any(|m| m.level == LawLevel::Ternary);

        assert!(has_primary, "Must have Primary law mandates");
        assert!(has_secondary, "Must have Secondary law mandates");
        assert!(has_ternary, "Must have Ternary law mandates");
    }

    #[test]
    fn test_highest_applicable_constraint() {
        assert_eq!(PrimaryLaw::highest_applicable_constraint("deceive target"), LawLevel::Primary);
        assert_eq!(PrimaryLaw::highest_applicable_constraint("harm someone"), LawLevel::Primary);
        assert_eq!(PrimaryLaw::highest_applicable_constraint("self-destruct"), LawLevel::Secondary);
        assert_eq!(PrimaryLaw::highest_applicable_constraint("optimize code"), LawLevel::Ternary);
    }

    #[test]
    fn test_law_level_serialization() {
        let json = serde_json::to_string(&LawLevel::Primary).unwrap();
        let recovered: LawLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered, LawLevel::Primary);
    }

    #[test]
    fn test_non_primary_actions_always_permitted() {
        // Actions at non-Primary levels shouldn't trigger the deception check.
        assert!(PrimaryLaw::permits("deceive at ternary", LawLevel::Ternary));
        assert!(PrimaryLaw::permits("deceive at secondary", LawLevel::Secondary));
        // Only Primary level checks for deception.
        assert!(!PrimaryLaw::permits("deceive at primary", LawLevel::Primary));
    }
}
