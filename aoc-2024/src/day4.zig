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

fn parse(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList([]const u8) {
    var lines = std.mem.split(u8, input, "\n");

    var values = std.ArrayList([]const u8).init(allocator);
    errdefer values.deinit();

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        try values.append(line);
    }

    return values;
}

fn checkAt(word_search: std.ArrayList([]const u8), i: usize, j: usize) i32 {
    var n_xmas: i32 = 0;
    if (i > 2) {
        const vertical = word_search.items[j][i - 3 .. i + 1];
        if (std.mem.eql(u8, vertical, "XMAS") or std.mem.eql(u8, vertical, "SAMX")) {
            n_xmas += 1;
        }
    }

    if (j > 2) {
        const horizontal = [_]u8{ word_search.items[j - 3][i], word_search.items[j - 2][i], word_search.items[j - 1][i], word_search.items[j][i] };
        if (std.mem.eql(u8, &horizontal, "XMAS") or std.mem.eql(u8, &horizontal, "SAMX")) {
            n_xmas += 1;
        }
    }

    if (j > 2 and i > 2) {
        const diagonal = [_]u8{ word_search.items[j - 3][i - 3], word_search.items[j - 2][i - 2], word_search.items[j - 1][i - 1], word_search.items[j][i] };
        if (std.mem.eql(u8, &diagonal, "XMAS") or std.mem.eql(u8, &diagonal, "SAMX")) {
            n_xmas += 1;
        }
    }

    if (i > 2 and j < word_search.items.len - 3) {
        const diagonal = [_]u8{ word_search.items[j + 3][i - 3], word_search.items[j + 2][i - 2], word_search.items[j + 1][i - 1], word_search.items[j][i] };
        if (std.mem.eql(u8, &diagonal, "XMAS") or std.mem.eql(u8, &diagonal, "SAMX")) {
            n_xmas += 1;
        }
    }
    return n_xmas;
}

fn checkXmasAt(word_search: std.ArrayList([]const u8), i: usize, j: usize) i32 {
    if (j > 0 and i > 0 and i < word_search.items[j].len - 1 and j < word_search.items.len - 1) {
        const left_diagonal = [_]u8{ word_search.items[j - 1][i - 1], word_search.items[j][i], word_search.items[j + 1][i + 1] };
        const right_diagonal = [_]u8{ word_search.items[j - 1][i + 1], word_search.items[j][i], word_search.items[j + 1][i - 1] };

        const left_is_mas = std.mem.eql(u8, &left_diagonal, "MAS") or std.mem.eql(u8, &left_diagonal, "SAM");
        const right_is_mas = std.mem.eql(u8, &right_diagonal, "MAS") or std.mem.eql(u8, &right_diagonal, "SAM");
        if (left_is_mas and right_is_mas) {
            return 1;
        }
    }

    return 0;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day4");
    const word_search = try parse(allocator, input);
    defer word_search.deinit();

    // var n_xmas: i32 = 0;
    // for (0..word_search.items.len) |j| {
    //     for (0..word_search.items[j].len) |i| {
    //         n_xmas += checkAt(word_search, i, j);
    //     }
    // }

    // std.debug.print("Part one {}\n", .{n_xmas});

    var n_x_mas: i32 = 0;
    for (0..word_search.items.len) |j| {
        for (0..word_search.items[j].len) |i| {
            n_x_mas += checkXmasAt(word_search, i, j);
        }
    }

    std.debug.print("Part two {}\n", .{n_x_mas});
}

pub fn main() !void {
    try solve();
}

test "case one" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const input =
        \\..X...
        \\.SAMX.
        \\.A..A.
        \\XMAS.S
        \\.X....
    ;
    const word_search = try parse(allocator, input);
    try std.testing.expect(checkAt(word_search, 4, 1) == 1);

    try std.testing.expect(checkAt(word_search, 1, 4) == 1);

    try std.testing.expect(checkAt(word_search, 5, 3) == 1);
}

test "case two" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const input =
        \\....XXMAS.
        \\.SAMXMS...
        \\...S..A...
        \\..A.A.MS.X
        \\XMASAMX.MM
        \\X.....XA.A
        \\S.S.S.S.SS
        \\.A.A.A.A.A
        \\..M.M.M.MM
        \\.X.X.XMASX
    ;
    const word_search = try parse(allocator, input);
    // Linear forward cases
    try std.testing.expect(checkAt(word_search, 8, 0) == 1);

    try std.testing.expect(checkAt(word_search, 3, 4) == 1);

    try std.testing.expect(checkAt(word_search, 8, 9) == 1);

    // Linear backward cases
    try std.testing.expect(checkAt(word_search, 4, 1) == 1);

    try std.testing.expect(checkAt(word_search, 6, 4) == 2);
}

test "part two" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const input =
        \\.M.S......
        \\..A..MSMS.
        \\.M.S.MAA..
        \\..A.ASMSM.
        \\.M.S.M....
        \\..........
        \\S.S.S.S.S.
        \\.A.A.A.A..
        \\M.M.M.M.M.
        \\..........
    ;
    const word_search = try parse(allocator, input);
    // Linear forward cases
    try std.testing.expect(checkXmasAt(word_search, 2, 1) == 1);
}
