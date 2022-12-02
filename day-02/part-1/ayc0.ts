const enum Opponent {
  Rock = "A",
  Paper = "B",
  Scissors = "C",
}

const enum Own {
  Rock = "X",
  Paper = "Y",
  Scissors = "Z",
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
 * @param {string} s puzzle input in string format
 * @returns {string} solution flag
 */
const run = (s: string): unknown => {
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

    if (char === Own.Rock) {
      score += Score.Rock;
      if (opponent === Opponent.Scissors) {
        // Rock beats scissors => 6 points
        score += Score.Win;
      } else if (opponent === Opponent.Rock) {
        // Draw => 3 points
        score += Score.Draw;
      }
      continue;
    }

    if (char === Own.Paper) {
      score += Score.Paper;
      if (opponent === Opponent.Rock) {
        // Paper beats rock => 6 points
        score += Score.Win;
      } else if (opponent === Opponent.Paper) {
        // Draw => 3 points
        score += Score.Draw;
      }
      continue;
    }

    if (char === Own.Scissors) {
      score += Score.Scissors;
      if (opponent === Opponent.Paper) {
        // Scissors beats paper => 6 points
        score += Score.Win;
      } else if (opponent === Opponent.Scissors) {
        // Draw => 3 points
        score += Score.Draw;
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
