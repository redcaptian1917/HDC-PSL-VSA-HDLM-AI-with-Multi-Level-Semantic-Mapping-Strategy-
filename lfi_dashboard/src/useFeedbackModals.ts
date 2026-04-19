import { useCallback, useState } from 'react';

// c2-433 / #313 (App.tsx hooks refactor pass 2): negative + correct feedback
// modal state lifted from App.tsx. Both modals are mutually exclusive in
// practice (one opens at a time) but each carries its own draft text + target
// metadata, so we keep two independent state slots and expose discrete
// open/close helpers.
//
// Why grouped: both modals branch from a single AssistantMessage callback
// triplet (positive / negative / correct). Co-locating their state makes the
// component-side "what flow am I in?" obvious and gives one cancel-all
// callback for the Esc handler chain.

export interface NegFeedbackTarget { msgId: number; conclusionId?: number }
export interface CorrectFeedbackTarget {
  msgId: number;
  conclusionId?: number;
  lfiReply: string;
  userQuery?: string;
}

export interface FeedbackModalsAPI {
  // Negative feedback
  negFeedbackFor: NegFeedbackTarget | null;
  negFeedbackCategory: string;
  negFeedbackText: string;
  setNegFeedbackCategory: (s: string) => void;
  setNegFeedbackText: (s: string) => void;
  openNegFeedback: (target: NegFeedbackTarget) => void;
  closeNegFeedback: () => void;
  // Correct (teach) feedback
  correctFeedbackFor: CorrectFeedbackTarget | null;
  correctFeedbackText: string;
  setCorrectFeedbackText: (s: string) => void;
  openCorrectFeedback: (target: CorrectFeedbackTarget) => void;
  closeCorrectFeedback: () => void;
  // Composite: close whichever (or both) is open. Used by the global Esc
  // handler chain so it doesn't need to know which modal is up.
  closeAll: () => void;
}

const NEG_DEFAULT_CATEGORY = 'Incorrect';

export function useFeedbackModals(): FeedbackModalsAPI {
  const [negFeedbackFor, setNegFeedbackFor] = useState<NegFeedbackTarget | null>(null);
  const [negFeedbackCategory, setNegFeedbackCategory] = useState<string>(NEG_DEFAULT_CATEGORY);
  const [negFeedbackText, setNegFeedbackText] = useState<string>('');

  const [correctFeedbackFor, setCorrectFeedbackFor] = useState<CorrectFeedbackTarget | null>(null);
  const [correctFeedbackText, setCorrectFeedbackText] = useState<string>('');

  const openNegFeedback = useCallback((target: NegFeedbackTarget) => {
    setNegFeedbackFor(target);
    setNegFeedbackCategory(NEG_DEFAULT_CATEGORY);
    setNegFeedbackText('');
  }, []);
  const closeNegFeedback = useCallback(() => {
    setNegFeedbackFor(null);
  }, []);

  const openCorrectFeedback = useCallback((target: CorrectFeedbackTarget) => {
    setCorrectFeedbackFor(target);
    setCorrectFeedbackText('');
  }, []);
  const closeCorrectFeedback = useCallback(() => {
    setCorrectFeedbackFor(null);
  }, []);

  const closeAll = useCallback(() => {
    setNegFeedbackFor(null);
    setCorrectFeedbackFor(null);
  }, []);

  return {
    negFeedbackFor, negFeedbackCategory, negFeedbackText,
    setNegFeedbackCategory, setNegFeedbackText,
    openNegFeedback, closeNegFeedback,
    correctFeedbackFor, correctFeedbackText,
    setCorrectFeedbackText,
    openCorrectFeedback, closeCorrectFeedback,
    closeAll,
  };
}
