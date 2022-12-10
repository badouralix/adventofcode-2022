const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

pub const Cpu = struct {
    x_register: i64 = 1,
    cycles: usize = 0,

    pub fn signalStrength(self: *const Cpu) i64 {
        return self.x_register * @intCast(i64, self.cycles);
    }

    pub fn updateSignalStrength(self: *const Cpu, signal_strength_sum: *i64) void {
        if ((self.cycles + 20) % 40 == 0) {
            signal_strength_sum.* += self.signalStrength();
            //stdout.print("Cycle: {} - X: {} - Strength: {} - Sum: {}\n", .{ self.cycles, self.x_register, self.signalStrength(), signal_strength_sum.* }) catch unreachable;
        }
    }
};

fn run(input: [:0]const u8) i64 {
    var cpu = Cpu{};
    var signal_strength_sum: i64 = 0;
    var it = std.mem.split(u8, input, "\n");
    while (it.next()) |line| {
        var token_it = std.mem.split(u8, line, " ");
        const op = token_it.next().?;
        //stdout.print("Cycle: {} - {s}\n", .{ cpu.cycles, line }) catch unreachable;
        if (std.mem.eql(u8, op, "noop")) {
            cpu.cycles += 1;
            cpu.updateSignalStrength(&signal_strength_sum);
        } else if (std.mem.eql(u8, op, "addx")) {
            var value = std.fmt.parseInt(i64, token_it.next().?, 10) catch unreachable;
            cpu.cycles += 1;
            cpu.updateSignalStrength(&signal_strength_sum);
            cpu.cycles += 1;
            cpu.updateSignalStrength(&signal_strength_sum);
            cpu.x_register += value;
        } else unreachable;
    }
    return signal_strength_sum;
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
        \\addx 15
        \\addx -11
        \\addx 6
        \\addx -3
        \\addx 5
        \\addx -1
        \\addx -8
        \\addx 13
        \\addx 4
        \\noop
        \\addx -1
        \\addx 5
        \\addx -1
        \\addx 5
        \\addx -1
        \\addx 5
        \\addx -1
        \\addx 5
        \\addx -1
        \\addx -35
        \\addx 1
        \\addx 24
        \\addx -19
        \\addx 1
        \\addx 16
        \\addx -11
        \\noop
        \\noop
        \\addx 21
        \\addx -15
        \\noop
        \\noop
        \\addx -3
        \\addx 9
        \\addx 1
        \\addx -3
        \\addx 8
        \\addx 1
        \\addx 5
        \\noop
        \\noop
        \\noop
        \\noop
        \\noop
        \\addx -36
        \\noop
        \\addx 1
        \\addx 7
        \\noop
        \\noop
        \\noop
        \\addx 2
        \\addx 6
        \\noop
        \\noop
        \\noop
        \\noop
        \\noop
        \\addx 1
        \\noop
        \\noop
        \\addx 7
        \\addx 1
        \\noop
        \\addx -13
        \\addx 13
        \\addx 7
        \\noop
        \\addx 1
        \\addx -33
        \\noop
        \\noop
        \\noop
        \\addx 2
        \\noop
        \\noop
        \\noop
        \\addx 8
        \\noop
        \\addx -1
        \\addx 2
        \\addx 1
        \\noop
        \\addx 17
        \\addx -9
        \\addx 1
        \\addx 1
        \\addx -3
        \\addx 11
        \\noop
        \\noop
        \\addx 1
        \\noop
        \\addx 1
        \\noop
        \\noop
        \\addx -13
        \\addx -19
        \\addx 1
        \\addx 3
        \\addx 26
        \\addx -30
        \\addx 12
        \\addx -1
        \\addx 3
        \\addx 1
        \\noop
        \\noop
        \\noop
        \\addx -9
        \\addx 18
        \\addx 1
        \\addx 2
        \\noop
        \\noop
        \\addx 9
        \\noop
        \\noop
        \\noop
        \\addx -1
        \\addx 2
        \\addx -37
        \\addx 1
        \\addx 3
        \\noop
        \\addx 15
        \\addx -21
        \\addx 22
        \\addx -6
        \\addx 1
        \\noop
        \\addx 2
        \\addx 1
        \\noop
        \\addx -10
        \\noop
        \\noop
        \\addx 20
        \\addx 1
        \\addx 2
        \\addx 2
        \\addx -6
        \\addx -11
        \\noop
        \\noop
        \\noop
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(i64, 13140), result);
}
