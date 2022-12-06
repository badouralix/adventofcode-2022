/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): unknown => {
  const buffer = new Uint8Array(3);
  buffer[0] = s.charCodeAt(0);
  buffer[1] = s.charCodeAt(1);
  buffer[2] = s.charCodeAt(2);

  let charCode: number;
  for (let i = 3; i < s.length; i++) {
    charCode = s.charCodeAt(i);
    if (
      charCode !== buffer[0] &&
      charCode !== buffer[1] &&
      charCode !== buffer[2] &&
      buffer[1] !== buffer[0] &&
      buffer[2] !== buffer[1] &&
      buffer[2] !== buffer[0]
    ) {
      return i + 1;
    }
    buffer[i % 3] = charCode;
  }
  throw new Error("Not found");
};

run("bvwbjplbgvbhsrlpgdmjqwftvncz") === 5;
run("nppdvjthqldpwncqszvftbrmjlhg") === 6;
run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") === 10;
run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") === 11;

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
