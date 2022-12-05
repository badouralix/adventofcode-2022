const std = @import("std");

fn get_size() usize {
    if (@import("builtin").is_test) {
        return 3;
    } else {
        return 9;
    }
}

fn get_height() usize {
    if (@import("builtin").is_test) {
        return 3;
    } else {
        return 8;
    }
}

const max_stack_size = 256;
const containers = get_size();
const height = get_height();
const line_length = containers * 4;

var word: [containers]u8 = .{'!'} ** containers;

const Stacks = struct {
    stacks: [max_stack_size * containers]u8,

    fn init() Stacks {
        return Stacks{ .stacks = [_]u8{0} ** (max_stack_size * containers) };
    }

    fn push(self: *Stacks, idx: usize, value: u8) void {
        var cur_idx: *u8 = &self.stacks[idx * max_stack_size];
        cur_idx.* += 1;
        self.stacks[idx * max_stack_size + 1 + cur_idx.*] = value;
    }

    fn pop(self: *Stacks, idx: usize) u8 {
        var cur_idx: *u8 = &self.stacks[idx * max_stack_size];
        const value = self.stacks[idx * max_stack_size + 1 + cur_idx.*];
        cur_idx.* -= 1;
        return value;
    }

    fn add_line(self: *Stacks, line: []const u8) void {
        var i: usize = 0;
        while (i < containers) : (i += 1) {
            var char = line[(i * 4) + 1];
            if (char != ' ') {
                self.push(i, char);
            }
        }
    }

    fn move(self: *Stacks, amount: u32, from: u32, to: u32) void {
        var i: usize = 0;
        var from_idx : *u8 = &self.stacks[from * max_stack_size];
        var to_idx: *u8 = &self.stacks[to * max_stack_size];
        while (i < amount) : (i += 1) {
            self.stacks[to * max_stack_size + 2 + to_idx.* + i] = self.stacks[from * max_stack_size + 2 + from_idx.* - amount + i];
        }
        from_idx.* -= @intCast(u8, amount);
        to_idx.* += @intCast(u8, amount);
    }

    fn fill_word(self: *Stacks) void {
        var i: usize = 0;
        while (i < containers) : (i += 1) {
            var idx = self.stacks[i * max_stack_size];
            word[i] = self.stacks[i * max_stack_size + 1 + idx];
        }
    }
};

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) void {
    var stacks: Stacks = Stacks.init();
    var i: usize = 0;
    while (i < height) : (i += 1) {
        var line = input[(height - i - 1) * line_length .. (height - i) * line_length];
        stacks.add_line(line);
    }
    // your code here
    var rest = input[(height + 1) * line_length + 2 ..];
    var lines = std.mem.tokenize(u8, rest, "\n");
    while (lines.next()) |line| {
        var parts = std.mem.tokenize(u8, line, " movefromto");
        var amount = std.fmt.parseInt(u32, parts.next().?, 10) catch unreachable;
        var from = std.fmt.parseInt(u32, parts.next().?, 10) catch unreachable;
        var to = std.fmt.parseInt(u32, parts.next().?, 10) catch unreachable;
        stacks.move(amount, from - 1, to - 1);
    }
    stacks.fill_word();
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();

    var arg_it = try std.process.argsWithAllocator(a);
    _ = arg_it.skip(); // skip over exe name
    const input: [:0]const u8 = arg_it.next().?;

    const start: i128 = std.time.nanoTimestamp(); // start time
    run(input); // compute answer
    const end: i128 = std.time.nanoTimestamp();
    const elapsed_nano = @intToFloat(f64, end - start);
    const elapsed_milli = elapsed_nano / 1_000_000.0;
    try stdout.print("_duration:{d}\n{s}\n", .{ elapsed_milli, word }); // emit actual lines parsed by AOC
}

test "example" {
    const input: [:0]const u8 =
        \\    [D]    
        \\[N] [C]    
        \\[Z] [M] [P]
        \\ 1   2   3 
        \\move 1 from 2 to 1
        \\move 3 from 1 to 3
        \\move 2 from 2 to 1
        \\move 1 from 1 to 2
    ;
    run(input);
    try std.testing.expectEqual(word, .{ 'C', 'M', 'Z' });
}