import { readFileSync, writeFileSync, lstatSync } from "node:fs";
import {
  canonical,
  parseCanonical,
  makePackage,
  replay,
  testTrust,
  LIMITS,
} from "./core.mjs";
import { scenario } from "./scenarios.mjs";
import { verifyCheckpoint } from "./checkpoint.mjs";
function read(path) {
  const stat = lstatSync(path);
  if (!stat.isFile() || stat.isSymbolicLink() || stat.size > LIMITS.file)
    throw new Error("RESOURCE_LIMIT_OR_FILE");
  return parseCanonical(readFileSync(path));
}
try {
  const [command, ...args] = process.argv.slice(2);
  if (command === "generate" && args.length === 2)
    writeFileSync(args[0], canonical(makePackage(scenario(Number(args[1])))));
  else if (command === "trust-fixture" && args.length === 1)
    writeFileSync(args[0], canonical(testTrust()));
  else if (command === "verify" && args.length === 2) {
    const pkg = read(args[0]);
    const result = replay(pkg, read(args[1]));
    if (pkg.anchor?.body.checkpoint) result.checkpoint = verifyCheckpoint(pkg);
    process.stdout.write(canonical(result) + "\n");
  } else
    throw new Error(
      "Usage: generate PACKAGE COUNT | trust-fixture TRUST | verify PACKAGE TRUST",
    );
} catch (error) {
  process.stderr.write(error.message + "\n");
  process.exitCode = 1;
}
