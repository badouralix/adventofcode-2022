function onlyDifferent(buffer: Uint8Array): boolean {
  for (let i = buffer.length - 1; i >= 0; i--) {
    for (let j = i - 1; j >= 0; j--) {
      if (buffer[i] === buffer[j]) {
        return false;
      }
    }
  }
  return true;
}

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): unknown => {
  const buffer = new Uint8Array(14);
  for (let i = 0; i < s.length; i++) {
    buffer[i % 14] = s.charCodeAt(i);
    if (buffer[13] === 0) {
      continue;
    }
    if (onlyDifferent(buffer)) {
      return i + 1;
    }
  }
  throw new Error("Not found");
};

run("mjqjpqmgbljsphdztnvjfqwrcgsmlb") === 19;
run("bvwbjplbgvbhsrlpgdmjqwftvncz") === 23;
run("nppdvjthqldpwncqszvftbrmjlhg") === 23;
run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") === 29;
run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") === 26;

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
