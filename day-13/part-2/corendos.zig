const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

pub fn Iterator(comptime T: type) type {
    return struct {
        const Self = @This();

        buffer: []const T,
        index: usize = 0,

        pub fn next(self: *Self) ?T {
            if (self.index >= self.buffer.len) return null;
            defer self.index += 1;
            return self.buffer[self.index];
        }

        pub fn peek(self: *Self) ?T {
            if (self.index >= self.buffer.len) return null;
            return self.buffer[self.index];
        }
    };
}

pub fn parseInteger(it: *Iterator(u8)) u64 {
    const start = it.index;
    while (it.peek()) |c| switch (c) {
        '0'...'9' => _ = it.next(),
        else => break,
    } else unreachable;

    return std.fmt.parseInt(u64, it.buffer[start..it.index], 10) catch unreachable;
}

pub fn parseItems(it: *Iterator(u8)) []PacketItem {
    _ = it.next().?;
    var item_list = std.ArrayList(PacketItem).init(a);
    while (it.peek()) |c| switch (c) {
        ']' => {
            _ = it.next();
            break;
        },
        ',' => _ = it.next(),
        else => {
            const item = PacketItem.parseImpl(it);
            item_list.append(item) catch unreachable;
        },
    };
    return item_list.toOwnedSlice();
}

pub const PacketOrder = enum {
    right,
    wrong,
    none,
};

pub const PacketItem = union(enum) {
    integer: u64,
    list: []PacketItem,

    fn parseImpl(it: *Iterator(u8)) PacketItem {
        const c = it.peek().?;
        return switch (c) {
            '0'...'9' => PacketItem{ .integer = parseInteger(it) },
            '[' => PacketItem{ .list = parseItems(it) },
            else => {
                stdout.print("Got {}\n", .{c}) catch unreachable;
                unreachable;
            },
        };
    }

    pub fn compare(item1: PacketItem, item2: PacketItem) PacketOrder {
        if (item1 == .integer and item2 == .integer) {
            if (item1.integer < item2.integer) {
                return .right;
            } else if (item1.integer > item2.integer) {
                return .wrong;
            }
            return .none;
        } else if (item1 == .list and item2 == .list) {
            const min_length = @min(item1.list.len, item2.list.len);
            var i: usize = 0;
            while (i < min_length) : (i += 1) {
                const order = PacketItem.compare(item1.list[i], item2.list[i]);
                if (order != .none) return order;
            }
            if (item1.list.len < item2.list.len) {
                return .right;
            } else if (item1.list.len > item2.list.len) {
                return .wrong;
            }
            return .none;
        } else if (item1 == .list and item2 == .integer) {
            const adapted_item = PacketItem{ .list = &[_]PacketItem{item2} };
            return compare(item1, adapted_item);
        } else if (item1 == .integer and item2 == .list) {
            const adapted_item = PacketItem{ .list = &[_]PacketItem{item1} };
            return compare(adapted_item, item2);
        } else unreachable;
    }
};

pub const Packet = struct {
    items: []PacketItem,

    pub fn compare(packet1: Packet, packet2: Packet) PacketOrder {
        const item1 = PacketItem{ .list = packet1.items };
        const item2 = PacketItem{ .list = packet2.items };
        return PacketItem.compare(item1, item2);
    }

    pub fn parse(input: []const u8) Packet {
        var it = Iterator(u8){ .buffer = input };
        const item = PacketItem.parseImpl(&it);
        return Packet{ .items = item.list };
    }
};

fn lessThan(context: void, lhs: Packet, rhs: Packet) bool {
    _ = context;
    return switch (Packet.compare(lhs, rhs)) {
        .right => true,
        .wrong => false,
        .none => unreachable,
    };
}

fn run(input: [:0]const u8) usize {
    var packet_list = std.ArrayList(Packet).init(a);
    const divider2_packet = Packet.parse("[[2]]");
    const divider6_packet = Packet.parse("[[6]]");
    packet_list.append(divider2_packet) catch unreachable;
    packet_list.append(divider6_packet) catch unreachable;

    var it = std.mem.split(u8, input, "\n\n");
    while (it.next()) |packets| {
        var packet_it = std.mem.split(u8, packets, "\n");
        var packet1 = Packet.parse(packet_it.next().?);
        var packet2 = Packet.parse(packet_it.next().?);
        switch (Packet.compare(packet1, packet2)) {
            .right => {
                packet_list.append(packet1) catch unreachable;
                packet_list.append(packet2) catch unreachable;
            },
            .wrong => {
                packet_list.append(packet2) catch unreachable;
                packet_list.append(packet1) catch unreachable;
            },
            .none => unreachable,
        }
    }

    var packets = packet_list.toOwnedSlice();
    std.sort.sort(Packet, packets, {}, lessThan);

    var divider2_index: ?usize = null;
    var divider6_index: ?usize = null;
    for (packets) |p, i| {
        if (Packet.compare(p, divider2_packet) == .none) {
            divider2_index = i + 1;
        } else if (Packet.compare(p, divider6_packet) == .none) {
            divider6_index = i + 1;
        }
    }

    return divider2_index.? * divider6_index.?;
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
        \\[1,1,3,1,1]
        \\[1,1,5,1,1]
        \\
        \\[[1],[2,3,4]]
        \\[[1],4]
        \\
        \\[9]
        \\[[8,7,6]]
        \\
        \\[[4,4],4,4]
        \\[[4,4],4,4,4]
        \\
        \\[7,7,7,7]
        \\[7,7,7]
        \\
        \\[]
        \\[3]
        \\
        \\[[[]]]
        \\[[]]
        \\
        \\[1,[2,[3,[4,[5,6,7]]]],8,9]
        \\[1,[2,[3,[4,[5,6,0]]]],8,9]
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(usize, 140), result);
}
