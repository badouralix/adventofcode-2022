const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn Vec2D(comptime T: type) type {
    return struct {
        x: T,
        y: T,
    };
}

fn Rect(comptime T: type) type {
    return Vec2D(T);
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
pub const State = struct {
    map: Grid(u8),
    start: Vec2D(usize),
    end: Vec2D(usize),

    pub fn initFromInput(input: []const u8) State {
        var dim = findDimensions(input);
        var map = Grid(u8).init(dim, 0, a);
        var start: Vec2D(usize) = undefined;
        var end: Vec2D(usize) = undefined;

        var line: usize = 0;
        for (input) |c, i| {
            if (c == '\n') {
                line += 1;
                continue;
            }

            switch (c) {
                'S' => {
                    start = Vec2D(usize){ .x = (i - line) % dim.x, .y = (i - line) / dim.x };
                    map.data[i - line] = 'a';
                },
                'E' => {
                    end = Vec2D(usize){ .x = (i - line) % dim.x, .y = (i - line) / dim.x };
                    map.data[i - line] = 'z';
                },
                else => {
                    map.data[i - line] = c;
                },
            }
        }

        return State{ .map = map, .start = start, .end = end };
    }
};

fn comparePointCost(context: *const Grid(u64), item1: Vec2D(usize), item2: Vec2D(usize)) std.math.Order {
    const item1_cost = context.getAt(item1.x, item1.y);
    const item2_cost = context.getAt(item2.x, item2.y);
    return std.math.order(item1_cost, item2_cost);
}

const PriorityQueue = std.PriorityQueue(Vec2D(usize), *const Grid(u64), comparePointCost);

fn run(input: [:0]const u8) u64 {
    var state = State.initFromInput(input);
    const dim = state.map.dim;

    var costs = Grid(u64).init(dim, std.math.maxInt(u64), a);
    costs.getAtPtr(state.end.x, state.end.y).* = 0;
    var visited = Grid(bool).init(dim, false, a);
    var in_frontier = Grid(bool).init(dim, false, a);
    in_frontier.getAtPtr(state.end.x, state.end.y).* = true;

    var frontier = PriorityQueue.init(a, &costs);
    frontier.add(state.end) catch unreachable;

    while (true) {
        if (frontier.count() == 0) @panic("Uh oh");

        var node = frontier.remove();
        visited.getAtPtr(node.x, node.y).* = true;

        const node_height = state.map.getAt(node.x, node.y);
        const node_cost = costs.getAt(node.x, node.y);
        if (node_height == 'a') {
            return node_cost;
        }

        if (node.x > 0) {
            const neighbor = Vec2D(usize){ .x = node.x - 1, .y = node.y };
            const neighbor_height = state.map.getAt(neighbor.x, neighbor.y);
            if (node_height - 1 <= neighbor_height) {
                if (visited.getAt(neighbor.x, neighbor.y) == false and in_frontier.getAt(neighbor.x, neighbor.y) == false) {
                    frontier.add(neighbor) catch unreachable;
                    in_frontier.getAtPtr(neighbor.x, neighbor.y).* = true;
                }
                var ptr = costs.getAtPtr(neighbor.x, neighbor.y);
                ptr.* = std.math.min(ptr.*, node_cost + 1);
            }
        }
        if (node.x < dim.x - 1) {
            const neighbor = Vec2D(usize){ .x = node.x + 1, .y = node.y };
            const neighbor_height = state.map.getAt(neighbor.x, neighbor.y);
            if (node_height - 1 <= neighbor_height) {
                if (visited.getAt(neighbor.x, neighbor.y) == false and in_frontier.getAt(neighbor.x, neighbor.y) == false) {
                    frontier.add(neighbor) catch unreachable;
                    in_frontier.getAtPtr(neighbor.x, neighbor.y).* = true;
                }
                var ptr = costs.getAtPtr(neighbor.x, neighbor.y);
                ptr.* = std.math.min(ptr.*, node_cost + 1);
            }
        }
        if (node.y > 0) {
            const neighbor = Vec2D(usize){ .x = node.x, .y = node.y - 1 };
            const neighbor_height = state.map.getAt(neighbor.x, neighbor.y);
            if (node_height - 1 <= neighbor_height) {
                if (visited.getAt(neighbor.x, neighbor.y) == false and in_frontier.getAt(neighbor.x, neighbor.y) == false) {
                    frontier.add(neighbor) catch unreachable;
                    in_frontier.getAtPtr(neighbor.x, neighbor.y).* = true;
                }
                var ptr = costs.getAtPtr(neighbor.x, neighbor.y);
                ptr.* = std.math.min(ptr.*, node_cost + 1);
            }
        }
        if (node.y < dim.y - 1) {
            const neighbor = Vec2D(usize){ .x = node.x, .y = node.y + 1 };
            const neighbor_height = state.map.getAt(neighbor.x, neighbor.y);
            if (node_height - 1 <= neighbor_height) {
                if (visited.getAt(neighbor.x, neighbor.y) == false and in_frontier.getAt(neighbor.x, neighbor.y) == false) {
                    frontier.add(neighbor) catch unreachable;
                    in_frontier.getAtPtr(neighbor.x, neighbor.y).* = true;
                }
                var ptr = costs.getAtPtr(neighbor.x, neighbor.y);
                ptr.* = std.math.min(ptr.*, node_cost + 1);
            }
        }
    }

    return 0;
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
        \\Sabqponm
        \\abcryxxl
        \\accszExk
        \\acctuvwj
        \\abdefghi
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(u64, 29), result);
}
