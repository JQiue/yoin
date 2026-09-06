import { useState } from "preact/hooks";
import { useCommentStore } from "@/client/store";
import type { ReactionSummary } from "@/shared/api/types";
import { Button } from "@/shared/components/Button";
import Icon from "@/shared/components/Icon";
import { useI18n } from "@/shared/i18n";

const FALLBACK_REACTIONS = ["👍", "❤️", "😄", "🎉", "👎"];

interface Props {
  summary: ReactionSummary;
  onSelect: (reaction: string) => void | Promise<void>;
  hideEmpty?: boolean;
  spread?: boolean;
  showPickerOnHover?: boolean;
}

const ReactionButton = ({
  reaction,
  count,
  isMine,
  spread,
  loading,
  disabled,
  onSelect,
}: {
  reaction: string;
  count: number;
  isMine: boolean;
  spread: boolean;
  loading: boolean;
  disabled: boolean;
  onSelect: (reaction: string) => void;
}) => {
  const { t } = useI18n();
  return (
    <Button
      type="button"
      variant={isMine ? "secondary" : "ghost"}
      size={spread ? "md" : "sm"}
      aria-pressed={isMine}
      aria-busy={loading}
      aria-label={
        loading
          ? t("client.submittingReaction", { reaction })
          : t("client.reaction", { reaction })
      }
      disabled={disabled}
      onClick={() => onSelect(reaction)}
      className={`${spread ? "min-w-0 flex-1 py-3 text-2xl" : "px-1.5 py-0.5"} ${isMine ? "bg-(--yo-surface-soft)" : ""} ${disabled ? "cursor-wait" : ""}`}
    >
      <span>{reaction}</span>
      {loading ? (
        <span className="inline-flex animate-spin text-sm">
          <Icon name="refresh" />
        </span>
      ) : count > 0 ? (
        <span className={spread ? "text-sm" : undefined}>{count}</span>
      ) : null}
    </Button>
  );
};

export default ({
  summary,
  onSelect,
  hideEmpty = false,
  spread = false,
  showPickerOnHover = false,
}: Props) => {
  const [pendingReaction, setPendingReaction] = useState<string | null>(null);
  const reactions =
    useCommentStore((state) => state.siteConfig?.allowed_reactions) ??
    FALLBACK_REACTIONS;
  const countedReactions = reactions.filter(
    (reaction) => (summary.counts[reaction] ?? 0) > 0,
  );
  const pickerReactions = showPickerOnHover
    ? reactions.filter((reaction) => (summary.counts[reaction] ?? 0) === 0)
    : [];
  const visibleReactions = hideEmpty
    ? countedReactions
    : showPickerOnHover
      ? countedReactions
      : reactions;

  if (visibleReactions.length === 0 && pickerReactions.length === 0) {
    return null;
  }

  const handleSelect = async (reaction: string) => {
    if (pendingReaction) return;
    setPendingReaction(reaction);
    try {
      await onSelect(reaction);
    } finally {
      setPendingReaction(null);
    }
  };

  return (
    <div
      className={spread ? "flex w-full gap-2" : "flex flex-wrap gap-1"}
      aria-busy={pendingReaction != null}
    >
      {visibleReactions.map((reaction) => (
        <ReactionButton
          key={reaction}
          reaction={reaction}
          count={summary.counts[reaction] ?? 0}
          isMine={summary.my_reaction === reaction}
          spread={spread}
          loading={pendingReaction === reaction}
          disabled={pendingReaction != null}
          onSelect={handleSelect}
        />
      ))}
      {pickerReactions.length > 0 ? (
        <div className="hidden flex-wrap gap-1 group-hover:flex">
          {pickerReactions.map((reaction) => (
            <ReactionButton
              key={reaction}
              reaction={reaction}
              count={0}
              isMine={false}
              spread={false}
              loading={pendingReaction === reaction}
              disabled={pendingReaction != null}
              onSelect={handleSelect}
            />
          ))}
        </div>
      ) : null}
    </div>
  );
};
