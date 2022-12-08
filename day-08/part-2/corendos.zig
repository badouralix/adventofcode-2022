const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn Rect(comptime T: type) type {
    return struct {
        x: T,
        y: T,
    };
}

fn findDimensions(input: []const u8) Rect(usize) {
    var dim = Rect(usize){ .x = 0, .y = 0 };
    var first_line_last_index = std.mem.indexOf(u8, input, "\n");
    dim.x = first_line_last_index.?;
    dim.y = std.mem.count(u8, input, "\n") + 1;
    return dim;
}

pub fn Grid(comptime T: type) type {
    return struct {
        const Self = @This();
        data: []T,
        dim: Rect(usize),

        pub fn init(dim: Rect(usize), value: T, allocator: std.mem.Allocator) Self {
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

pub const Map = struct {
    trees: Grid(u8),

    pub fn init(dim: Rect(usize), allocator: std.mem.Allocator) Map {
        var trees = Grid(u8).init(dim, 0, allocator);
        return Map{ .trees = trees };
    }

    pub fn initFromInput(input: []const u8, allocator: std.mem.Allocator) Map {
        const dim = findDimensions(input);
        var self = init(dim, allocator);
        var line: usize = 0;
        for (input) |c, i| {
            if (c == '\n') {
                line += 1;
                continue;
            }
            self.trees.data[i - line] = c - '0';
        }
        return self;
    }

    pub fn countVisibleTree(self: *const Map, allocator: std.mem.Allocator) usize {
        var visibility_grid = Grid(bool).init(self.trees.dim, false, allocator);
        var count: usize = 0;

        {
            var line_index: usize = 0;
            while (line_index < self.trees.dim.y) : (line_index += 1) {
                var column_index: usize = 0;
                var current_max_height: isize = -1;
                var current_max_height_index: usize = 0;

                // Find visible tree from left to right
                while (column_index < self.trees.dim.x) : (column_index += 1) {
                    const current_height = @intCast(isize, self.trees.getAt(column_index, line_index));
                    if (current_height > current_max_height) {
                        var visibility = visibility_grid.getAtPtr(column_index, line_index);
                        if (!visibility.*) {
                            count += 1;
                        }
                        visibility.* = true;
                        current_max_height = current_height;
                        current_max_height_index = column_index;
                    }
                }

                const max_height = current_max_height;
                current_max_height = -1;

                // Find visible tree from right to left
                column_index -= 1;
                while (column_index > current_max_height_index) : (column_index -= 1) {
                    const current_height = @intCast(isize, self.trees.getAt(column_index, line_index));
                    if (current_height > current_max_height) {
                        var visibility = visibility_grid.getAtPtr(column_index, line_index);
                        if (!visibility.*) {
                            count += 1;
                        }
                        visibility.* = true;
                        current_max_height = current_height;
                    }
                    if (current_height == max_height) break;
                }
            }
        }

        {
            var column_index: usize = 0;
            while (column_index < self.trees.dim.x) : (column_index += 1) {
                var line_index: usize = 0;
                var current_max_height: isize = -1;
                var current_max_height_index: usize = 0;

                // Find visible tree from left to right
                while (line_index < self.trees.dim.y) : (line_index += 1) {
                    const current_height = @intCast(isize, self.trees.getAt(column_index, line_index));
                    if (current_height > current_max_height) {
                        var visibility = visibility_grid.getAtPtr(column_index, line_index);
                        if (!visibility.*) {
                            count += 1;
                        }
                        visibility.* = true;
                        current_max_height = current_height;
                        current_max_height_index = line_index;
                    }
                }

                const max_height = current_max_height;
                current_max_height = -1;

                // Find visible tree from right to left
                line_index -= 1;
                while (line_index > current_max_height_index) : (line_index -= 1) {
                    const current_height = @intCast(isize, self.trees.getAt(column_index, line_index));
                    if (current_height > current_max_height) {
                        var visibility = visibility_grid.getAtPtr(column_index, line_index);
                        if (!visibility.*) {
                            count += 1;
                        }
                        visibility.* = true;
                        current_max_height = current_height;
                    }
                    if (current_height == max_height) break;
                }
            }
        }

        return count;
    }

    pub fn findHighestScenicScore(self: *const Map) usize {
        var highest_score: usize = 0;

        var y: usize = 1;
        while (y < self.trees.dim.y - 1) : (y += 1) {
            var x: usize = 1;
            while (x < self.trees.dim.x - 1) : (x += 1) {
                const current_height = self.trees.getAt(x, y);
                var scores = [_]usize{ 0, 0, 0, 0 };
                // Find Score to the west
                scores[0] = blk: {
                    var index: usize = 0;
                    while (x - index > 0) : (index += 1) {
                        if (self.trees.getAt(x - index - 1, y) >= current_height) {
                            index += 1;
                            break;
                        }
                    }
                    break :blk index;
                };

                // Find Score to the east
                scores[1] = blk: {
                    var index: usize = 0;
                    while (x + index < self.trees.dim.x - 1) : (index += 1) {
                        if (self.trees.getAt(x + index + 1, y) >= current_height) {
                            index += 1;
                            break;
                        }
                    }
                    break :blk index;
                };

                // Find Score to the north
                scores[2] = blk: {
                    var index: usize = 0;
                    while (y - index > 0) : (index += 1) {
                        if (self.trees.getAt(x, y - index - 1) >= current_height) {
                            index += 1;
                            break;
                        }
                    }
                    break :blk index;
                };

                // Find Score to the south
                scores[3] = blk: {
                    var index: usize = 0;
                    while (y + index < self.trees.dim.y - 1) : (index += 1) {
                        if (self.trees.getAt(x, y + index + 1) >= current_height) {
                            index += 1;
                            break;
                        }
                    }
                    break :blk index;
                };
                const score = scores[0] * scores[1] * scores[2] * scores[3];
                highest_score = @max(score, highest_score);
            }
        }

        return highest_score;
    }
};

fn run(input: [:0]const u8) usize {
    var map = Map.initFromInput(input, a);
    return map.findHighestScenicScore();
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
        \\30373
        \\25512
        \\65332
        \\33549
        \\35390
    ;

    try std.testing.expectEqual(@as(usize, 8), run(input));
}
