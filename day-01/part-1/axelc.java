import java.util.List;
import java.util.stream.IntStream;
import java.util.stream.Stream;

class Solution {
    // BEWARE: your main class MUST be named Solution

    private static String solve(String input) {
        var lines = List.of(input.split("\n\n"));
        var elves = lines.stream().map(line -> Stream.of(line.split("\n")).flatMapToInt(s -> IntStream.of(Integer.parseInt(s))).sum()).toList();
        var biggestElf = elves.stream().max(Integer::compare).get();
        return biggestElf.toString();
    };

    public static void main(String[] args) {
        String input = args[0];
        long startTime = System.currentTimeMillis();
        String result = solve(input);
        System.out.println("_duration: " + (System.currentTimeMillis() - startTime) + "\n" + result);
    }
}

