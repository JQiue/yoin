import { getLocale, t } from "@/shared/i18n";

export const formatDate = (dateStr: string) => {
  try {
    const date = new Date(dateStr);
    const now = new Date();
    const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);
    const locale = getLocale() === "zh" ? "zh-CN" : "en-US";

    if (diffInSeconds < 60) return t("date.justNow");
    const diffInMinutes = Math.floor(diffInSeconds / 60);
    if (diffInMinutes < 60)
      return t("date.minutesAgo", { count: diffInMinutes });

    const diffInHours = Math.floor(diffInMinutes / 60);
    if (diffInHours < 24 && date.getDate() === now.getDate()) {
      return t("date.hoursAgo", { count: diffInHours });
    }

    const yesterday = new Date(now);
    yesterday.setDate(now.getDate() - 1);
    if (date.toDateString() === yesterday.toDateString()) {
      const time = `${date.getHours().toString().padStart(2, "0")}:${date.getMinutes().toString().padStart(2, "0")}`;
      return t("date.yesterday", { time });
    }

    return date.toLocaleString(locale, {
      month: "numeric",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
      hour12: false,
    });
  } catch (_e) {
    return dateStr;
  }
};

export const formatLocalDateTime = (dateStr: string) => {
  try {
    const date = new Date(dateStr);
    return date.toLocaleString(getLocale() === "zh" ? "zh-CN" : "en-US", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
      hour12: false,
    });
  } catch (_e) {
    return dateStr;
  }
};

// export const formatDate = (dateStr: string) => {
// 	try {
// 		return new Date(dateStr).toLocaleString("zh-CN", {
// 			month: "numeric",
// 			day: "numeric",
// 			hour: "2-digit",
// 			minute: "2-digit",
// 		});
// 	} catch (_e) {
// 		return dateStr;
// 	}
// };
