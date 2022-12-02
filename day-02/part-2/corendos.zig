const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

pub const Result = enum {
    loss,
    draw,
    win,

    pub inline fn fromLetter(c: u8) Result {
        return switch (c) {
            'X' => .loss,
            'Y' => .draw,
            'Z' => .win,
            else => unreachable,
        };
    }

    pub inline fn requiredShapeForResult(result: Result, opponent_shape: Shape) Shape {
        return switch (result) {
            .loss => switch (opponent_shape) {
                .rock => .scissors,
                .paper => .rock,
                .scissors => .paper,
            },
            .draw => switch (opponent_shape) {
                .rock => .rock,
                .paper => .paper,
                .scissors => .scissors,
            },
            .win => switch (opponent_shape) {
                .rock => .paper,
                .paper => .scissors,
                .scissors => .rock,
            },
        };
    }
};

pub const Shape = enum(u4) {
    rock,
    paper,
    scissors,

    pub inline fn fromLetter(c: u8) Shape {
        return switch (c) {
            'A' => .rock,
            'B' => .paper,
            'C' => .scissors,
            else => unreachable,
        };
    }

    pub inline fn score(shape: Shape) u64 {
        return @intCast(i64, @enumToInt(shape));
    }

    pub inline fn roundScore(shape_a: Shape, shape_b: Shape) u64 {
        return switch (shape_a) {
            .rock => switch (shape_b) {
                .rock => 3 + 1,
                .paper => 6 + 2,
                .scissors => 0 + 3,
            },
            .paper => switch (shape_b) {
                .rock => 0 + 1,
                .paper => 3 + 2,
                .scissors => 6 + 3,
            },
            .scissors => switch (shape_b) {
                .rock => 6 + 1,
                .paper => 0 + 2,
                .scissors => 3 + 3,
            },
        };
    }
};

fn run(input: [:0]const u8) u64 {
    var total_score: u64 = 0;
    var iterator = std.mem.split(u8, input, "\n");
    while (iterator.next()) |line| {
        const opponent_shape = Shape.fromLetter(line[0]);
        const result = Result.fromLetter(line[2]);
        const required_shape = Result.requiredShapeForResult(result, opponent_shape);
        total_score += Shape.roundScore(opponent_shape, required_shape);
    }
    return total_score;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();

    var arg_it = try std.process.argsWithAllocator(a);
    _ = arg_it.skip(); // skip over exe name
    const input: [:0]const u8 = arg_it.next().?;

    const start: i128 = std.time.nanoTimestamp(); // start time
    const answer = run(input); // compute answer
    const end: i128 = std.time.nanoTimestamp();
    const elapsed_nano = @intToFloat(f64, end - start);
    const elapsed_milli = elapsed_nano / 1_000_000.0;
    try stdout.print("_duration:{d}\n{}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}

test {
    const input =
        \\A Y
        \\B X
        \\C Z
    ;
    const score = run(input);
    try std.testing.expectEqual(@as(u64, 12), score);
}
