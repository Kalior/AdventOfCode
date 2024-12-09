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

fn parse(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(i32) {
    var lines = std.mem.splitSequence(u8, input, "\n");

    var values = std.ArrayList(i32).init(allocator);
    errdefer values.deinit();

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        const v_parsed = try std.fmt.parseInt(i32, line, 10);
        try values.append(v_parsed);
    }

    return values;
}

fn check(allocator: std.mem.Allocator, line: []const u8, with_concat: bool) !i128 {
    var vs = std.mem.split(u8, line, ": ");
    const result = try std.fmt.parseInt(i128, vs.next().?, 10);

    const operands = vs.next().?;

    var operands_it = std.mem.splitSequence(u8, operands, " ");

    var values = std.ArrayList(i128).init(allocator);
    defer values.deinit();

    while (operands_it.next()) |new_v| {
        if (new_v.len == 0) {
            continue;
        }
        const new_v_parsed = try std.fmt.parseInt(i128, new_v, 10);

        var new_values = std.ArrayList(i128).init(allocator);
        //defer new_values.deinit();
        for (values.items) |v| {
            try new_values.append(v + new_v_parsed);
            try new_values.append(v * new_v_parsed);

            if (with_concat) {
                var b: [32]u8 = undefined;
                const b_w = try std.fmt.bufPrint(&b, "{}{}", .{ v, new_v_parsed });
                try new_values.append(try std.fmt.parseInt(i128, b_w, 10));
            }
        }
        if (new_values.items.len == 0) {
            try new_values.append(new_v_parsed);
        }
        values = new_values;
    }

    for (values.items) |v| {
        if (v == result) {
            return result;
        }
    }

    return 0;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day7");

    var lines = std.mem.split(u8, input, "\n");
    var works: i128 = 0;
    var works_2: i128 = 0;
    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        works += try check(allocator, lineg, false);
        works_2 += try check(allocator, line, true);
    }

    std.debug.print("Part one {}\n", .{works});

    std.debug.print("Part two {}\n", .{works_2});
}

pub fn main() !void {
    try solve();
}
