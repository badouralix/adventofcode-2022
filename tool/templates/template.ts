/**
 * @param {string} s puzzle input in string format
 * @returns {string} solution flag
 */
const run = (s: string): unknown => {
  // Your code goes here
  return s;
};

const start = Date.now();
const answer = run(Deno.args[0]);

console.log("_duration:" + (Date.now() - start).toString());
console.log(answer);
