export type ForestxConfigValue = string | number | boolean | ForestxConfigValue[] | ForestxConfigObject;

export type ForestxConfigObject = { [key: string]: ForestxConfigValue };

export type ForestxOptions = {
  forestxPathOverride?: string;
  baseUrl?: string;
  apiKey?: string;
  /**
   * Additional `--config key=value` overrides to pass to the Forestx CLI.
   *
   * Provide a JSON object and the SDK will flatten it into dotted paths and
   * serialize values as TOML literals so they are compatible with the CLI's
   * `--config` parsing.
   */
  config?: ForestxConfigObject;
  /**
   * Environment variables passed to the Forestx CLI process. When provided, the SDK
   * will not inherit variables from `process.env`.
   */
  env?: Record<string, string>;
};
