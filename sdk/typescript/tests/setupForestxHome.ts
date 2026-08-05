import fs from "node:fs/promises";
import os from "node:os";
import path from "node:path";

import { afterEach, beforeEach } from "@jest/globals";

const originalForestxHome = process.env.FORESTX_HOME;
let currentForestxHome: string | undefined;

beforeEach(async () => {
  currentForestxHome = await fs.mkdtemp(path.join(os.tmpdir(), "forestx-sdk-test-"));
  process.env.FORESTX_HOME = currentForestxHome;
});

afterEach(async () => {
  const forestxHomeToDelete = currentForestxHome;
  currentForestxHome = undefined;

  if (originalForestxHome === undefined) {
    delete process.env.FORESTX_HOME;
  } else {
    process.env.FORESTX_HOME = originalForestxHome;
  }

  if (forestxHomeToDelete) {
    await fs.rm(forestxHomeToDelete, { recursive: true, force: true });
  }
});
