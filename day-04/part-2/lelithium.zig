const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) u64 {
    var score: u64 = 0;

    var iterator = std.mem.tokenize(u8, input, "\n-,");
    while (iterator.next()) |e1_lo_raw| {
        const e1_lo = std.fmt.parseInt(u16, e1_lo_raw, 10) catch unreachable;
        const e1_hi = std.fmt.parseInt(u16, iterator.next().?, 10) catch unreachable;
        const e2_lo = std.fmt.parseInt(u16, iterator.next().?, 10) catch unreachable;
        const e2_hi = std.fmt.parseInt(u16, iterator.next().?, 10) catch unreachable;
        if (!(e1_hi < e2_lo or e2_hi < e1_lo))
            score += 1;
        //stdout.print("{}-{},{}-{}\n", .{ e1_lo, e1_hi, e2_lo, e2_hi }) catch unreachable;
    }
    return score;
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

test "ez" {
    const input =
        \\2-4,6-8
        \\2-3,4-5
        \\5-7,7-9
        \\2-8,3-7
        \\6-6,4-6
        \\2-6,4-8
    ;
    var buf = input.*;
    try stdout.print("\n", .{});
    const ans = run(&buf);
    try std.testing.expect(ans == 4);
}
