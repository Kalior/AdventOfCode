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

const Map = std.ArrayList(Row);
const Row = std.ArrayList(i32);

fn parse(allocator: std.mem.Allocator, input: []u8) !Map {
    var lines = std.mem.split(u8, input, "\n");

    var map = Map.init(allocator);
    errdefer map.deinit();

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }

        var row = Row.init(allocator);

        for (line) |v| {
            const v_parsed = try std.fmt.parseInt(i32, &[_]u8{v}, 10);
            try row.append(v_parsed);
        }
        try map.append(row);
    }

    return map;
}

const Point = struct {
    x: i32,
    y: i32,
};

fn add(a: Point, b: Point) Point {
    return .{ .x = a.x + b.x, .y = a.y + b.y };
}

fn scoreTrailhead(trailhead: Point, map: Map, allocator: std.mem.Allocator) !i32 {
    var locations = std.ArrayList(Point).init(allocator);
    try locations.append(trailhead);

    var visited = std.AutoHashMap(Point, bool).init(allocator);

    var score: i32 = 0;

    while (locations.items.len != 0) {
        const point = locations.pop();
        if (map.items[@intCast(point.y)].items[@intCast(point.x)] == 9 and !(visited.get(point) orelse false)) {
            try visited.put(point, true);
            score += 1;
        } else {
            const current_elevation = map.items[@intCast(point.y)].items[@intCast(point.x)];

            const directions = [_]Point{
                .{ .x = 0, .y = -1 },
                .{ .x = 0, .y = 1 },
                .{ .x = -1, .y = 0 },
                .{ .x = 1, .y = 0 },
            };
            for (directions) |dir| {
                const new_point = add(point, dir);

                if (new_point.x < 0 or new_point.x >= map.items[0].items.len or new_point.y < 0 or new_point.y >= map.items.len) {
                    continue;
                }

                const new_elevation = map.items[@intCast(new_point.y)].items[@intCast(new_point.x)];
                if (new_elevation - current_elevation == 1) {
                    try locations.append(new_point);
                }
            }
        }
    }
    return score;
}

fn rateTrailhead(trailhead: Point, map: Map, allocator: std.mem.Allocator) !i32 {
    var locations = std.ArrayList(Point).init(allocator);
    try locations.append(trailhead);

    var score: i32 = 0;

    while (locations.items.len != 0) {
        const point = locations.pop();
        if (map.items[@intCast(point.y)].items[@intCast(point.x)] == 9) {
            score += 1;
        } else {
            const current_elevation = map.items[@intCast(point.y)].items[@intCast(point.x)];

            const directions = [_]Point{
                .{ .x = 0, .y = -1 },
                .{ .x = 0, .y = 1 },
                .{ .x = -1, .y = 0 },
                .{ .x = 1, .y = 0 },
            };
            for (directions) |dir| {
                const new_point = add(point, dir);

                if (new_point.x < 0 or new_point.x >= map.items[0].items.len or new_point.y < 0 or new_point.y >= map.items.len) {
                    continue;
                }

                const new_elevation = map.items[@intCast(new_point.y)].items[@intCast(new_point.x)];
                if (new_elevation - current_elevation == 1) {
                    try locations.append(new_point);
                }
            }
        }
    }
    return score;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day10");
    const map = try parse(allocator, input);
    defer map.deinit();

    var score: i32 = 0;
    var ratings: i32 = 0;

    for (map.items, 0..map.items.len) |row, i| {
        for (row.items, 0..row.items.len) |v, j| {
            if (v == 0) {
                score += try scoreTrailhead(Point{ .x = @intCast(j), .y = @intCast(i) }, map, allocator);
                ratings += try rateTrailhead(Point{ .x = @intCast(j), .y = @intCast(i) }, map, allocator);
            }
        }
    }

    std.debug.print("Part one {}\n", .{score});

    std.debug.print("Part two {}\n", .{ratings});
}

pub fn main() !void {
    try solve();
}
