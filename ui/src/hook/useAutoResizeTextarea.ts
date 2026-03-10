import { useEffect, useRef } from "preact/compat";

export const useAutoResizeTextarea = (value: string) => {
	const textareaRef = useRef<HTMLTextAreaElement | null>(null);

	useEffect(() => {
		const textarea = textareaRef.current;
		if (!textarea) return;
		textarea.style.height = "0px";
		textarea.style.height = `${textarea.scrollHeight}px`;
	}, [value]);

	return textareaRef;
};
