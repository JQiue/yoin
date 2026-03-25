import { useCallback, useState } from "preact/hooks";
import { storage } from "@/shared/helper";

export interface StoredUser {
	nickname: string;
	website: string;
	email: string;
	avatar?: string;
}

export const useStoredUser = () => {
	const [currentUser, setCurrentUser] = useState<StoredUser | null>(null);

	const syncUserFromStorage = useCallback(() => {
		const token = storage.get("yoin:token");
		const savedUser = storage.get("yoin:user_info");
		if (token && savedUser) {
			setCurrentUser(savedUser);
			return savedUser;
		}
		setCurrentUser(null);
		return null;
	}, []);

	const clearStoredUser = useCallback(() => {
		storage.remove("yoin:token");
		storage.remove("yoin:user_info");
		setCurrentUser(null);
	}, []);

	return {
		currentUser,
		syncUserFromStorage,
		clearStoredUser,
	};
};
