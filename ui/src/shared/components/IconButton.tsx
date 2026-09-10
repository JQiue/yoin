import type { LucideIcon } from "lucide-preact";
import type { ButtonHTMLAttributes } from "preact";

interface Props extends ButtonHTMLAttributes {
  icon: LucideIcon;
  label: string;
  pressed?: boolean;
  iconSize?: number;
}

export const IconButton = ({
  icon: Icon,
  label,
  pressed,
  iconSize = 16,
  className = "",
  ...props
}: Props) => {
  return (
    <button
      type="button"
      aria-label={label}
      title={label}
      aria-pressed={pressed}
      className={`inline-flex h-7 w-7 shrink-0 items-center justify-center rounded-md text-(--yo-text-muted) transition-colors hover:text-(--yo-text) disabled:cursor-wait disabled:opacity-60 ${className}`}
      {...props}
    >
      <Icon
        size={iconSize}
        strokeWidth={2}
        fill={pressed ? "currentColor" : "none"}
      />
    </button>
  );
};
