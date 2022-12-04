const regex = /(\d+)-(\d+),(\d+)-(\d+)/g;

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): unknown => {
  let score = 0;
  let startA: number;
  let endA: number;
  let startB: number;
  let endB: number;

  let start: boolean;
  let end: boolean;

  for (const match of s.matchAll(regex)) {
    start = false;
    end = false;

    startA = parseInt(match[1], 10);
    endA = parseInt(match[2], 10);
    startB = parseInt(match[3], 10);
    endB = parseInt(match[4], 10);

    if (startB <= endA && endA <= endB) {
      end = true;
    } else if (startA <= endB && endB <= endA) {
      end = true;
    }

    if (startB <= startA && startA <= endB) {
      start = true;
    } else if (startA <= startB && startB <= endA) {
      start = true;
    }

    if (start !== false && end !== false) {
      score++;
    }
  }
  return score;
};

run(`2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8`);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
