const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) u64 {
    var elves_it = std.mem.split(u8, input, "\n\n");
    var max1: u64 = 0;
    var max2: u64 = 0;
    var max3: u64 = 0;
    var cur: u64 = 0;
    while (elves_it.next()) |elf| {
        cur = 0;
        var cal_it = std.mem.tokenize(u8, elf, "\n");
        while (cal_it.next()) |cal_str| {
            cur += std.fmt.parseInt(u32, cal_str, 10) catch unreachable;
        }
        
        if (cur > max1){
            max3 = max2;
            max2 = max1;
            max1 = cur;
        }
        else if (cur > max2){
            max3 = max2;
            max2 = cur;
        }
        else if (cur > max3) {
            max3 = cur;
        }
    }
    return max1 + max2 + max3;
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
