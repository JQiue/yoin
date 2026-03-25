interface UserInfo {
  nickname: string;
  website: string;
  email: string;
  avatar?: string;
}

interface StorageSchema {
  "yoin:token": string;
  "yoin:user_info": UserInfo;
  "yoin:comment_draft": string;
}

export const storage = {
  set<K extends keyof StorageSchema>(key: K, value: StorageSchema[K]): void {
    localStorage.setItem(key, JSON.stringify(value));
  },

  get<K extends keyof StorageSchema>(key: K): StorageSchema[K] | null {
    const data = localStorage.getItem(key);
    return data ? JSON.parse(data) : null;
  },

  remove<K extends keyof StorageSchema>(key: K): void {
    localStorage.removeItem(key);
  },

  clear(): void {
    localStorage.clear();
  },
};
