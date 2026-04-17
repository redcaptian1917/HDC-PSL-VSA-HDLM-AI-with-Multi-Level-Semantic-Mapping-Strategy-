# The Classroom — PlausiDen AI Training Center

## Concept

The "Classroom" is the dedicated area for everything related to teaching the AI.
Think: what does a teacher need? That's what goes here.

## What a Teacher Needs

### 1. Student Profile (AI Current State)
- Overall grade (A+ to D, composite score)
- Strengths: domains where accuracy is high
- Weaknesses: domains where accuracy is low or data is thin
- Learning rate: is the AI improving over time?
- Personality assessment: is it repetitive? too formal? not helpful enough?

### 2. Curriculum (Training Data)
- All training datasets with: name, size, domain, quality, time spent training on each
- Which datasets have been completed vs in-progress vs queued
- Import new training data (upload JSONL, paste text, record conversation)
- Curriculum gaps: what topics are missing?

### 3. Gradebook (Performance Tracking)
- Pass/fail rate per domain
- Accuracy trend over time (chart)
- Total training hours logged
- Per-session results: what was taught, how well it was learned
- Comparison: before training vs after training on specific topics

### 4. Lesson Plans (Active Training)
- Currently running training sessions with live progress
- Queue of upcoming training tasks
- Estimated time to completion
- Start/stop/pause controls
- Schedule training for off-peak hours

### 5. Test Center (Evaluation)
- Run benchmark tests on demand
- Quick quiz: ask the AI 10 questions, grade responses
- A/B comparison: test response quality before/after a training set
- Regression detection: did new training break old capabilities?

### 6. Report Cards (Historical)
- Weekly/monthly progress reports
- Training hours per week
- Facts added per week
- Quality improvement trend
- Domain coverage progression

### 7. Office Hours (Feedback Loop)
- Review user feedback (thumbs up/down, categories)
- See which responses were corrected and what the correction was
- Convert corrections into training data automatically
- Flag patterns: "AI keeps getting X wrong"

### 8. Library (Reference)
- Browse the fact database by domain
- Search facts
- View fact quality scores
- Mark facts for review/deletion
- Import new facts from files

## Time Tracking

Every training operation logs:
```json
{
  "session_id": "uuid",
  "started_at": "2026-04-17T05:00:00Z",
  "ended_at": "2026-04-17T05:15:00Z",
  "duration_seconds": 900,
  "dataset": "domain_gap_comprehensive.jsonl",
  "domain": "economics",
  "pairs_processed": 500,
  "source": "ollama_generation",
  "initiated_by": "claude-1",
  "method": "rag_ingestion",
  "result": {
    "facts_added": 480,
    "duplicates_skipped": 20,
    "quality_avg": 0.78
  }
}
```

Stored in brain.db table `training_log`. Displayed in Classroom dashboard.

## API Endpoints

- GET /api/classroom/overview — student profile + grade + strengths/weaknesses
- GET /api/classroom/curriculum — all training datasets with time tracking
- GET /api/classroom/gradebook — pass/fail, accuracy trends, per-domain scores
- GET /api/classroom/lessons — active/queued training sessions
- GET /api/classroom/tests — run benchmarks, get results
- GET /api/classroom/reports — weekly/monthly summaries
- GET /api/classroom/feedback — user feedback review
- GET /api/classroom/library — fact browser with search/filter
- POST /api/classroom/train — start a training session
- POST /api/classroom/test — run a benchmark test
- POST /api/classroom/import — import training data

## UI Layout

Full-screen dedicated section (not a modal — a page):
- Left sidebar: navigation between Classroom sections
- Main area: content for selected section
- Top bar: overall grade badge, training status indicator, quick stats

The Classroom should feel like a real LMS (Learning Management System).
Reference: Google Classroom, Canvas LMS, Coursera instructor dashboard.
