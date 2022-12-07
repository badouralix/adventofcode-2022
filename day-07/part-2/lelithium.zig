const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = if (builtin.is_test) std.testing.allocator else undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const TOTAL_SIZE = 70_000_000;
const UPDATE_SIZE = 30_000_000;

fn solve_day_2(dir: Dir, min_size: u64) u64 {
    if (dir.size < min_size)
        return UPDATE_SIZE; // return early - we know this directory is too big anyway
    var least = dir.size;
    //stdout.print("\nFolder {s} size {}\n", .{dir.name, dir.size}) catch unreachable;
    for (dir.subdirectories.?.items)|subdir|{
        least = @min(solve_day_2(subdir, min_size), least);
    }
    //stdout.print("Current least is {}\n", .{least}) catch unreachable;
    return least;
}

const Dir = struct {
    name: []const u8 = "",
    size: u64 = 0,
    parent: ?*Dir = null,
    subdirectories: ?std.ArrayList(Dir) = null,

    fn parse(self: *Dir, lines: *std.mem.TokenIterator(u8)) void {
        var subdirectories = std.ArrayList(Dir).init(a);
        //stdout.print("Parsing for directory {s}\n", .{self.name}) catch unreachable;
        parse: while (lines.*.next()) |line| {
            //stdout.print("{s}\n", .{line}) catch unreachable;
            switch (line[2]) { // Skip over `$ `
                'l' => { // ls
                    ls: while (lines.*.peek()) |ls_line| {
                        if (ls_line[0] == '$') {
                            // End of directory listing, break out of here
                            break :ls;
                        }
                        var ls_space_it = std.mem.tokenize(u8, ls_line, " ");
                        const ls_space_first = ls_space_it.next().?;
                        if (ls_space_first[0] != 'd') { // skip over `dir <dirname>`
                            self.size += std.fmt.parseInt(u64, ls_space_first, 10) catch unreachable;
                        }
                        _ = lines.*.next();
                    }
                },
                'c' => { // cd
                    if (line[5] == '.' and line[6] == '.'){
                        break :parse;
                    }
                    // create a new leaf
                    var subdir = Dir{
                        .name = line[5..],
                        .parent = self
                    };
                    subdir.parse(lines);
                    self.size += subdir.size;
                    subdirectories.append(subdir) catch unreachable;
                },
                else => {
                    //stdout.print("Unreachable {s}\n", .{line}) catch unreachable;
                    unreachable;
                },
            }
        }
        self.subdirectories = subdirectories;
        //stdout.print("\nFolder {s} has size {} and {} subdirs\n", .{self.name, self.size, subdirectories.items.len}) catch unreachable;
    }

    fn deinit(self: *Dir) void{
        if (self.subdirectories != null){
            for (self.subdirectories.?.items) |*subdir|{
                subdir.deinit();
            }
            self.subdirectories.?.deinit();
        }
    }

    fn print(self: *Dir) void {
        _ = self;
    }
};

fn run(input: [:0]const u8) u64 {
    var root_dir = Dir{
        .name = "/"
    };
    defer root_dir.deinit();
    var lines = std.mem.tokenize(u8, input, "\n");
    _ = lines.next(); // Skip over first `$ cd /`
    root_dir.parse(&lines);
    const free_space = TOTAL_SIZE - root_dir.size;
    return solve_day_2(root_dir, UPDATE_SIZE - free_space);
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
    try std.testing.expect(run(
        \\$ cd /
        \\$ ls
        \\dir a
        \\14848514 b.txt
        \\8504156 c.dat
        \\dir d
        \\$ cd a
        \\$ ls
        \\dir e
        \\29116 f
        \\2557 g
        \\62596 h.lst
        \\$ cd e
        \\$ ls
        \\584 i
        \\$ cd ..
        \\$ cd ..
        \\$ cd d
        \\$ ls
        \\4060174 j
        \\8033020 d.log
        \\5626152 d.ext
        \\7214296 k
    ) == 24933642);
}
