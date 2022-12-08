const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn make_config() struct { grid_size: usize, map_type: type, log2t: type } {
    if (builtin.is_test) {
        return .{ .grid_size = 5, .map_type = u8, .log2t = u3 };
    } else {
        return .{ .grid_size = 99, .map_type = u128, .log2t = u7 };
    }
}
const config = make_config();

fn run(input: [:0]const u8) i64 {
    var i: config.log2t = 0;
    var maps : [config.grid_size]config.map_type  = .{0} ** config.grid_size;
    while (i < config.grid_size) : (i += 1) {
        var max_from_left: u8 = '0' - 1;
        var max_from_right: u8 = '0' - 1;
        var max_from_top: u8 = '0' - 1;
        var max_from_bottom: u8 = '0' - 1;
        var j: config.log2t = 0;
        while (j < config.grid_size) : (j += 1) {
            var from_left = input[i * (config.grid_size + 1) + j];
            var from_right = input[i * (config.grid_size + 1) + config.grid_size - j - 1];
            var from_top = input[j * (config.grid_size + 1) + i];
            var from_bottom = input[(config.grid_size - j - 1) * (config.grid_size + 1) + i];
            if (from_left > max_from_left) {
                max_from_left = from_left;
                maps[i] |= (@as(config.map_type, 1) << (config.grid_size - 1)) >> j;
            }
            if (from_right > max_from_right) {
                max_from_right = from_right;
                maps[i] |= @as(config.map_type, 1) << j;
            }
            if (from_top > max_from_top) {
                max_from_top = from_top;
                maps[j] |= (@as(config.map_type, 1) << (config.grid_size - 1)) >> i;   
            }
            if (from_bottom > max_from_bottom) {
                max_from_bottom = from_bottom;
                maps[config.grid_size - j - 1] |=  (@as(config.map_type, 1) << (config.grid_size - 1)) >> i;
            }
        }
    }
    var answer: i64 = 0;
    i = 0;
    while (i < config.grid_size) : (i += 1) {
        var j: config.log2t = 0;
        while (j < config.grid_size) : (j += 1) {
            if (maps[i] & 1 == 1) {
                answer += 1;
            }
            maps[i] >>= 1;
        }
    }
    return answer;
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

test "example" {
    const input =
        \\30373
        \\25512
        \\65332
        \\33549
        \\35390
    ;
    const answer = run(input);
    try std.testing.expectEqual(@as(i64, 21), answer);
}