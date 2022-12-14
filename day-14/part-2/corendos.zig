const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn Vec2D(comptime T: type) type {
    return struct {
        x: T,
        y: T,
    };
}

pub fn Grid(comptime T: type) type {
    return struct {
        const Self = @This();
        data: []T,
        dim: Vec2D(usize),

        pub fn init(dim: Vec2D(usize), value: T, allocator: std.mem.Allocator) Self {
            var data = allocator.alloc(T, dim.x * dim.y) catch unreachable;
            std.mem.set(T, data, value);
            return Self{ .data = data, .dim = dim };
        }

        pub fn getAt(self: *const Self, x: usize, y: usize) T {
            return self.data[y * self.dim.x + x];
        }

        pub fn getAtPtr(self: *const Self, x: usize, y: usize) *T {
            return &self.data[y * self.dim.x + x];
        }

        pub fn format(value: Self, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
            _ = options;
            _ = fmt;
            var y: usize = 0;
            while (y < value.dim.y) : (y += 1) {
                var x: usize = 0;
                while (x < value.dim.x) : (x += 1) {
                    const space = if (x == 0) "" else " ";
                    try writer.print("{s}{any}", .{ space, value.data[y * value.dim.x + x] });
                }
                try writer.writeByte('\n');
            }
        }
    };
}

pub fn parseVec2D(comptime T: type, input: []const u8) Vec2D(T) {
    var it = std.mem.split(u8, input, ",");
    return Vec2D(T){
        .x = std.fmt.parseUnsigned(T, it.next().?, 10) catch unreachable,
        .y = std.fmt.parseUnsigned(T, it.next().?, 10) catch unreachable,
    };
}

pub const CellType = enum {
    empty,
    wall,
    sand,
};

pub const State = struct {
    grid: Grid(CellType),
    max_y: usize,
};

pub fn initGridFromInput(input: []const u8) State {
    var grid = Grid(CellType).init(Vec2D(usize){ .x = 1000, .y = 600 }, .empty, a);
    var global_y_max: usize = 0;

    var line_it = std.mem.split(u8, input, "\n");
    while (line_it.next()) |line| {
        var coordinates_it = std.mem.split(u8, line, " -> ");
        var last_coordinates: Vec2D(usize) = parseVec2D(usize, coordinates_it.next().?);
        while (coordinates_it.next()) |coordinates_str| {
            const coordinates = parseVec2D(usize, coordinates_str);
            if (last_coordinates.x == coordinates.x) {
                // Vertical line
                const y_min = @min(last_coordinates.y, coordinates.y);
                const y_max = @max(last_coordinates.y, coordinates.y);
                var y: usize = y_min;
                while (y <= y_max) : (y += 1) {
                    grid.getAtPtr(last_coordinates.x, y).* = .wall;
                }
                global_y_max = @max(global_y_max, y_max);
            } else if (last_coordinates.y == coordinates.y) {
                // Horizontal line
                const x_min = @min(last_coordinates.x, coordinates.x);
                const x_max = @max(last_coordinates.x, coordinates.x);
                var x: usize = x_min;
                while (x <= x_max) : (x += 1) {
                    grid.getAtPtr(x, last_coordinates.y).* = .wall;
                }
                global_y_max = @max(global_y_max, coordinates.y);
            } else unreachable;
            last_coordinates = coordinates;
        }
    }

    var x: usize = 0;
    while (x < grid.dim.x) : (x += 1) {
        grid.getAtPtr(x, global_y_max + 2).* = .wall;
    }

    return State{ .grid = grid, .max_y = global_y_max };
}

fn run(input: [:0]const u8) usize {
    var state = initGridFromInput(input);

    var sand_unit_count: usize = 0;

    top: while (true) : (sand_unit_count += 1) {
        var new_sand_unit_position = Vec2D(usize){ .x = 500, .y = 0 };
        while (true) {
            if (state.grid.getAt(new_sand_unit_position.x, new_sand_unit_position.y + 1) == .empty) {
                new_sand_unit_position.y += 1;
            } else if (state.grid.getAt(new_sand_unit_position.x - 1, new_sand_unit_position.y + 1) == .empty) {
                new_sand_unit_position.y += 1;
                new_sand_unit_position.x -= 1;
            } else if (state.grid.getAt(new_sand_unit_position.x + 1, new_sand_unit_position.y + 1) == .empty) {
                new_sand_unit_position.y += 1;
                new_sand_unit_position.x += 1;
            } else {
                if (new_sand_unit_position.x == 500 and new_sand_unit_position.y == 0) {
                    sand_unit_count += 1;
                    break :top;
                }
                state.grid.getAtPtr(new_sand_unit_position.x, new_sand_unit_position.y).* = .sand;
                break;
            }
        }
    }

    return sand_unit_count;
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
        \\498,4 -> 498,6 -> 496,6
        \\503,4 -> 502,4 -> 502,9 -> 494,9
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(u64, 93), result);
}
