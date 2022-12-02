const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const O_ROCK = 'A';
const O_PAPER = 'B';
const O_SCISSORS = 'C';

const M_ROCK = 'X';
const M_PAPER = 'Y';
const M_SCISSORS = 'Z';

const S_ROCK = 1;
const S_PAPER = 2;
const S_SCISSORS = 3;

const S_LOSS = 0;
const S_TIE = 3;
const S_WIN = 6;

fn run(input: [:0]const u8) u64 {
    var play_it = std.mem.tokenize(u8, input, "\n");

    var score: u64 = 0;

    while (play_it.next()) |play| {
        const opp = play[0];
        const me = play[2];

        switch (me) {
            M_ROCK => {
                score += S_ROCK;
                score += switch (opp) {
                    O_ROCK => S_TIE,
                    O_PAPER => S_LOSS,
                    else => S_WIN, // O_SCISSORS
                };
            },
            M_PAPER => {
                score += S_PAPER;
                score += switch (opp) {
                    O_ROCK => S_WIN,
                    O_PAPER => S_TIE,
                    else => S_LOSS, // O_SCISSORS
                };
            },
            else => { // M_SCISSORS
                score += S_SCISSORS;
                score += switch (opp) {
                    O_ROCK => S_LOSS,
                    O_PAPER => S_WIN,
                    else => S_TIE, // O_SCISSORS
                };
            }
        }
        //stdout.print("{c} {c} {d}\n", .{opp, me, score}) catch unreachable;
    }
    return score;
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

test "ez" {
    const input =
        \\A Y
        \\B X
        \\C Z
    ;
    var buf = input.*;
    try stdout.print("\n",.{});
    const ans = run(&buf);
    try std.testing.expect(ans == 15);
}
