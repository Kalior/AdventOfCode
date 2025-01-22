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

fn parse(allocator: std.mem.Allocator, input: []const u8) !std.ArrayList(i128) {
    var vs = std.mem.splitSequence(u8, input, " ");

    var values = std.ArrayList(i128).init(allocator);
    errdefer values.deinit();

    while (vs.next()) |v| {
        const v_parsed = try std.fmt.parseInt(i128, v, 10);
        try values.append(v_parsed);
    }

    return values;
}

fn blink(stones: std.ArrayList(i128)) !std.ArrayList(i128) {
    var new_stones = std.ArrayList(i128).init(stones.allocator);
    errdefer new_stones.deinit();

    for (stones.items) |stone| {
        var buf: [99999]u8 = undefined;
        const stoneAsString = try std.fmt.bufPrint(&buf, "{}", .{stone});
        if (stone == 0) {
            try new_stones.append(1);
        } else if (stoneAsString.len % 2 == 0) {
            const halfLen = stoneAsString.len / 2;

            const left = try std.fmt.parseInt(i128, stoneAsString[0..halfLen], 10);
            const right = try std.fmt.parseInt(i128, stoneAsString[halfLen..], 10);

            try new_stones.append(left);
            try new_stones.append(right);
        } else {
            try new_stones.append(stone * 2024);
        }
    }

    return new_stones;
}

const StoneBlink = struct {
    stone: i128,
    n_blinks: usize,
};

fn blinkStone(stone: i128, n_blinks: usize, cache: *std.AutoHashMap(StoneBlink, i128), allocator: std.mem.Allocator) !i128 {
    const cache_key = StoneBlink{ .stone = stone, .n_blinks = n_blinks };
    if (cache.contains(cache_key)) {
        return cache.get(cache_key).?;
    }

    if (n_blinks == 0) {
        return 1;
    }

    var stones = std.ArrayList(i128).init(allocator);
    defer stones.deinit();
    try stones.append(stone);

    stones = try blink(stones);

    var size: i128 = 0;
    for (stones.items) |s| {
        size += try blinkStone(s, n_blinks - 1, cache, allocator);
    }

    try cache.put(cache_key, size);

    return size;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day11");
    var stones = try parse(allocator, input);
    defer stones.deinit();

    var small_cache = std.AutoHashMap(StoneBlink, i128).init(allocator);
    var size: i128 = 0;
    for (stones.items) |stone| {
        size += try blinkStone(stone, 25, &small_cache, allocator);
    }

    std.debug.print("Part one {}\n", .{size});

    var cache = std.AutoHashMap(StoneBlink, i128).init(allocator);
    size = 0;
    for (stones.items) |stone| {
        size += try blinkStone(stone, 75, &cache, allocator);
    }

    std.debug.print("Part two {}\n", .{size});
}

pub fn main() !void {
    try solve();
}
