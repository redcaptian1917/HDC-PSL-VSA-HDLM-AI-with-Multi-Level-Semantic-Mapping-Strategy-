import { useCallback, useEffect, useState } from 'react';

// c2-433 / #313 (App.tsx hooks refactor pass 4): in-flight chat thinking
// state. While a request is awaiting a backend response, App.tsx tracks:
//   isThinking      — boolean spinner gate
//   thinkingStart   — epoch ms when the turn was sent (drives elapsed timer
//                     + the stuck-state guardrail)
//   thinkingStep    — human-readable progress label (from WS progress msgs)
//   thinkingElapsed — seconds since start, ticking once per second
//   activeModule    — currently-dispatching cognitive module (#316)
//   modulesUsed     — Set of every module that has touched this turn
//
// All six are tightly coupled: handleSend starts everything, chat_done /
// stop / Esc / pendingConfirm-dismiss clear everything. The hook owns the
// elapsed-tick interval (auto-cleaned on stop/unmount) so callers don't
// have to wire it. Setter passthroughs are exposed for the WS handler that
// nudges step/module mid-stream.
//
// start(step?) seeds isThinking+thinkingStart+thinkingStep+modulesUsed
// stop() flips isThinking false + clears module pulse (modulesUsed kept for
//   the post-turn glance until next start)
// recordModule(name) updates activeModule + adds to modulesUsed Set
// reset() = full clear; used by handleSend before start() and by Esc

export interface ThinkingStateAPI {
  isThinking: boolean;
  thinkingStart: number | null;
  thinkingStep: string;
  thinkingElapsed: number;
  activeModule: string | null;
  modulesUsed: Set<string>;
  setThinkingStep: (s: string) => void;
  setActiveModule: (m: string | null) => void;
  setIsThinking: React.Dispatch<React.SetStateAction<boolean>>;
  setThinkingStart: React.Dispatch<React.SetStateAction<number | null>>;
  start: (step?: string) => void;
  stop: () => void;
  recordModule: (m: string) => void;
  reset: () => void;
}

export function useThinkingState(): ThinkingStateAPI {
  const [isThinking, setIsThinking] = useState<boolean>(false);
  const [thinkingStart, setThinkingStart] = useState<number | null>(null);
  const [thinkingStep, setThinkingStep] = useState<string>('');
  const [thinkingElapsed, setThinkingElapsed] = useState<number>(0);
  const [activeModule, setActiveModule] = useState<string | null>(null);
  const [modulesUsed, setModulesUsed] = useState<Set<string>>(() => new Set());

  // Tick elapsed seconds while active. Resets to 0 whenever a new turn
  // starts (thinkingStart change). Cleans up on stop/unmount.
  useEffect(() => {
    if (!isThinking || thinkingStart == null) { setThinkingElapsed(0); return; }
    setThinkingElapsed(0);
    const id = setInterval(() => {
      setThinkingElapsed(Math.floor((Date.now() - thinkingStart) / 1000));
    }, 1000);
    return () => clearInterval(id);
  }, [isThinking, thinkingStart]);

  const start = useCallback((step: string = 'Thinking…') => {
    setIsThinking(true);
    setThinkingStart(Date.now());
    setThinkingStep(step);
    setActiveModule(null);
    setModulesUsed(new Set());
  }, []);

  const stop = useCallback(() => {
    setIsThinking(false);
    setThinkingStart(null);
    setActiveModule(null);
    // modulesUsed is intentionally retained — the post-turn read-out can
    // still show which modules contributed; cleared on next start().
  }, []);

  const reset = useCallback(() => {
    setIsThinking(false);
    setThinkingStart(null);
    setThinkingStep('');
    setActiveModule(null);
    setModulesUsed(new Set());
  }, []);

  const recordModule = useCallback((m: string) => {
    setActiveModule(m);
    setModulesUsed(prev => prev.has(m) ? prev : new Set(prev).add(m));
  }, []);

  return {
    isThinking, thinkingStart, thinkingStep, thinkingElapsed,
    activeModule, modulesUsed,
    setThinkingStep, setActiveModule,
    setIsThinking, setThinkingStart,
    start, stop, recordModule, reset,
  };
}
