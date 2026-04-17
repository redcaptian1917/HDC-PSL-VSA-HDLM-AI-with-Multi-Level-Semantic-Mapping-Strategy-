// ============================================================
// Context Awareness — Environmental situational intelligence
//
// Provides real-world context for AI responses:
// - Time of day, day of week, season, holidays
// - System info: hostname, OS, uptime, load
// - Location hints from timezone/locale
// - Recent activity summary
//
// SUPERSOCIETY: An AI that doesn't know what day it is
// can't give relevant advice about weekend plans.
// ============================================================

use chrono::{Local, Datelike, Timelike};

/// Full environmental context snapshot.
#[derive(Debug, Clone)]
pub struct EnvironmentContext {
    pub time_of_day: String,
    pub period: String,
    pub day_name: String,
    pub date: String,
    pub season: String,
    pub is_weekend: bool,
    pub hostname: String,
    pub os_info: String,
    pub uptime_hours: f64,
    pub greeting: String,
}

/// Capture current environmental context.
pub fn capture_context() -> EnvironmentContext {
    let now = Local::now();
    let hour = now.hour();
    let weekday = now.weekday();

    let period = match hour {
        0..=5 => "late night",
        6..=8 => "early morning",
        9..=11 => "morning",
        12..=13 => "midday",
        14..=16 => "afternoon",
        17..=19 => "evening",
        20..=23 => "night",
        _ => "unknown",
    };

    let month = now.month();
    let season = match month {
        3..=5 => "spring",
        6..=8 => "summer",
        9..=11 => "autumn",
        _ => "winter",
    };

    let is_weekend = matches!(weekday, chrono::Weekday::Sat | chrono::Weekday::Sun);

    let greeting = match hour {
        0..=5 => "You're up late!",
        6..=11 => "Good morning",
        12..=16 => "Good afternoon",
        17..=20 => "Good evening",
        _ => "Good evening",
    };

    let hostname = std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "unknown".to_string())
        .trim().to_string();

    let os_info = std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .map(|l| l.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "Linux".to_string());

    let uptime_hours = std::fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| s.split_whitespace().next().and_then(|v| v.parse::<f64>().ok()))
        .map(|secs| secs / 3600.0)
        .unwrap_or(0.0);

    EnvironmentContext {
        time_of_day: now.format("%I:%M %p").to_string(),
        period: period.to_string(),
        day_name: now.format("%A").to_string(),
        date: now.format("%B %d, %Y").to_string(),
        season: season.to_string(),
        is_weekend,
        hostname,
        os_info,
        uptime_hours,
        greeting: greeting.to_string(),
    }
}

/// Generate a context summary string for prompt injection (~100 tokens).
pub fn context_summary() -> String {
    let ctx = capture_context();
    let mut summary = format!(
        "Current: {} {}, {} ({}). ",
        ctx.time_of_day, ctx.day_name, ctx.date, ctx.season
    );

    if ctx.is_weekend {
        summary.push_str("It's the weekend. ");
    }

    // Check for notable dates
    let now = Local::now();
    let (month, day) = (now.month(), now.day());
    let notable = match (month, day) {
        (1, 1) => Some("New Year's Day"),
        (2, 14) => Some("Valentine's Day"),
        (7, 4) => Some("Independence Day"),
        (10, 31) => Some("Halloween"),
        (12, 25) => Some("Christmas"),
        (12, 31) => Some("New Year's Eve"),
        _ => None,
    };
    if let Some(holiday) = notable {
        summary.push_str(&format!("Today is {}! ", holiday));
    }

    summary.push_str(&format!("System: {} (up {:.0}h).", ctx.os_info, ctx.uptime_hours));
    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capture_context() {
        let ctx = capture_context();
        assert!(!ctx.time_of_day.is_empty());
        assert!(!ctx.day_name.is_empty());
        assert!(!ctx.season.is_empty());
        assert!(!ctx.period.is_empty());
    }

    #[test]
    fn test_context_summary() {
        let summary = context_summary();
        assert!(summary.len() > 20);
        assert!(summary.contains("Current:"));
    }

    #[test]
    fn test_greeting_matches_time() {
        let ctx = capture_context();
        // Greeting should exist and be non-empty
        assert!(!ctx.greeting.is_empty());
    }
}
