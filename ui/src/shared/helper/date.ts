export const formatDate = (dateStr: string) => {
  try {
    const date = new Date(dateStr);
    const now = new Date();
    const diffInSeconds = Math.floor((now.getTime() - date.getTime()) / 1000);

    if (diffInSeconds < 0) return "刚刚";
    if (diffInSeconds < 60) return "刚刚";
    const diffInMinutes = Math.floor(diffInSeconds / 60);
    if (diffInMinutes < 60) return `${diffInMinutes}分钟前`;

    const diffInHours = Math.floor(diffInMinutes / 60);
    if (diffInHours < 24 && date.getDate() === now.getDate()) {
      return `${diffInHours}小时前`;
    }

    const yesterday = new Date(now);
    yesterday.setDate(now.getDate() - 1);
    if (date.toDateString() === yesterday.toDateString()) {
      return `昨天 ${date.getHours().toString().padStart(2, "0")}:${date.getMinutes().toString().padStart(2, "0")}`;
    }

    return date.toLocaleString("zh-CN", {
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
    return date.toLocaleString(undefined, {
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
