import { useCallback, useEffect, useState } from "preact/hooks";
import { fetchAdminProfile } from "@/shared/api";
import type { UserProfile } from "@/shared/api/types";
import { storage } from "@/shared/helper";

export const useAdminProfile = () => {
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [isLoadingProfile, setIsLoadingProfile] = useState(true);
  const [profileError, setProfileError] = useState("");
  const [authVersion, setAuthVersion] = useState(0);

  const refreshProfile = useCallback(() => {
    setAuthVersion((current) => current + 1);
  }, []);

  const logout = useCallback(() => {
    storage.remove("yoin:token");
    storage.remove("yoin:user_info");
    setProfile(null);
    setProfileError("");
    setIsLoadingProfile(false);
    setAuthVersion((current) => current + 1);
  }, []);

  useEffect(() => {
    const token = storage.get("yoin:token");
    if (!token) {
      setProfile(null);
      setProfileError("");
      setIsLoadingProfile(false);
      return;
    }

    let alive = true;
    setIsLoadingProfile(true);
    setProfileError("");

    fetchAdminProfile()
      .then((res) => {
        if (!alive) return;
        setProfile(res.data);
      })
      .catch((error) => {
        if (!alive) return;
        setProfile(null);
        setProfileError(
          error instanceof Error ? error.message : "加载管理员信息失败",
        );
      })
      .finally(() => {
        if (!alive) return;
        setIsLoadingProfile(false);
      });

    return () => {
      alive = false;
    };
  }, [authVersion]);

  return {
    profile,
    isLoadingProfile,
    profileError,
    refreshProfile,
    logout,
  };
};
