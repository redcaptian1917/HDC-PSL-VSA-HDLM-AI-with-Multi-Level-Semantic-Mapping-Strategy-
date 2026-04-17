import React from 'react';
import { T } from './tokens';

// Sidebar "Administration" button cluster. Parent owns loading state + handlers.
// c2-239 / #20: migrated hardcoded spacing/radii/typography to tokens.ts, and
// extracted the repeated button style into a shared factory (4 buttons were
// carrying 8 duplicate properties each).

export interface AdminActionsProps {
  C: any;
  adminLoading: string;
  onFetchFacts: () => void;
  onFetchQos: () => void;
  onClearChat: () => void;
  onOpenSettings: () => void;
  children?: React.ReactNode; // lets the parent slot FactsPanel + QosPanel below the buttons
}

const baseBtnStyle: React.CSSProperties = {
  padding: T.spacing.sm + ' ' + T.spacing.sm,
  fontSize: T.typography.sizeSm,
  fontWeight: T.typography.weightBold,
  borderRadius: T.radii.lg,
  cursor: 'pointer', fontFamily: 'inherit',
  textTransform: 'uppercase', letterSpacing: '0.05em',
};

export const AdminActions: React.FC<AdminActionsProps> = ({
  C, adminLoading, onFetchFacts, onFetchQos, onClearChat, onOpenSettings, children,
}) => (
  <div style={{ padding: T.spacing.xl }}>
    <div style={{
      fontSize: T.typography.sizeXs, fontWeight: T.typography.weightBlack,
      color: C.textMuted, textTransform: 'uppercase',
      letterSpacing: T.typography.trackingCap, marginBottom: T.spacing.lg,
    }}>
      Administration
    </div>
    <div style={{ display: 'flex', flexDirection: 'column', gap: T.spacing.sm }}>
      <button onClick={onFetchFacts} disabled={adminLoading === 'facts'} style={{
        ...baseBtnStyle, color: C.accent,
        background: C.accentBg, border: `1px solid ${C.accentBorder}`,
      }}>{adminLoading === 'facts' ? 'Loading...' : 'View Facts'}</button>
      <button onClick={onFetchQos} disabled={adminLoading === 'qos'} style={{
        ...baseBtnStyle, color: C.purple,
        background: C.purpleBg, border: `1px solid ${C.purpleBorder}`,
      }}>{adminLoading === 'qos' ? 'Loading...' : 'QoS Report'}</button>
      <button onClick={onClearChat} style={{
        ...baseBtnStyle, color: C.textMuted,
        background: 'transparent', border: `1px solid ${C.border}`,
      }}>Clear Chat</button>
      <button onClick={onOpenSettings} style={{
        ...baseBtnStyle, color: C.accent,
        background: 'transparent', border: `1px solid ${C.accentBorder}`,
      }}>Settings</button>
    </div>
    {children}
  </div>
);
