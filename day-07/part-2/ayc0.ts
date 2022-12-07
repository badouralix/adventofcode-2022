import { iterate } from "@lib/iterate.ts";

type FileSystem = { [folderName: string]: FileSystem | number };

const TOTAL_SPACE = 70_000_000;
const TO_BE_AVAILABLE = 30_000_000;

function buildFileSystem(input: string): FileSystem {
  const fileSystem: FileSystem = {};
  let activeFolder = fileSystem;
  const pathToFolder: string[] = [];
  for (const command of iterate(input, "$ ")) {
    if (!command) {
      continue;
    }
    if (command.startsWith("cd /")) {
      continue;
    }
    if (command.startsWith("cd")) {
      const folderName = command.trim().substring(3);
      if (folderName === "..") {
        pathToFolder.pop();
        activeFolder = fileSystem;
        for (const p of pathToFolder) {
          activeFolder = activeFolder[p] as FileSystem;
        }
      } else {
        pathToFolder.push(folderName);
        activeFolder = activeFolder[folderName] as FileSystem;
      }
      continue;
    }

    // ls
    for (const line of iterate(command, "\n")) {
      if (line === "ls" || !line) {
        continue;
      }
      if (line.startsWith("dir")) {
        activeFolder[line.substring(4)] = {};
      } else {
        const [sizeString, fileName] = line.split(" ");
        activeFolder[fileName] = parseInt(sizeString, 10);
      }
    }
  }
  return fileSystem;
}

function computeSize(fs: FileSystem, notify: (size: number) => void): number {
  let total = 0;
  for (const value of Object.values(fs)) {
    if (typeof value === "number") {
      total += value;
    } else {
      total += computeSize(value, notify);
    }
  }
  notify(total);
  return total;
}

/**
 * @param s puzzle input in string format
 * @returns solution flag
 */
const run = (s: string): unknown => {
  const fileSystem = buildFileSystem(s);

  const sizes: number[] = [];
  const usedSize = computeSize(fileSystem, (size) => {
    sizes.push(size);
  });
  const toDelete = TO_BE_AVAILABLE - (TOTAL_SPACE - usedSize);
  sizes.sort((a, b) => a - b);
  return sizes.find((size) => size >= toDelete);
};

run(`$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k`);

const start = performance.now();
const answer = run(Deno.args[0]);

console.log(`_duration:${performance.now() - start}`);
console.log(answer);
