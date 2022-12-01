const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) u64 {
    var current_elf: u64 = 0;
    var first_elf: u64 = 0;
    var second_elf: u64 = 0;
    var third_elf: u64 = 0;
    var lines = std.mem.split(u8, input, "\n");
    while (lines.next()) |line| {
        if (line.len == 0) {
            if (current_elf > first_elf) {
                third_elf = second_elf;
                second_elf = first_elf;
                first_elf = current_elf;
            } else if (current_elf > second_elf) {
                third_elf = second_elf;
                second_elf = current_elf;
            } else if (current_elf > third_elf) {
                third_elf = current_elf;
            }
            current_elf = 0;
            continue;
        }
        current_elf += std.fmt.parseInt(u64, line, 10) catch unreachable;
    }
    if (current_elf > first_elf) {
        third_elf = second_elf;
        second_elf = first_elf;
        first_elf = current_elf;
    } else if (current_elf > second_elf) {
        third_elf = second_elf;
        second_elf = current_elf;
    } else if (current_elf > third_elf) {
        third_elf = current_elf;
    }
    return first_elf + second_elf + third_elf;
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
    const elapsed_nano: f128 = @intToFloat(f128, std.time.nanoTimestamp() - start);
    const elapsed_milli: f64 = @floatCast(f64, @divFloor(elapsed_nano, 1_000_000));
    try stdout.print("_duration:{d}\n{}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}
