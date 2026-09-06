import { useEffect, useState } from "preact/hooks";
import { useCommentStore } from "@/client/store";
import { getRuntimeConfig } from "@/config/runtime";

const LOCATION_CHANGE_EVENT = "yoin:locationchange";

const emitLocationChange = () => {
  window.dispatchEvent(new Event(LOCATION_CHANGE_EVENT));
};

const patchHistoryMethods = () => {
  const historyState = window.history as History & {
    __yoinLocationPatched__?: boolean;
  };

  if (historyState.__yoinLocationPatched__) {
    return;
  }

  const originalPushState = window.history.pushState.bind(window.history);
  const originalReplaceState = window.history.replaceState.bind(window.history);

  window.history.pushState = (...args) => {
    originalPushState(...args);
    emitLocationChange();
  };

  window.history.replaceState = (...args) => {
    originalReplaceState(...args);
    emitLocationChange();
  };

  historyState.__yoinLocationPatched__ = true;
};

const useCommentPagePath = () => {
  const [pathname, setPathname] = useState(() => location.pathname);

  useEffect(() => {
    patchHistoryMethods();

    const syncPathname = () => {
      setPathname(location.pathname);
    };

    window.addEventListener("popstate", syncPathname);
    window.addEventListener(LOCATION_CHANGE_EVENT, syncPathname);

    return () => {
      window.removeEventListener("popstate", syncPathname);
      window.removeEventListener(LOCATION_CHANGE_EVENT, syncPathname);
    };
  }, []);

  return pathname;
};

const useInitializeComments = (siteId?: number) => {
  const fetchComments = useCommentStore((state) => state.fetchComments);
  const fetchPageReactions = useCommentStore(
    (state) => state.fetchPageReactions,
  );
  const fetchSiteConfig = useCommentStore((state) => state.fetchSiteConfig);
  const pathname = useCommentPagePath();

  useEffect(() => {
    if (siteId == null) {
      return;
    }

    fetchSiteConfig();
    fetchComments(1, false);
    fetchPageReactions();
  }, [fetchComments, fetchPageReactions, fetchSiteConfig, pathname, siteId]);
};

export const useInitializeCommentPage = () => {
  const siteId = getRuntimeConfig().site_id;

  useInitializeComments(siteId);
};
