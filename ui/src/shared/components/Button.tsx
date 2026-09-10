import type { ButtonHTMLAttributes } from "preact";

interface ButtonProps extends ButtonHTMLAttributes {
  variant?: "primary" | "secondary" | "ghost";
  size?: "sm" | "md";
  loading?: boolean;
  fullWidth?: boolean;
}

const baseClass =
  "rounded-md inline-flex items-center justify-center gap-2 transition-(--yo-transition) active:scale-95 disabled:cursor-not-allowed";

const variantClassMap = {
  primary:
    "text-(--yo-primary-contrast) bg-(--yo-primary) hover:bg-(--yo-primary-hover) disabled:bg-(--yo-surface-strong) disabled:text-(--yo-text-muted)",
  secondary:
    "bg-(--yo-surface-soft) hover:bg-(--yo-surface-strong) disabled:bg-(--yo-surface-soft) disabled:text-(--yo-text-soft)",
  ghost: "text-(--yo-text-muted) bg-transparent hover:text-(--yo-text)",
};

const sizeClassMap = {
  sm: "px-2.5 py-1.5 text-xs",
  md: "px-3 py-2 text-sm",
};

export const Button = ({
  variant = "secondary",
  size = "md",
  loading = false,
  fullWidth,
  className = "",
  disabled,
  children,
  ...props
}: ButtonProps) => {
  const widthClass = fullWidth ? "w-full" : "";

  return (
    <button
      {...props}
      disabled={disabled || loading}
      className={`${baseClass} ${widthClass} ${variantClassMap[variant]} ${sizeClassMap[size]} ${className}`}
    >
      {children}
    </button>
  );
};
