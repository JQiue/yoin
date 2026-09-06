import { storage } from "@/shared/helper/storage";

export const GUEST_ID_HEADER = "x-yoin-guest-id";

export function getGuestId() {
  const existing = storage.get("yoin:guest_id");
  if (existing) return existing;
  const created =
    globalThis.crypto?.randomUUID?.().replaceAll("-", "") ??
    `${Date.now().toString(36)}${Math.random().toString(36).slice(2, 12)}`;
  storage.set("yoin:guest_id", created);
  return created;
}

export function guestLabel(guestId = getGuestId()) {
  return `访客 ${guestId.slice(0, 6)}`;
}
