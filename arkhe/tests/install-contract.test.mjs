import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

test('Ganache Darwin-only watcher remains optional for clean Windows/Linux npm ci', () => {
  const lock = JSON.parse(readFileSync(new URL('../package-lock.json', import.meta.url), 'utf8'));
  const watcher = lock.packages['node_modules/ganache/node_modules/chokidar'];
  const fsevents = lock.packages['node_modules/ganache/node_modules/fsevents'];
  assert.equal(watcher.optionalDependencies.fsevents, '~2.3.2');
  assert.deepEqual(fsevents.os, ['darwin']);
  assert.equal(fsevents.optional, true, 'npm 10 rejects required Darwin-only fsevents on Windows');
  assert.equal(fsevents.version, '2.3.2');
  assert.ok(fsevents.integrity.startsWith('sha512-'));
});
