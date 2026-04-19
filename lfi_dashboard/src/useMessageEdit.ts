import { useCallback, useState } from 'react';

// c2-433 / #313 (App.tsx hooks refactor pass 7): user-message edit-in-place
// state. When the user picks "Edit and resend" on their own bubble, the
// bubble swaps for a textarea seeded with the original content. Two slots
// move together:
//   editingId — the msg.id whose bubble is in edit mode (null = nobody)
//   draft     — the current textarea content
//
// Hook collapses the begin/cancel/commit lifecycle into named helpers so
// the bubble component + the right-click context menu both fire one call.
// commit returns the trimmed value or null (empty cancels), so callers can
// gate the resend on a valid string without redoing the trim.

export interface MessageEditAPI {
  editingId: number | null;
  draft: string;
  setDraft: (s: string) => void;
  begin: (id: number, content: string) => void;
  cancel: () => void;
  // Returns the trimmed draft or null if empty (caller should treat null as
  // "cancel"). Always exits edit mode.
  commit: () => string | null;
}

export function useMessageEdit(): MessageEditAPI {
  const [editingId, setEditingId] = useState<number | null>(null);
  const [draft, setDraft] = useState<string>('');

  const begin = useCallback((id: number, content: string) => {
    setEditingId(id);
    setDraft(content);
  }, []);

  const cancel = useCallback(() => {
    setEditingId(null);
    setDraft('');
  }, []);

  const commit = useCallback((): string | null => {
    setEditingId(null);
    const trimmed = draft.trim();
    setDraft('');
    return trimmed || null;
  }, [draft]);

  return { editingId, draft, setDraft, begin, cancel, commit };
}
