import { iterate } from "@lib/iterate.ts";

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): unknown => {
  let score = 0;

  const setA = new Set<string>();
  const setB = new Set<string>();

  for (const step of iterate(s, "\n")) {
    if (!step) {
      // Remove ""
      continue;
    }
    if (setA.size === 0) {
      for (const x of step) {
        setA.add(x);
      }
      continue;
    }
    if (setB.size === 0) {
      for (const x of step) {
        setB.add(x);
      }
      continue;
    }

    let result: string | undefined;
    for (const x of step) {
      if (setA.has(x) && setB.has(x)) {
        result = x;
        break;
      }
    }
    setA.clear();
    setB.clear();

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
