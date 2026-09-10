const ICON_MAP = {
  alert: "⚠",
  refresh: "↻",
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
