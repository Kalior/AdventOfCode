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

fn parse(allocator: std.mem.Allocator, input: []u8) !std.ArrayList(i32) {
    var lines = std.mem.split(u8, input, "\n");

    var values = std.ArrayList(i32).init(allocator);
    errdefer values.deinit();

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        const v_parsed = try std.fmt.parseInt(i32, v, 10);
        try values.append(v_parsed);
    }

    return values;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day2");
    const reports = try parse(allocator, input);
    defer reports.deinit();

    std.debug.print("Part one {}\n", .{-1});

    std.debug.print("Part two {}\n", .{-1});
}

pub fn main() !void {
    try solve();
}
