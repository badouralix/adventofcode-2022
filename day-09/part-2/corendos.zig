const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

pub fn Vec2D(comptime T: type) type {
    return struct {
        x: T,
        y: T,

        pub fn substract(self: @This(), other: @This()) @This() {
            return .{
                .x = self.x - other.x,
                .y = self.y - other.y,
            };
        }

        pub fn add(self: @This(), other: @This()) @This() {
            return .{
                .x = self.x + other.x,
                .y = self.y + other.y,
            };
        }

        pub fn cast(self: @This(), comptime U: type) Vec2D(U) {
            return .{
                .x = @intCast(U, self.x),
                .y = @intCast(U, self.y),
            };
        }
    };
}

pub inline fn toIndex(v: Vec2D(i3)) u6 {
    return @as(u6, @bitCast(u3, v.x)) | @as(u6, @bitCast(u3, v.y)) << 3;
}

const lookup_table: [std.math.maxInt(u6) + 1]Vec2D(i8) = blk: {
    var table = [_]Vec2D(i8){Vec2D(i8){ .x = 0, .y = 0 }} ** (std.math.maxInt(u6) + 1);

    table[toIndex(Vec2D(i3){ .x = -2, .y = -2 })] = Vec2D(i8){ .x = -1, .y = -1 };
    table[toIndex(Vec2D(i3){ .x = -1, .y = -2 })] = Vec2D(i8){ .x = -1, .y = -1 };
    table[toIndex(Vec2D(i3){ .x = 0, .y = -2 })] = Vec2D(i8){ .x = 0, .y = -1 };
    table[toIndex(Vec2D(i3){ .x = 1, .y = -2 })] = Vec2D(i8){ .x = 1, .y = -1 };
    table[toIndex(Vec2D(i3){ .x = 2, .y = -2 })] = Vec2D(i8){ .x = 1, .y = -1 };
    table[toIndex(Vec2D(i3){ .x = -2, .y = -1 })] = Vec2D(i8){ .x = -1, .y = -1 };
    table[toIndex(Vec2D(i3){ .x = -1, .y = -1 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 0, .y = -1 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 1, .y = -1 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 2, .y = -1 })] = Vec2D(i8){ .x = 1, .y = -1 };
    table[toIndex(Vec2D(i3){ .x = -2, .y = 0 })] = Vec2D(i8){ .x = -1, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = -1, .y = 0 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 0, .y = 0 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 1, .y = 0 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 2, .y = 0 })] = Vec2D(i8){ .x = 1, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = -2, .y = 1 })] = Vec2D(i8){ .x = -1, .y = 1 };
    table[toIndex(Vec2D(i3){ .x = -1, .y = 1 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 0, .y = 1 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 1, .y = 1 })] = Vec2D(i8){ .x = 0, .y = 0 };
    table[toIndex(Vec2D(i3){ .x = 2, .y = 1 })] = Vec2D(i8){ .x = 1, .y = 1 };
    table[toIndex(Vec2D(i3){ .x = -2, .y = 2 })] = Vec2D(i8){ .x = -1, .y = 1 };
    table[toIndex(Vec2D(i3){ .x = -1, .y = 2 })] = Vec2D(i8){ .x = -1, .y = 1 };
    table[toIndex(Vec2D(i3){ .x = 0, .y = 2 })] = Vec2D(i8){ .x = 0, .y = 1 };
    table[toIndex(Vec2D(i3){ .x = 1, .y = 2 })] = Vec2D(i8){ .x = 1, .y = 1 };
    table[toIndex(Vec2D(i3){ .x = 2, .y = 2 })] = Vec2D(i8){ .x = 1, .y = 1 };

    break :blk table;
};

pub const State = struct {
    knot_positions: [10]Vec2D(i64) = [_]Vec2D(i64){.{ .x = 0, .y = 0 }} ** 10,
    visited_set: std.AutoHashMap(Vec2D(i64), void),

    pub fn init(allocator: std.mem.Allocator) State {
        return State{
            .visited_set = std.AutoHashMap(Vec2D(i64), void).init(allocator),
        };
    }

    pub fn update(self: *State, movement: Vec2D(i64)) bool {
        self.knot_positions[0] = self.knot_positions[0].add(movement);

        var i: usize = 0;
        while (i < 10) : (i += 1) {
            if (i != 9) {
                const raw_delta = self.knot_positions[i].substract(self.knot_positions[i + 1]);
                const delta = raw_delta.cast(i3);
                const next_knot_movement = lookup_table[toIndex(delta)].cast(i64);
                self.knot_positions[i + 1] = self.knot_positions[i + 1].add(next_knot_movement);
            }
        }

        const result = self.visited_set.getOrPut(self.knot_positions[9]) catch unreachable;
        // Return true if it's a new position
        return !result.found_existing;
    }
};

fn run(input: [:0]const u8) u64 {
    var state = State.init(a);
    var count: u64 = 0;

    // Handle start position
    if (state.update(.{ .x = 0, .y = 0 })) {
        count += 1;
    }

    var it = std.mem.split(u8, input, "\n");
    while (it.next()) |line| {
        var distance = std.fmt.parseInt(usize, line[2..], 10) catch unreachable;
        const movement = switch (line[0]) {
            'R' => Vec2D(i64){ .x = 1, .y = 0 },
            'L' => Vec2D(i64){ .x = -1, .y = 0 },
            'U' => Vec2D(i64){ .x = 0, .y = -1 },
            'D' => Vec2D(i64){ .x = 0, .y = 1 },
            else => unreachable,
        };
        var i: usize = 0;
        while (i < distance) : (i += 1) {
            if (state.update(movement)) {
                count += 1;
            }
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

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();

    const input =
        \\R 4
        \\U 4
        \\L 3
        \\D 1
        \\R 4
        \\D 1
        \\L 5
        \\R 2
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(u64, 1), result);
}

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();

    const input =
        \\R 5
        \\U 8
        \\L 8
        \\D 3
        \\R 17
        \\D 10
        \\L 25
        \\U 20
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(u64, 36), result);
}
