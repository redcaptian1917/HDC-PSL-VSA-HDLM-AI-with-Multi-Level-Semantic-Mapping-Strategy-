import { useCallback, useState } from 'react';

// c2-433 / #313 (App.tsx hooks refactor pass 6): pinned-convo drag state.
// The sidebar lets pinned conversations be reordered via HTML5 drag-and-
// drop. Two state slots track the in-flight drag:
//   draggedId  — the convo currently being dragged (null when idle)
//   overId     — the convo the cursor is hovering over (null when idle)
//
// Both reset to null on dragend or drop. Bundling them as one hook lets the
// row renderer + the drop target share one source of truth and call a
// single end() helper instead of two setters.

export interface ConvoDragAPI {
  draggedId: string | null;
  overId: string | null;
  begin: (id: string) => void;
  hover: (id: string) => void;
  leave: (id: string) => void;
  end: () => void;
}

export function useConvoDrag(): ConvoDragAPI {
  const [draggedId, setDraggedId] = useState<string | null>(null);
  const [overId, setOverId] = useState<string | null>(null);

  const begin = useCallback((id: string) => { setDraggedId(id); }, []);

  // Only update on transition — repeated dragover events on the same row
  // would otherwise re-render the whole sidebar 60Hz.
  const hover = useCallback((id: string) => {
    setOverId(prev => prev === id ? prev : id);
  }, []);

  const leave = useCallback((id: string) => {
    setOverId(prev => prev === id ? null : prev);
  }, []);

  const end = useCallback(() => {
    setDraggedId(null);
    setOverId(null);
  }, []);

  return { draggedId, overId, begin, hover, leave, end };
}
