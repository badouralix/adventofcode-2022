import { iterate } from "@lib/iterate.ts";

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): unknown => {
  let score = 0;

  const set = new Set<string>();

  for (const step of iterate(s, "\n")) {
    if (!step) {
      // Remove ""
      continue;
    }
    for (const x of step.substring(0, step.length / 2)) {
      set.add(x);
    }

    let result: string | undefined;
    for (const x of step.substring(step.length / 2)) {
      if (set.has(x)) {
        result = x;
        break;
      }
    }
    set.clear();

    if (!result) {
      continue;
    }

    const code = result.charCodeAt(0);
    if (code >= 97) {
      // a-z are mapped to 1->26
      score += code - 96;
    } else {
      // A-Z are mapped to 27->52
      score += code - 38;
    }
  }

  return score;
};

run(`vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw`);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
