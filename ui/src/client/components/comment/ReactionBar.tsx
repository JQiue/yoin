import type { ReactionSummary } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";

const REACTIONS = ["👍", "❤️", "😄", "🎉", "👎"];

interface Props {
  summary: ReactionSummary;
  onSelect: (reaction: string) => void;
}

export default ({ summary, onSelect }: Props) => {
  return (
    <div className="flex flex-wrap gap-1">
      {REACTIONS.map((reaction) => {
        const count = summary.counts[reaction] ?? 0;
        const isMine = summary.my_reaction === reaction;
        return (
          <Button
            key={reaction}
            type="button"
            variant={isMine ? "secondary" : "ghost"}
            size="sm"
            aria-pressed={isMine}
            onClick={() => onSelect(reaction)}
            className={`px-1.5 py-0.5 ${isMine ? "bg-zinc-100" : ""}`}
          >
            <span>{reaction}</span>
            {count > 0 ? <span>{count}</span> : null}
          </Button>
        );
      })}
    </div>
  );
};
