/**
 * accessTier.ts — stub for the future owner/admin/subscriber/user
 * access model. See docs/ACCESS_TIERS.md for the full specification.
 *
 * TODAY: every client hydrates as `owner` because self-hosted operators
 * are the only deployment shape and the backend has no auth layer yet.
 *
 * TOMORROW: replace the hook body with an /api/me fetch + JWT claim
 * parse. Gated component `<Gate tier="...">` is the UX primitive.
 *
 * This file is intentionally minimal — real gating lands in one shot
 * when the backend auth model ships.
 */
import React, { useEffect, useState } from 'react';

export type Tier = 'anonymous' | 'user' | 'subscriber' | 'admin' | 'owner';

const TIER_RANK: Record<Tier, number> = {
  anonymous: 0,
  user: 1,
  subscriber: 2,
  admin: 3,
  owner: 4,
};

/**
 * Returns the current user's access tier. Today: always 'owner' on
 * self-hosted. Future: reads a JWT claim from the backend auth layer.
 */
export function useAccessTier(): Tier {
  const [tier, setTier] = useState<Tier>(() => {
    // Future: const t = parseJwt(localStorage.getItem('lfi_auth_token'))?.tier;
    return 'owner';
  });
  useEffect(() => {
    // Future: fetch /api/me and update tier.
    // fetch('/api/me').then(r => r.json()).then(d => d?.tier && setTier(d.tier));
  }, []);
  return tier;
}

/**
 * hasAccess — true if the current tier is at least `required`.
 * Works with the ladder anonymous < user < subscriber < admin < owner.
 */
export function hasAccess(current: Tier, required: Tier): boolean {
  return TIER_RANK[current] >= TIER_RANK[required];
}

/**
 * <Gate tier="admin">...</Gate> — renders children only when the user's
 * tier meets the requirement. Falls back to `fallback` prop (default
 * null, rendering nothing).
 *
 *   <Gate tier="owner"><SubstrateTelemetry ... /></Gate>
 *
 * Currently a no-op since useAccessTier always returns 'owner', but
 * putting the gate in now means the switch lands cleanly when auth
 * arrives.
 */
export interface GateProps {
  tier: Tier;
  fallback?: React.ReactNode;
  children: React.ReactNode;
}

export const Gate: React.FC<GateProps> = ({ tier, fallback = null, children }) => {
  const current = useAccessTier();
  if (!hasAccess(current, tier)) return <>{fallback}</>;
  return <>{children}</>;
};
