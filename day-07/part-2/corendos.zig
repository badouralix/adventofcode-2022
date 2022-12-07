const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

pub const FSNodeType = enum {
    file,
    directory,
};

pub const FSNode = struct {
    first_child: ?*FSNode = null,
    last_child: ?*FSNode = null,
    next: ?*FSNode = null,
    type: FSNodeType,
    name: []const u8,
    size: usize,

    pub fn deinit(allocator: std.mem.Allocator) void {
        _ = allocator;
    }

    pub fn allocDirectory(name: []const u8) *FSNode {
        var node = a.create(FSNode) catch unreachable;
        node.* = FSNode{ .type = .directory, .name = name, .size = 0 };
        return node;
    }

    pub fn allocFile(name: []const u8, size: usize) *FSNode {
        var node = a.create(FSNode) catch unreachable;
        node.* = FSNode{ .type = .file, .name = name, .size = size };
        return node;
    }

    pub fn find(self: *FSNode, name: []const u8) ?*FSNode {
        var current: ?*FSNode = self;
        while (current) |_| : (current = current.?.next) {
            if (std.mem.eql(u8, current.?.name, name)) return current;
        }
        return null;
    }

    pub fn addChild(self: *FSNode, new_child: *FSNode) void {
        if (self.first_child == null) {
            self.first_child = new_child;
            self.last_child = new_child;
        } else {
            self.last_child.?.next = new_child;
            self.last_child = new_child;
        }
    }

    pub fn printSiblings(self: *const FSNode) void {
        stdout.print("START\n", .{}) catch unreachable;
        var current_opt: ?*const FSNode = self;
        while (current_opt) |current| : (current_opt = current.next) {
            stdout.print("dir {s}\n", .{current.name}) catch unreachable;
        }
        stdout.print("END\n", .{}) catch unreachable;
    }

    pub fn printTree(self: *const FSNode, indentation: usize, depth: usize, max_depth: usize) void {
        stdout.writeByteNTimes(' ', indentation) catch unreachable;
        stdout.print("- {s} ({s}, size={})\n", .{ self.name, @tagName(self.type), self.size }) catch unreachable;
        if (depth < max_depth) {
            var current_opt = self.first_child;
            while (current_opt) |current| : (current_opt = current.next) {
                current.printTree(indentation + 2, depth + 1, max_depth);
            }
        }
    }

    pub fn findSmallDirectoryTotalSum(self: *const FSNode) usize {
        var sum: usize = 0;

        if (self.type == .directory and self.size <= 100000) {
            sum += self.size;
        }

        var current_opt = self.first_child;
        while (current_opt) |current| : (current_opt = current.next) {
            sum += current.findSmallDirectoryTotalSum();
        }
        return sum;
    }

    pub fn findSmallestDirectoryBiggerThan(self: *const FSNode, size: u64, current_smallest_one: *u64) void {
        if (self.type == .directory and self.size >= size and self.size < current_smallest_one.*) {
            current_smallest_one.* = self.size;
        }

        var current_opt = self.first_child;
        while (current_opt) |current| : (current_opt = current.next) {
            current.findSmallestDirectoryBiggerThan(size, current_smallest_one);
        }
    }
};

pub const State = struct {
    const Stack = std.TailQueue(*FSNode);
    root: ?*FSNode = null,
    stack: Stack = Stack{},

    pub fn pushDirectory(self: *State, dir: *FSNode) void {
        var stack_node = a.create(Stack.Node) catch unreachable;
        stack_node.* = Stack.Node{ .data = dir };
        self.stack.append(stack_node);
    }

    pub fn popDirectory(self: *State) *FSNode {
        var stack_node = self.stack.pop().?;
        var node = stack_node.data;
        a.destroy(stack_node);
        return node;
    }

    pub fn currentDirectory(self: *State) *FSNode {
        return self.stack.last.?.data;
    }
};

pub fn parseCommand(it: *std.mem.SplitIterator(u8), state: *State) void {
    const command = it.next().?;
    if (std.mem.eql(u8, command, "cd")) {
        const destination = it.next().?;
        if (std.mem.eql(u8, destination, "/")) {
            state.root = FSNode.allocDirectory(destination);
            state.pushDirectory(state.root.?);
        } else if (std.mem.eql(u8, destination, "..")) {
            const item = state.popDirectory();
            var current_directory = state.currentDirectory();
            current_directory.size += item.size;
        } else {
            var current_directory = state.currentDirectory();
            var child_directory = current_directory.first_child.?.find(destination);
            state.pushDirectory(child_directory.?);
        }
    } else if (std.mem.eql(u8, command, "ls")) {} else unreachable;
}

pub fn parseOutput(it: *std.mem.SplitIterator(u8), first_token: []const u8, state: *State) void {
    if (std.mem.eql(u8, first_token, "dir")) {
        const name = it.next().?;
        var new_dir = FSNode.allocDirectory(name);
        var current_directory = state.currentDirectory();
        current_directory.addChild(new_dir);
    } else {
        const name = it.next().?;
        const size = std.fmt.parseInt(usize, first_token, 10) catch unreachable;
        var new_file = FSNode.allocFile(name, size);
        var current_directory = state.currentDirectory();
        current_directory.addChild(new_file);
        current_directory.size += size;
    }
}

fn run(input: [:0]const u8) u64 {
    var state = State{};
    var it = std.mem.split(u8, input, "\n");
    while (it.next()) |line| {
        var token_it = std.mem.split(u8, line, " ");
        const first_token = token_it.first();
        if (std.mem.eql(u8, first_token, "$")) {
            parseCommand(&token_it, &state);
        } else {
            parseOutput(&token_it, first_token, &state);
        }
    }
    while (!std.mem.eql(u8, state.stack.last.?.data.name, "/")) {
        var item = state.popDirectory();
        var current_directory = state.currentDirectory();
        current_directory.size += item.size;
    }

    const free_space = 70000000 - state.root.?.size;
    const space_to_free = 30000000 - free_space;

    var size: u64 = std.math.maxInt(u64);
    state.root.?.findSmallestDirectoryBiggerThan(space_to_free, &size);
    return size;
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
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();
    const input =
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
    ;

    try std.testing.expectEqual(@as(u64, 24933642), run(input));
}
