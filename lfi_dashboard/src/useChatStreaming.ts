import { useCallback, useEffect, useState } from 'react';

// c2-433 / #313 (App.tsx hooks refactor pass 8): chat streaming telemetry
// + the retry affordance for the most-recently-failed turn.
//
// Streaming side:
//   timing.startAt — epoch ms of the first chat_chunk (null when no stream)
//   timing.chars   — accumulated character count across chunks this turn
//   tick           — increments every 500ms while a stream is live; UI deps
//                    on it to refresh the chars/s readout without re-
//                    rendering the whole tree
// Error side:
//   lastError      — { userContent, at } for the prompt that errored last;
//                    used to render a retry chip above the input bar
//
// Both clear on the next successful turn (handleSend gates on !lastError or
// clears it explicitly). The hook bundles the tick interval so callers
// don't have to wire a useEffect — and the interval is automatically
// cleared when the stream ends.

export interface StreamTiming { startAt: number; chars: number }
export interface LastErrorRetry { userContent: string; at: number }

export interface ChatStreamingAPI {
  timing: StreamTiming | null;
  tick: number;
  lastError: LastErrorRetry | null;
  // chars/s computed from the live timing — null when no stream.
  charsPerSecond: number | null;
  // Begin a fresh stream — wipes any prior tracking state.
  begin: () => void;
  // Append `n` chars to the current stream. Idempotent if no stream is
  // active (call sites can do this from the chunk handler without a guard).
  growBy: (n: number) => void;
  // End the current stream — clears timing + tick.
  end: () => void;
  setLastError: (e: LastErrorRetry | null) => void;
}

export function useChatStreaming(): ChatStreamingAPI {
  const [timing, setTiming] = useState<StreamTiming | null>(null);
  const [tick, setTick] = useState<number>(0);
  const [lastError, setLastError] = useState<LastErrorRetry | null>(null);

  // Tick once per 500ms while a stream is live. Effect re-runs only when
  // the stream becomes active or inactive (boolean dep) — not on every
  // tick or chars-grow.
  useEffect(() => {
    if (!timing) return;
    const id = setInterval(() => setTick(t => t + 1), 500);
    return () => clearInterval(id);
  }, [timing != null]);

  const begin = useCallback(() => {
    setTiming({ startAt: Date.now(), chars: 0 });
    setTick(0);
  }, []);

  const growBy = useCallback((n: number) => {
    if (n <= 0) return;
    setTiming(prev => prev
      ? { ...prev, chars: prev.chars + n }
      : { startAt: Date.now(), chars: n });
  }, []);

  const end = useCallback(() => {
    setTiming(null);
  }, []);

  const charsPerSecond = timing != null
    ? Math.round(timing.chars / Math.max(0.5, (Date.now() - timing.startAt) / 1000))
    : null;

  return { timing, tick, lastError, charsPerSecond, begin, growBy, end, setLastError };
}
