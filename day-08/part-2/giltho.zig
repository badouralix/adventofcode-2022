const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn get_grid_size() usize {
    if (builtin.is_test) {
        return 5;
    } else {
        return 99;
    }
}
const grid_size = get_grid_size();

fn run(input: [:0]const u8) usize {
    var i: usize = 0;
    var view_scores: [(grid_size + 1) * grid_size]usize = .{1} ** ((grid_size + 1) * grid_size);
    var k: usize = 0;
    while (i < grid_size) : (i += 1) {
        var last_seen_from_left: [10]usize = .{0} ** 10;
        var last_seen_from_right: [10]usize = .{0} ** 10;
        var last_seen_from_top: [10]usize = .{0} ** 10;
        var last_seen_from_bottom: [10]usize = .{0} ** 10;
        var j: usize = 0;
        while (j < grid_size) : (j += 1) {
            var left_idx = i * (grid_size + 1) + j;
            var from_left = input[left_idx] - '0';
            view_scores[left_idx] *= j - last_seen_from_left[from_left];
            k = from_left;
            while (true) {
                last_seen_from_left[k] = j;
                if (k == 0) {
                    break;
                }
                k -= 1;
            }

            var right_idx = i * (grid_size + 1) + grid_size - j - 1;
            var from_right = input[right_idx] - '0';
            view_scores[right_idx] *= j - last_seen_from_right[from_right];
            k = from_right;
            while (true) {
                last_seen_from_right[k] = j;
                if (k == 0) {
                    break;
                }
                k -= 1;
            }

            var top_idx = j * (grid_size + 1) + i;
            var from_top = input[top_idx] - '0';
            view_scores[top_idx] *= j - last_seen_from_top[from_top];
            k = from_top;
            while (true) {
                last_seen_from_top[k] = j;
                if (k == 0) {
                    break;
                }
                k -= 1;
            }

            var bottom_idx = (grid_size - j - 1) * (grid_size + 1) + i;
            var from_bottom = input[bottom_idx] - '0';
            view_scores[bottom_idx] *= j - last_seen_from_bottom[from_bottom];
            k = from_bottom;
            while (true) {
                last_seen_from_bottom[k] = j;
                if (k == 0) {
                    break;
                }
                k -= 1;
            }
        }
    }
    var answer: usize = 1;
    i = 0;
    while (i < (grid_size + 1) * grid_size) : (i += 1) {
        if (view_scores[i] > answer) {
            answer = view_scores[i];
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
    try std.testing.expectEqual(@as(usize, 8), answer);
}
