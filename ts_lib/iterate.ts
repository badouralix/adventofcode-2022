export function* iterate(input: string, char: string) {
  const charLength = char.length;
  let startIndex = 0;
  for (let i = 0; i < input.length - charLength + 1; i++) {
    if (input.substring(i, i + charLength) === char) {
      yield input.substring(startIndex, i);
      startIndex = i + charLength;
      i += charLength;
    }
  }
  yield input.substring(startIndex);
}
