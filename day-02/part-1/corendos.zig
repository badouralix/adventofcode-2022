const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

pub const Shape = enum(u2) {
    rock,
    paper,
    scissors,

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

pub inline fn toIndex(shape_a: Shape, shape_b: Shape) u4 {
    return @intCast(u4, @enumToInt(shape_a)) << 2 | @intCast(u4, @enumToInt(shape_b));
}

const lookup_table = blk: {
    var table = [_]u64{0} ** (std.math.maxInt(u4) + 1);
    for (std.meta.tags(Shape).*) |shape_a| {
        for (std.meta.tags(Shape).*) |shape_b| {
            const index = toIndex(shape_a, shape_b);
            table[index] = Shape.roundScore(shape_a, shape_b);
        }
    }

    break :blk table;
};

const unroll_factor = 4;

fn run(input: [:0]const u8) u64 {
    var total_score: u64 = 0;
    var cursor: usize = 0;

    const lines = (input.len + 1) / 4;
    const remaining = lines % unroll_factor;
    var i: usize = 0;
    while (i < remaining) : ({
        i += 1;
        cursor += 4;
    }) {
        const shape_a = @intToEnum(Shape, input[cursor + 0] - 'A');
        const shape_b = @intToEnum(Shape, input[cursor + 2] - 'X');
        const index = toIndex(shape_a, shape_b);
        total_score += lookup_table[index];
    }

    while (cursor < input.len) : (cursor += 4 * unroll_factor) {
        comptime var j: usize = 0;
        inline while (j < unroll_factor) : (j += 1) {
            const shape_a = @intToEnum(Shape, input[cursor + j * 4 + 0] - 'A');
            const shape_b = @intToEnum(Shape, input[cursor + j * 4 + 2] - 'X');
            const index = toIndex(shape_a, shape_b);
            total_score += lookup_table[index];
        }
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
    try std.testing.expectEqual(@as(u64, 15), score);
}
