const std = @import("std");
const builtin = @import("builtin");

pub fn DeQueue(comptime T: type) type {
    // Quick DeQueue inspired from https://ziglang.org/learn/samples/
    return struct {
        const This = @This();
        const Node = struct { data: T, next: ?*Node, prev: ?*Node };

        allocator: std.mem.Allocator,
        start: ?*Node,
        end: ?*Node,

        pub fn init(allocator: std.mem.Allocator) This {
            return This{
                .allocator = allocator,
                .start = null,
                .end = null,
            };
        }
        pub fn append(this: *This, value: T) !void {
            const node = try this.allocator.create(Node);
            node.* = .{ .data = value, .next = null, .prev = this.end };
            if (this.end) |end| end.next = node //
            else this.start = node;
            this.end = node;
        }
        pub fn prepend(this: *This, value: T) !void {
            const node = try this.allocator.create(Node);
            node.* = .{ .data = value, .next = this.start, .prev = null };
            if (this.start) |start| start.prev = node //
            else this.end = node;
            this.start = node;
        }
        pub fn pop(this: *This) ?T {
            const start = this.start orelse return null;
            defer this.allocator.destroy(start);
            if (start.next) |next|
                this.start = next
            else {
                this.start = null;
                this.end = null;
            }
            return start.data;
        }
    };
}

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const COLUMN_COUNT = if (builtin.is_test) 3 else 9;

fn run(input: [:0]const u8) [:0]const u8 {
    var out = [_:0]u8{0} ** COLUMN_COUNT;
    var line_it = std.mem.split(u8, input, "\n");
    var queues = [_]DeQueue(u8){DeQueue(u8).init(if (builtin.is_test) std.testing.allocator else a)} ** COLUMN_COUNT;
    // Build stacks
    while (line_it.next()) |line| {
        if (line.len == 0)
            break; // Break once column definition is done;
        inline for (queues) |*queue, col_idx| {
            const idx = 1 + col_idx * 4;
            if (idx > line.len) // skip badly-formatted inputs
                break;
            const c = line[idx];
            if (c >= 'A') // Ignore col index
                queue.append(c) catch unreachable;
        }
    }
    // Apply moves
    while (line_it.next()) |line| {
        // Parse line
        var parts_it = std.mem.split(u8, line, " ");
        _ = parts_it.next(); // discard "move"
        var amount = std.fmt.parseInt(u16, parts_it.next().?, 10) catch unreachable;
        _ = parts_it.next(); // discard "from"
        const source = std.fmt.parseInt(u16, parts_it.next().?, 10) catch unreachable;
        _ = parts_it.next(); // discard "to"
        const destination = std.fmt.parseInt(u16, parts_it.next().?, 10) catch unreachable;
        while (amount != 0) : (amount -= 1)
            queues[destination - 1].prepend(queues[source - 1].pop().?) catch unreachable; // Prepend to reverse order
    }

    // Get first boxes
    inline for (queues) |*queue, col_idx| {
        out[col_idx] = queue.pop().?;
    }

    // Cleanup
    for (queues) |*queue, q_idx| {
        _ = q_idx;
        //stdout.print("q_idx {}:", .{q_idx}) catch unreachable;
        while (queue.pop()) |val| {
            _ = val;
            //stdout.print(" {c}", .{val}) catch unreachable;
        }
        //stdout.print("\n", .{}) catch unreachable;
    }
    return &out;
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
    try stdout.print("_duration:{d}\n{s}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}

test "ez" {
    const input =
        \\    [D]    
        \\[N] [C]    
        \\[Z] [M] [P]
        \\ 1   2   3 
        \\
        \\move 1 from 2 to 1
        \\move 3 from 1 to 3
        \\move 2 from 2 to 1
        \\move 1 from 1 to 2
    ;
    try stdout.print("\n", .{});
    const ans = run(input);
    try std.testing.expect(std.mem.eql(u8, ans, "CMZ"));
}
