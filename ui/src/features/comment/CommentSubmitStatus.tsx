import Icon from "@/src/components/Icon";

interface Props {
	type: string;
	msg: string;
}

export default ({ type, msg }: Props) => {
	if (!msg) return null;

	return (
		<div className="flex items-center gap-2">
			<span
				className={`flex items-center gap-1.5 rounded-md px-2 py-1 text-xs font-bold ${type === "success" ? "bg-zinc-100 text-zinc-600" : "bg-red-50 text-red-600"
					}`}
			>
				{type === "error" && <Icon name="alert" />}
				{msg}
			</span>
		</div>
	);
};
