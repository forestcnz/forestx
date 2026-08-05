import path from "node:path";

export function forestxPathOverride() {
  return (
    process.env.FORESTX_EXECUTABLE ??
    path.join(process.cwd(), "..", "..", "forestx-rs", "target", "debug", "forestx")
  );
}
