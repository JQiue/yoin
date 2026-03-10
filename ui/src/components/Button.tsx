import type { ButtonHTMLAttributes } from "preact";

interface ButtonProps extends ButtonHTMLAttributes {
  variant?: "primary" | "secondary" | "ghost";
  size?: "sm" | "md";
  loading?: boolean;
  fullWidth?: boolean
}

const baseClass =
  "rounded-md inline-flex items-center justify-center gap-2 transition-(--yo-transition) active:scale-95 disabled:cursor-not-allowed";

const variantClassMap = {
  primary: "text-[var(--yo-primary-contrast)] bg-[var(--yo-primary)] hover:bg-[var(--yo-primary-hover)] disabled:bg-zinc-300 disabled:text-zinc-500",
  secondary: "bg-[var(--yo-surface-soft)] bg-[var(--yo-surface-soft)] hover:bg-[var(--yo-surface-strong)] disabled:bg-zinc-100 disabled:text-zinc-400",
  ghost: "text-[var(--yo-text-muted)] bg-transparent hover:text-zinc-900",
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
