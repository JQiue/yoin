export const ANONYMOUS_AVATAR =
  "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'%3E%3Crect width='64' height='64' rx='32' fill='%23e4e4e7'/%3E%3Ccircle cx='32' cy='24' r='10' fill='%2371717a'/%3E%3Cpath d='M12 54c0-11 9-18 20-18s20 7 20 18' fill='%2371717a'/%3E%3C/svg%3E";

export const commentAvatar = (comment: {
  avatar?: string | null;
  is_anonymous?: boolean;
}) => {
  if (comment.avatar) return comment.avatar;
  if (comment.is_anonymous) return ANONYMOUS_AVATAR;
  return "";
};
