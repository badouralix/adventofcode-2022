const enum Opponent {
  Rock = "A",
  Paper = "B",
  Scissors = "C",
}

const enum Strategy {
  Lose = "X",
  Draw = "Y",
  Win = "Z",
}

const enum Score {
  Rock = 1,
  Paper = 2,
  Scissors = 3,

  Lose = 0,
  Draw = 3,
  Win = 6,
}

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): number => {
  let score = 0;
  let opponent = "";

  for (const char of s) {
    if (
      char === Opponent.Rock ||
      char === Opponent.Paper ||
      char === Opponent.Scissors
    ) {
      opponent = char;
      continue;
    }

    if (char === Strategy.Win) {
      score += Score.Win;

      if (opponent === Opponent.Paper) {
        score += Score.Scissors;
      } else if (opponent === Opponent.Rock) {
        score += Score.Paper;
      } else if (opponent === Opponent.Scissors) {
        score += Score.Rock;
      }

      continue;
    }

    if (char === Strategy.Draw) {
      score += Score.Draw;

      if (opponent === Opponent.Paper) {
        score += Score.Paper;
      } else if (opponent === Opponent.Rock) {
        score += Score.Rock;
      } else if (opponent === Opponent.Scissors) {
        score += Score.Scissors;
      }

      continue;
    }

    if (char === Strategy.Lose) {
      score += Score.Lose;

      if (opponent === Opponent.Paper) {
        score += Score.Rock;
      } else if (opponent === Opponent.Rock) {
        score += Score.Scissors;
      } else if (opponent === Opponent.Scissors) {
        score += Score.Paper;
      }

      continue;
    }
  }

  // Your code goes here
  return score;
};

run(`A Y
B X
C Z
`);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
