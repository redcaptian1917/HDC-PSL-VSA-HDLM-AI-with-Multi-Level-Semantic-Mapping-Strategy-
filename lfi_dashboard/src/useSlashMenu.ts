import { useCallback, useState } from 'react';

// c2-433 / #313 (App.tsx hooks refactor pass 5): slash-command menu state.
// The menu pops above the chat input when the user types a leading slash;
// shows command suggestions filtered by the typed prefix; arrow keys move
// the highlight; Enter / Tab runs the picked command.
//
// Three coupled slots:
//   show   — open/closed
//   filter — case-insensitive substring after the leading "/"
//   index  — currently-highlighted suggestion (clamped by the renderer
//            against the filtered command list length)
//
// Invariants the hook enforces:
//   - opening always reseeds index=0 (don't carry a stale highlight from a
//     previous open)
//   - changing the filter resets index=0 (different filter ⇒ different
//     ordering)
//   - moveUp/Down clamp at the boundaries

export interface SlashMenuAPI {
  show: boolean;
  filter: string;
  index: number;
  open: (filter?: string) => void;
  close: () => void;
  setFilter: (s: string) => void;
  moveUp: () => void;
  moveDown: (maxLen: number) => void;
  setIndex: React.Dispatch<React.SetStateAction<number>>;
}

export function useSlashMenu(): SlashMenuAPI {
  const [show, setShow] = useState<boolean>(false);
  const [filter, setFilterState] = useState<string>('');
  const [index, setIndex] = useState<number>(0);

  const open = useCallback((f: string = '') => {
    setShow(true);
    setFilterState(f);
    setIndex(0);
  }, []);

  const close = useCallback(() => { setShow(false); }, []);

  const setFilter = useCallback((s: string) => {
    setFilterState(s);
    setIndex(0);
  }, []);

  const moveUp = useCallback(() => {
    setIndex(i => Math.max(i - 1, 0));
  }, []);

  const moveDown = useCallback((maxLen: number) => {
    setIndex(i => Math.min(i + 1, Math.max(0, maxLen - 1)));
  }, []);

  return { show, filter, index, open, close, setFilter, moveUp, moveDown, setIndex };
}
