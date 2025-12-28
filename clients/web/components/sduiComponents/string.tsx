export function SDUIString({ data }: { data: import("@/app/sdui").StringData }) {
  return (
    <span className={data.className} style={data.style}>
      {data.text}
    </span>
  );
}