const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

pub const Operation = enum {
    add,
    mul,

    pub fn from(input: []const u8) Operation {
        if (std.mem.eql(u8, input, "*")) return .mul;
        if (std.mem.eql(u8, input, "+")) return .add;
        unreachable;
    }
};

pub const Operand = union(enum) {
    old: void,
    value: i64,

    pub fn from(input: []const u8) Operand {
        if (std.mem.eql(u8, input, "old")) return Operand{ .old = {} };
        return Operand{ .value = std.fmt.parseInt(i64, input, 10) catch unreachable };
    }
};

pub const BinaryExpr = struct {
    lhs: Operand,
    rhs: Operand,
    op: Operation,

    pub fn execute(self: *BinaryExpr, old: i64) i64 {
        std.debug.assert(self.lhs == .old);
        switch (self.op) {
            .add => {
                if (self.rhs == .old) return old + old;
                return old + self.rhs.value;
            },
            .mul => {
                if (self.rhs == .old) return old * old;
                return old * self.rhs.value;
            },
        }
    }

    pub fn from(lhs_str: []const u8, op_str: []const u8, rhs_str: []const u8) BinaryExpr {
        const lhs = Operand.from(lhs_str);
        const op = Operation.from(op_str);
        const rhs = Operand.from(rhs_str);

        return BinaryExpr{
            .lhs = lhs,
            .rhs = rhs,
            .op = op,
        };
    }
};

pub const Monkey = struct {
    const Stack = std.TailQueue(i64);
    items: Stack = Stack{},
    expr: BinaryExpr,
    test_value: i64,
    test_dst_true: usize,
    test_dst_false: usize,
    inspect_item_count: u64 = 0,

    pub fn pushItem(self: *Monkey, item: i64) void {
        var node = a.create(Stack.Node) catch unreachable;
        node.* = Stack.Node{ .data = item };
        self.items.append(node);
    }

    pub fn popItem(self: *Monkey) ?i64 {
        var node = self.items.popFirst() orelse return null;
        defer a.destroy(node);
        return node.data;
    }
};

pub const State = struct {
    monkeys: []Monkey,

    pub fn runRound(self: *State) void {
        for (self.monkeys) |*monkey| {
            while (monkey.popItem()) |item| {
                monkey.inspect_item_count += 1;
                var worry_level = item;
                worry_level = monkey.expr.execute(worry_level);
                // Monkey is bored
                worry_level = @divFloor(worry_level, 3);
                if (@rem(worry_level, monkey.test_value) == 0) {
                    self.monkeys[monkey.test_dst_true].pushItem(worry_level);
                } else {
                    self.monkeys[monkey.test_dst_false].pushItem(worry_level);
                }
            }
        }
    }

    pub fn monkeyBusiness(self: *const State) u64 {
        var max_inspected_items = [2]u64{ 0, 0 };
        for (self.monkeys) |monkey| {
            if (monkey.inspect_item_count > max_inspected_items[0]) {
                max_inspected_items[1] = max_inspected_items[0];
                max_inspected_items[0] = monkey.inspect_item_count;
            } else if (monkey.inspect_item_count > max_inspected_items[1]) {
                max_inspected_items[1] = monkey.inspect_item_count;
            }
        }

        return max_inspected_items[0] * max_inspected_items[1];
    }
};

pub fn parseInitialState(input: []const u8) State {
    var monkey_list = std.ArrayList(Monkey).initCapacity(a, 16) catch unreachable;

    var monkey_details_it = std.mem.split(u8, input, "\n\n");
    var current_monkey = Monkey{ .expr = undefined, .test_value = undefined, .test_dst_true = undefined, .test_dst_false = undefined };
    while (monkey_details_it.next()) |monkey_details| {
        var line_it = std.mem.split(u8, monkey_details, "\n");
        _ = line_it.next().?;

        // Parse Starting items
        {
            var tokens_it = std.mem.tokenize(u8, line_it.next().?, ", ");
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            while (tokens_it.next()) |item| {
                const item_index = std.fmt.parseInt(i64, item, 10) catch unreachable;
                current_monkey.pushItem(item_index);
            }
        }

        // Parse operation
        {
            var tokens_it = std.mem.split(u8, std.mem.trimLeft(u8, line_it.next().?, " "), " ");
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            current_monkey.expr = BinaryExpr.from(tokens_it.next().?, tokens_it.next().?, tokens_it.next().?);
        }

        // Parse test value
        {
            var tokens_it = std.mem.split(u8, std.mem.trimLeft(u8, line_it.next().?, " "), " ");
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            current_monkey.test_value = std.fmt.parseInt(u8, tokens_it.next().?, 10) catch unreachable;
        }

        // Parse test true destination
        {
            var tokens_it = std.mem.split(u8, std.mem.trimLeft(u8, line_it.next().?, " "), " ");
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            current_monkey.test_dst_true = std.fmt.parseInt(u8, tokens_it.next().?, 10) catch unreachable;
        }

        // Parse test false destination
        {
            var tokens_it = std.mem.split(u8, std.mem.trimLeft(u8, line_it.next().?, " "), " ");
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            _ = tokens_it.next().?;
            current_monkey.test_dst_false = std.fmt.parseInt(u8, tokens_it.next().?, 10) catch unreachable;
        }

        monkey_list.append(current_monkey) catch unreachable;
        current_monkey = Monkey{ .expr = undefined, .test_value = undefined, .test_dst_true = undefined, .test_dst_false = undefined };
    }

    return State{ .monkeys = monkey_list.toOwnedSlice() };
}

fn run(input: [:0]const u8) u64 {
    var state = parseInitialState(input);
    var i: usize = 0;
    while (i < 20) : (i += 1) {
        state.runRound();
    }
    return state.monkeyBusiness();
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
        \\Monkey 0:
        \\  Starting items: 79, 98
        \\  Operation: new = old * 19
        \\  Test: divisible by 23
        \\    If true: throw to monkey 2
        \\    If false: throw to monkey 3
        \\
        \\Monkey 1:
        \\  Starting items: 54, 65, 75, 74
        \\  Operation: new = old + 6
        \\  Test: divisible by 19
        \\    If true: throw to monkey 2
        \\    If false: throw to monkey 0
        \\
        \\Monkey 2:
        \\  Starting items: 79, 60, 97
        \\  Operation: new = old * old
        \\  Test: divisible by 13
        \\    If true: throw to monkey 1
        \\    If false: throw to monkey 3
        \\
        \\Monkey 3:
        \\  Starting items: 74
        \\  Operation: new = old + 3
        \\  Test: divisible by 17
        \\    If true: throw to monkey 0
        \\    If false: throw to monkey 1
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(u64, 10605), result);
}
