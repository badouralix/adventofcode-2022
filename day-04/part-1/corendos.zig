const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

pub const Range = struct {
    start: u64,
    end: u64,

    pub inline fn contains(self: Range, other: Range) bool {
        return other.start >= self.start and other.end <= self.end;
    }
};

fn run(input: [:0]const u8) i64 {
    var count: i64 = 0;
    var it = std.mem.tokenize(u8, input, "\n,-");
    while (it.peek()) |_| {
        var range1 = Range{
            .start = std.fmt.parseInt(u64, it.next() orelse unreachable, 10) catch unreachable,
            .end = std.fmt.parseInt(u64, it.next() orelse unreachable, 10) catch unreachable,
        };
        var range2 = Range{
            .start = std.fmt.parseInt(u64, it.next() orelse unreachable, 10) catch unreachable,
            .end = std.fmt.parseInt(u64, it.next() orelse unreachable, 10) catch unreachable,
        };

        if (range1.contains(range2) or range2.contains(range1)) {
            count += 1;
        }
    }
    return count;
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

test "r a n g e" {
    const input =
        \\2-4,6-8
        \\2-3,4-5
        \\5-7,7-9
        \\2-8,3-7
        \\6-6,4-6
        \\2-6,4-8
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 2), result);
}
