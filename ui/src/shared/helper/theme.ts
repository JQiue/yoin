// 宿主页面往往自己就有主题切换（mdBook、各类文档站），组件需要跟着走。

/** 宿主在 <html> 上用来声明暗色的 class（mdBook 的 coal/navy/ayu 都属暗色） */
const DARK_THEME_NAMES = ["dark", "coal", "navy", "ayu"];
/** 宿主在 <html> 上用来声明浅色的 class（mdBook 的 light/rust 都属浅色） */
const LIGHT_THEME_NAMES = ["light", "rust"];

const hostTheme = (): boolean | null => {
  const root = document.documentElement;
  const declared = root.dataset.theme;
  if (declared === "dark") return true;
  if (declared === "light") return false;
  if (DARK_THEME_NAMES.some((name) => root.classList.contains(name)))
    return true;
  if (LIGHT_THEME_NAMES.some((name) => root.classList.contains(name)))
    return false;
  return null;
};

/** 生效的暗色状态：宿主声明优先，宿主没声明时跟随系统配色。 */
const resolveDarkMode = () =>
  hostTheme() ??
  globalThis.matchMedia?.("(prefers-color-scheme: dark)").matches ??
  false;

/**
 * 把生效的主题映射到容器上的 `yo-dark` 类（样式见 styles/base.css），
 * 并监听宿主主题与系统配色的变化，切换时实时更新。返回取消订阅的函数。
 */
export const watchHostTheme = (element: HTMLElement) => {
  const media = globalThis.matchMedia?.("(prefers-color-scheme: dark)");
  const sync = () => element.classList.toggle("yo-dark", resolveDarkMode());

  const observer = new MutationObserver(sync);
  observer.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ["class", "data-theme"],
  });
  media?.addEventListener("change", sync);
  sync();

  return () => {
    observer.disconnect();
    media?.removeEventListener("change", sync);
  };
};
