import { useEffect, useState } from "preact/hooks";
import { en, type MessageKey } from "@/shared/i18n/en";
import { zh } from "@/shared/i18n/zh";

export type Locale = "zh" | "en";
export type { MessageKey };

const dictionaries: Record<Locale, Record<MessageKey, string>> = { en, zh };

let currentLocale: Locale = detectLocale();
const listeners = new Set<(locale: Locale) => void>();

export function detectLocale(lang = document.documentElement.lang): Locale {
  return lang.trim().toLowerCase().startsWith("zh") ? "zh" : "en";
}

export function getLocale() {
  return currentLocale;
}

export function setLocale(locale: Locale) {
  if (locale === currentLocale) return;
  currentLocale = locale;
  for (const listener of listeners) {
    listener(locale);
  }
}

export function subscribeLocale(listener: (locale: Locale) => void) {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

export function t(
  key: MessageKey,
  params?: Record<string, string | number>,
  locale = currentLocale,
) {
  let message = dictionaries[locale][key] ?? dictionaries.en[key] ?? key;
  if (params) {
    for (const [name, value] of Object.entries(params)) {
      message = message.replaceAll(`{${name}}`, String(value));
    }
  }
  return message;
}

export function watchHtmlLang() {
  const apply = () => setLocale(detectLocale());
  apply();
  const observer = new MutationObserver(apply);
  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ["lang"],
  });
  return () => observer.disconnect();
}

export function useI18n() {
  const [locale, setLocaleState] = useState(getLocale);
  useEffect(() => subscribeLocale(setLocaleState), []);
  return {
    locale,
    t: (key: MessageKey, params?: Record<string, string | number>) =>
      t(key, params, locale),
  };
}
