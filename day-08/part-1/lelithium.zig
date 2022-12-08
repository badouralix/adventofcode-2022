const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const LEN = if (builtin.is_test) 5 else 99; // input is always a square

inline fn get_char(input: *const [:0]const u8, x: usize, y: usize) u8 {
    return input.*[y + x * (LEN + 1)];
}

//  o--> y
//  |
//  v
//  x

fn run(input: [:0]const u8) u64 {
    var all_visible: u64 = LEN * 4 - 4; // edges are always visible

    var x: usize = 1; // don't double count edges
    var y: usize = 1;

    while (x < LEN - 1) : (x += 1) {
        y = 1;
        //stdout.print("{c}", .{get_char(&input, x, 0)}) catch unreachable;
        while (y < LEN - 1) : (y += 1) {
            const char = get_char(&input, x, y);
            //stdout.print("{c}", .{char}) catch unreachable;
            // from y=0
            var distance: usize = 1;
            var visible = char > get_char(&input, x, 0);
            while (y - distance > 0 and visible) : (distance += 1)
                visible = visible and (char > get_char(&input, x, y - distance));
            if (visible) {
                //stdout.print("visible left", .{}) catch unreachable;
                all_visible += 1;
                continue;
            }
            // from y=LEN
            distance = 1;
            visible = char > get_char(&input, x, LEN - 1);
            while (y + distance < LEN - 1 and visible) : (distance += 1)
                visible = visible and (char > get_char(&input, x, y + distance));
            if (visible) {
                //stdout.print("visible right", .{}) catch unreachable;
                all_visible += 1;
                continue;
            }
            // from x=0
            distance = 1;
            visible = char > get_char(&input, 0, y);
            while (x - distance > 0 and visible) : (distance += 1)
                visible = visible and (char > get_char(&input, x - distance, y));
            if (visible) {
                //stdout.print("visible top", .{}) catch unreachable;
                all_visible += 1;
                continue;
            }
            // from x=LEN
            distance = 1;
            visible = char > get_char(&input, LEN - 1, y);
            while (x + distance < LEN - 1 and visible) : (distance += 1)
                visible = visible and (char > get_char(&input, x + distance, y));
            if (visible) {
                //stdout.print("visible bot", .{}) catch unreachable;
                all_visible += 1;
            }
        }
        //stdout.print("{c}", .{get_char(&input, x, LEN - 1)}) catch unreachable;
        //stdout.print("\n", .{}) catch unreachable;
    }
    //stdout.print("\n{} visible\n", .{all_visible}) catch unreachable;
    return all_visible;
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
    try std.testing.expect(run(
        \\30373
        \\25512
        \\65332
        \\33549
        \\35390
    ) == 21);
}
