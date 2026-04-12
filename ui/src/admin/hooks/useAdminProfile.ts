import { useEffect, useState } from "preact/hooks";
import { fetchAdminProfile } from "@/shared/api";
import type { UserProfile } from "@/shared/api/types";

export const useAdminProfile = () => {
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [isLoadingProfile, setIsLoadingProfile] = useState(true);
  const [profileError, setProfileError] = useState("");

  useEffect(() => {
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
  }, []);

  return {
    profile,
    isLoadingProfile,
    profileError,
  };
};
