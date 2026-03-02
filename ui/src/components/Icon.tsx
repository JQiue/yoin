const ICON_MAP = {
	thumbsUp: "👍",
	thumbsDown: "👎",
	clock: "🕒",
	alert: "⚠",
	refresh: "↻",
	send: "➤",
	link: "🔗",
	close: "✕",
	replyTo: "➥",
	reply: "💬",
} as const;

type IconName = keyof typeof ICON_MAP;

interface Props {
	name: IconName;
}

export default (props: Props) => {
	return (
		<span role="img" aria-label={props.name}>
			{ICON_MAP[props.name] || ""}
		</span>
	);
};
