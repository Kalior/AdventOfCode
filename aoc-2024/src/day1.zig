const std = @import("std");

fn readFile(allocator: std.mem.Allocator, filename: []const u8) ![]u8 {
    const file = try std.fs.cwd().openFile(
        filename,
        .{},
    );
    defer file.close();

    const stat = try file.stat();
    return try file.readToEndAlloc(allocator, stat.size);
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day1");
    var lines = std.mem.split(u8, input, "\n");

    var left_list = std.ArrayList(i32).init(allocator);
    defer left_list.deinit();

    var right_list = std.ArrayList(i32).init(allocator);
    defer right_list.deinit();

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        std.debug.print("{s}.\n", .{line});

        var it = std.mem.splitSequence(u8, line, "   ");

        const left_v = try std.fmt.parseInt(i32, it.next().?, 10);
        try left_list.append(left_v);
        const right_v = try std.fmt.parseInt(i32, it.next().?, 10);
        try right_list.append(right_v);
    }

    std.mem.sort(i32, left_list.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, right_list.items, {}, comptime std.sort.asc(i32));

    var total_distance: i32 = 0;
    for (left_list.items, right_list.items) |left, right| {
        total_distance += @intCast(@abs(left - right));
    }

    std.debug.print("Total distance {}.\n", .{total_distance});

    var frequencies = std.AutoHashMap(i32, i32).init(allocator);

    for (right_list.items) |right| {
        const old = frequencies.get(right) orelse 0;

        try frequencies.put(right, old + 1);
    }

    var similarity_score: i32 = 0;
    for (left_list.items) |left| {
        const freq = frequencies.get(left) orelse 0;
        similarity_score += left * freq;
    }
    std.debug.print("Similarity score {}.\n", .{similarity_score});
}
