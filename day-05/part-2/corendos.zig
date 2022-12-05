const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Stack = struct {
    slots: []u8,
    current_size: usize = 0,

    fn init(size: usize, allocator: std.mem.Allocator) Stack {
        var slots = allocator.alloc(u8, size) catch unreachable;
        std.mem.set(u8, slots, 0);
        return Stack{ .slots = slots };
    }

    fn deinit(self: Stack, allocator: std.mem.Allocator) void {
        allocator.free(self.slots);
    }

    fn push(self: *Stack, c: u8) void {
        if (self.current_size >= self.slots.len) unreachable;
        self.slots[self.current_size] = c;
        self.current_size += 1;
    }

    fn pop(self: *Stack) u8 {
        if (self.current_size == 0) unreachable;
        self.current_size -= 1;
        return self.slots[self.current_size];
    }
};

fn parseStacks(input: []const u8, allocator: std.mem.Allocator) []Stack {
    var it = std.mem.splitBackwards(u8, input, "\n");
    var line_with_index = it.next() orelse unreachable;
    var stack_count = (line_with_index.len + 1) / 4;
    var stacks = allocator.alloc(Stack, stack_count) catch unreachable;
    for (stacks) |*stack| {
        stack.* = Stack.init(256, allocator);
    }

    while (it.next()) |line| {
        var i: usize = 0;
        while (i < stack_count) : (i += 1) {
            const c = line[4 * i + 1];
            if (c != ' ') {
                stacks[i].push(c);
            }
        }
    }

    return stacks;
}

fn printStacks(stacks: []const Stack) void {
    for (stacks) |stack, i| {
        stdout.print("[{}]: {s}\n", .{ i, stack.slots[0..stack.current_size] }) catch unreachable;
    }
    stdout.writeByte('\n') catch unreachable;
}

fn parseAndRunInstructions(input: []const u8, stacks: []Stack, answer: []u8) void {
    var crate_buffer: [256]u8 = undefined;
    var it = std.mem.split(u8, input, "\n");
    while (it.next()) |line| {
        var token_it = std.mem.tokenize(u8, line, " ");

        _ = token_it.next() orelse unreachable;
        const count = std.fmt.parseInt(usize, token_it.next() orelse unreachable, 10) catch unreachable;
        _ = token_it.next() orelse unreachable;
        const src_index = std.fmt.parseInt(usize, token_it.next() orelse unreachable, 10) catch unreachable;
        _ = token_it.next() orelse unreachable;
        const dst_index = std.fmt.parseInt(usize, token_it.next() orelse unreachable, 10) catch unreachable;

        var i: usize = 0;
        var buffer_index: usize = 0;
        while (i < count) : (i += 1) {
            crate_buffer[buffer_index] = stacks[src_index - 1].pop();
            buffer_index += 1;
        }

        while (buffer_index > 0) : (buffer_index -= 1) {
            stacks[dst_index - 1].push(crate_buffer[buffer_index - 1]);
        }
    }

    for (stacks) |*stack, i| {
        answer[i] = stack.slots[stack.current_size - 1];
    }
}

fn run(input: [:0]const u8) []const u8 {
    var it = std.mem.split(u8, input, "\n\n");
    const initial_stacks_str = it.next() orelse unreachable;
    var stacks = parseStacks(initial_stacks_str, a);
    var answer = a.alloc(u8, stacks.len) catch unreachable;

    parseAndRunInstructions(it.next() orelse unreachable, stacks, answer);
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
    try stdout.print("_duration:{d}\n{s}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}

test {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();
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
    try std.testing.expectEqualStrings("MCD", run(input));
}
