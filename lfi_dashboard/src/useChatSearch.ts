import { useCallback, useRef, useState } from 'react';

// c2-433 / #313 (App.tsx hooks refactor pass 3): in-conversation chat search
// state. Cmd+Shift+F opens the search bar; query filters or highlights matches
// (mode toggle); Enter / Shift+Enter steps through matches via cursor.
//
// Lifted as a single hook because all five slots move together — opening the
// bar should always set focus (after a paint), closing should always reset
// query + cursor, mode flip should reset cursor (different match indices
// across modes don't carry over). The hook bundles those invariants so call
// sites don't rederive them.

export type ChatSearchMode = 'filter' | 'highlight';

export interface ChatSearchAPI {
  query: string;
  show: boolean;
  mode: ChatSearchMode;
  cursor: number;
  inputRef: React.RefObject<HTMLInputElement>;
  setQuery: (s: string) => void;
  setMode: (m: ChatSearchMode) => void;
  setCursor: React.Dispatch<React.SetStateAction<number>>;
  // Show + focus the input on the next paint. Idempotent — safe to call
  // when already open (re-focuses).
  open: () => void;
  // Hide + clear query + reset cursor.
  close: () => void;
  // Toggle and refocus when opening. Returns the new open state so callers
  // can short-circuit on the result.
  toggle: () => boolean;
}

export function useChatSearch(): ChatSearchAPI {
  const [query, setQueryState] = useState<string>('');
  const [show, setShow] = useState<boolean>(false);
  const [mode, setModeState] = useState<ChatSearchMode>('filter');
  const [cursor, setCursor] = useState<number>(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const focusSoon = useCallback(() => {
    setTimeout(() => inputRef.current?.focus(), 0);
  }, []);

  const setQuery = useCallback((s: string) => {
    setQueryState(s);
    // New query → restart match cursor at the top.
    setCursor(0);
  }, []);

  const setMode = useCallback((m: ChatSearchMode) => {
    setModeState(m);
    setCursor(0);
  }, []);

  const open = useCallback(() => {
    setShow(true);
    focusSoon();
  }, [focusSoon]);

  const close = useCallback(() => {
    setShow(false);
    setQueryState('');
    setCursor(0);
  }, []);

  const toggle = useCallback((): boolean => {
    let next = false;
    setShow(curr => { next = !curr; return next; });
    if (next) focusSoon(); else { setQueryState(''); setCursor(0); }
    return next;
  }, [focusSoon]);

  return {
    query, show, mode, cursor, inputRef,
    setQuery, setMode, setCursor,
    open, close, toggle,
  };
}
