import React, { useMemo } from 'react';
import { T } from './tokens';

// Shown in the chat area when there are no messages yet. Six quick-start
// prompts, minimal copy. Parent owns the input textarea + ref; we pre-fill.
// c0-020: onboarding state — welcoming, professional, prompt cards as
// clickable launchpads.
// #93 contextual: when the user has a recent non-empty conversation,
// surface a "Continue where you left off" card so returning visits feel
// like a natural resumption rather than a blank slate.
// c2-236 / #20: migrated hardcoded spacing/radii/typography to tokens.ts.
export interface WelcomeScreenProps {
  C: any;
  isDesktop: boolean;
  onPickPrompt: (text: string) => void;
  recentContext?: { title: string; lastUserMsg?: string } | null;
}

// c0-023 fix: prior preset prompts produced poor responses from the
// backend. Claude 0 recommended simpler, more direct starters that let the
// AI lean on its RAG/knowledge context. Keep every starter answerable with
// the facts we've ingested.
// c2-394 / task 196: larger pool, random 6 per mount (seeded in useMemo so
// re-renders during a single welcome visit stay stable). Reduces "seen
// these already" repetition for returning users.
const QUICK_START_POOL: { t: string; p: string }[] = [
  { t: 'Capabilities', p: 'What can you do? List your skills and how you work.' },
  { t: 'Security check', p: 'Help me think through the security of my Linux setup.' },
  { t: 'Analyse my system', p: 'Walk me through interpreting my CPU, RAM, and disk usage.' },
  { t: 'Explain a topic', p: 'What do you know about sovereign AI and local-first systems?' },
  { t: 'Code help', p: 'Help me debug a Rust program. I will paste the error next.' },
  { t: 'Learn something', p: 'Teach me something useful about networking I probably do not know.' },
  { t: 'Regex help', p: 'Explain a regex I will paste — break it down token by token.' },
  { t: 'Shell one-liner', p: 'Write a shell one-liner for a task I will describe.' },
  { t: 'Research a topic', p: 'Give me a structured overview of a topic with citations.' },
  { t: 'Git diagnosis', p: 'Walk me through fixing a git state I will describe.' },
  { t: 'Compare options', p: 'Compare two tools / approaches with a pros-and-cons table.' },
  { t: 'Translate', p: 'Translate text I paste — preserve formatting and tone.' },
];
// Fisher–Yates pick: stable for a given visit, fresh per mount.
const pickStarters = (pool: typeof QUICK_START_POOL, n: number): typeof QUICK_START_POOL => {
  const copy = pool.slice();
  for (let i = copy.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [copy[i], copy[j]] = [copy[j], copy[i]];
  }
  return copy.slice(0, n);
};

export const WelcomeScreen: React.FC<WelcomeScreenProps> = ({ C, isDesktop, onPickPrompt, recentContext }) => {
  // useMemo with [] dep array so the random pick is stable across re-renders
  // of THIS component instance. New mount (e.g. switching to an empty convo)
  // rolls fresh.
  const starters = useMemo(() => pickStarters(QUICK_START_POOL, 6), []);
  return (
  <div style={{ textAlign: 'center', padding: isDesktop ? `72px ${T.spacing.xl} 40px` : `40px ${T.spacing.xl} ${T.spacing.xl}` }}>
    <h1 style={{
      fontSize: isDesktop ? '28px' : T.typography.size3xl,
      fontWeight: T.typography.weightSemibold, color: C.text,
      margin: `0 0 ${T.spacing.sm}`, letterSpacing: T.typography.trackingTight,
    }}>
      PlausiDen <span style={{ color: C.accent }}>AI</span>
    </h1>
    <p style={{
      fontSize: T.typography.sizeBody, color: C.textSecondary,
      margin: `0 0 28px`, maxWidth: '440px', marginLeft: 'auto', marginRight: 'auto',
      lineHeight: T.typography.lineNormal,
    }}>
      Sovereign AI that runs on your hardware. Private by default, remembers across sessions.
    </p>
    {/* Contextual continuation card — shown only when a recent conversation
        exists. Pre-fills a resumption prompt so users don't stare at an
        empty blank page on return visits. */}
    {recentContext && (
      <button
        onClick={() => onPickPrompt(`Continuing from "${recentContext.title}": `)}
        aria-label={`Continue conversation: ${recentContext.title}`}
        style={{
          display: 'block', textAlign: 'left',
          maxWidth: '720px', width: '100%', margin: `0 auto ${T.spacing.md}`,
          padding: `${T.spacing.lg} 18px`, borderRadius: T.radii.md,
          background: C.accentBg, border: `1px solid ${C.accentBorder}`, cursor: 'pointer',
          fontFamily: 'inherit', color: C.text,
          transition: `border-color ${T.motion.fast}`,
        }}
        onMouseEnter={(e) => { e.currentTarget.style.borderColor = C.accent; }}
        onMouseLeave={(e) => { e.currentTarget.style.borderColor = C.accentBorder; }}
      >
        <div style={{
          fontSize: T.typography.sizeXs, color: C.accent,
          fontWeight: T.typography.weightSemibold,
          marginBottom: '6px', textTransform: 'uppercase', letterSpacing: '0.04em',
        }}>
          Continue where you left off
        </div>
        <div style={{
          fontSize: '13.5px', color: C.text, fontWeight: T.typography.weightMedium,
          whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis',
        }}>
          {recentContext.title}
        </div>
        {recentContext.lastUserMsg && (
          <div style={{
            fontSize: T.typography.sizeSm, color: C.textSecondary, marginTop: T.spacing.xs,
            whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis',
          }}>
            Last: “{recentContext.lastUserMsg}”
          </div>
        )}
      </button>
    )}
    <div style={{
      display: 'grid',
      gridTemplateColumns: isDesktop ? 'repeat(3, 1fr)' : 'repeat(2, 1fr)',
      gap: '10px', maxWidth: '720px', margin: '0 auto',
    }}>
      {starters.map(s => (
        <button key={s.t}
          onClick={() => onPickPrompt(s.p)}
          aria-label={`${s.t}: ${s.p}`}
          style={{
            textAlign: 'left', padding: `${T.spacing.lg} ${T.spacing.lg}`, borderRadius: T.radii.md,
            background: C.bgCard, border: `1px solid ${C.border}`, cursor: 'pointer',
            fontFamily: 'inherit', color: C.text,
            transition: `border-color ${T.motion.fast}, background ${T.motion.fast}`,
          }}
          onMouseEnter={(e) => { e.currentTarget.style.borderColor = C.accent; e.currentTarget.style.background = C.bgHover; }}
          onMouseLeave={(e) => { e.currentTarget.style.borderColor = C.border; e.currentTarget.style.background = C.bgCard; }}
        >
          <div style={{
            fontSize: T.typography.sizeXs, color: C.accent, fontWeight: T.typography.weightSemibold,
            marginBottom: '6px', textTransform: 'uppercase', letterSpacing: '0.04em',
          }}>{s.t}</div>
          <div style={{
            fontSize: T.typography.sizeMd, color: C.textSecondary, lineHeight: 1.5,
          }}>{s.p}</div>
        </button>
      ))}
    </div>
  </div>
  );
};
