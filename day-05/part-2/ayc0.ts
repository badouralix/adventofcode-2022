import { iterate } from "@lib/iterate.ts";

const regexBoxes = /\[(.*?)\]/dg;
const regexMove = /move (\d+) from (\d+) to (\d+)/g;

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): unknown => {
  const stacks: string[][] = [];

  for (const row of iterate(s, "\n")) {
    if (row.startsWith(" 1 ")) {
      break;
    }

    for (const match of row.matchAll(regexBoxes)) {
      // @ts-expect-error – indices not supported by Deno TS but is by the runtime
      const index = (match.indices[1][0] + 3) / 4 - 1;
      if (!stacks[index]) {
        stacks[index] = [];
      }
      stacks[index].unshift(match[1]);
    }
  }

  for (const match of s.matchAll(regexMove)) {
    const len = parseInt(match[1], 10);
    const from = parseInt(match[2], 10) - 1;
    const to = parseInt(match[3], 10) - 1;

    stacks[to].push(...stacks[from].splice(stacks[from].length - len, len));
  }

  return stacks.map((s) => s.at(-1)).join("");
};

run(`    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2`);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
