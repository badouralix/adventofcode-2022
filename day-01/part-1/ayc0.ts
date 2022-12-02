/**
 * @param {string} s puzzle input in string format
 * @returns {string} solution flag
 */
const run = (s: string): unknown => {
  const calories: number[] = [];
  let calory = 0;
  for (const row of s.split("\n")) {
    if (!row && calory) {
      calories.push(calory);
      calory = 0;
      continue;
    }
    calory += parseInt(row, 10);
  }
  if (calory) {
    calories.push(calory);
    calory = 0;
  }
  return Math.max(...calories);
};

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
