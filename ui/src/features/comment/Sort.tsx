import { useCommentStore } from "@/store";

export default () => {
	const { sort, changeSort, isLoading } = useCommentStore();

	const tabs = [
		{ label: "最新", value: "created_desc" },
		{ label: "最旧", value: "created_asc" },
	];

	return (
		<nav className="inline-flex p-1">
			{tabs.map((tab) => {
				const isActive = sort === tab.value;
				return (
					<button
						type="button"
						key={tab.value}
						disabled={isLoading}
						onClick={() => !isActive && changeSort(tab.value)}
						className={`
              relative px-5 py-1.5 text-xs font-bold transition-all rounded-md
              ${isActive ? "bg-zinc-100 " : "hover:text-zinc-500 hover:bg-zinc-100"}
              ${isLoading ? "opacity-30 cursor-not-allowed" : "cursor-pointer"}
            `}
					>
						{tab.label}
					</button>
				);
			})}
		</nav>
	);
};
