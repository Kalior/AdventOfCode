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

const Point = struct {
    x: i32,
    y: i32,
};

fn parse(allocator: std.mem.Allocator, input: []const u8) !struct { start_pos: Point, blocks: std.AutoHashMap(Point, bool) } {
    var lines = std.mem.split(u8, input, "\n");

    var blocks = std.AutoHashMap(Point, bool).init(allocator);
    errdefer blocks.deinit();

    var start_pos = Point{ .x = 0, .y = 0 };

    var line_i: i32 = 0;
    while (lines.next()) |line| : (line_i += 1) {
        if (line.len == 0) {
            continue;
        }
        for (line, 0..line.len) |c, i| {
            if (c == '#') {
                try blocks.put(.{ .x = @intCast(i), .y = line_i }, true);
            }
            if (c == '^') {
                start_pos = .{ .x = @intCast(i), .y = line_i };
            }
        }
    }

    return .{ .start_pos = start_pos, .blocks = blocks };
}

const Day6Errors = error{
    InvalidDirection,
};

fn rotate(direction: Point) !Point {
    if (direction.x == 0 and direction.y == -1) {
        return .{ .x = 1, .y = 0 };
    } else if (direction.x == 1 and direction.y == 0) {
        return .{ .x = 0, .y = 1 };
    } else if (direction.x == 0 and direction.y == 1) {
        return .{ .x = -1, .y = 0 };
    } else if (direction.x == -1 and direction.y == 0) {
        return .{ .x = 0, .y = -1 };
    }
    return Day6Errors.InvalidDirection;
}

fn move_around(allocator: std.mem.Allocator, start_position: Point, blocks: std.AutoHashMap(Point, bool)) !std.AutoHashMap(Point, bool) {
    var visited = std.AutoHashMap(Point, bool).init(allocator);
    errdefer visited.deinit();

    var min_bounds = Point{ .x = start_position.x, .y = start_position.y };
    var max_bounds = Point{ .x = start_position.x, .y = start_position.y };
    var keys = blocks.keyIterator();
    while (keys.next()) |block_ptr| {
        min_bounds.x = @min(min_bounds.x, block_ptr.*.x);
        min_bounds.y = @min(min_bounds.y, block_ptr.*.y);

        max_bounds.x = @max(max_bounds.x, block_ptr.*.x);
        max_bounds.y = @max(max_bounds.y, block_ptr.*.y);
    }

    var current_position = start_position;

    var direction = Point{ .x = 0, .y = -1 };

    while (current_position.x >= min_bounds.x and current_position.y >= min_bounds.y and current_position.x <= max_bounds.x and current_position.y <= max_bounds.y) {
        try visited.put(current_position, true);

        const next_position = .{ .x = current_position.x + direction.x, .y = current_position.y + direction.y };

        if (blocks.contains(next_position)) {
            direction = try rotate(direction);
            continue;
        } else {
            current_position = next_position;
        }
    }

    return visited;
}

fn check_all_repeating(allocator: std.mem.Allocator, start_position: Point, blocks: *std.AutoHashMap(Point, bool)) !i32 {
    var max_bounds = Point{ .x = start_position.x, .y = start_position.y };
    var keys = blocks.keyIterator();
    while (keys.next()) |block_ptr| {
        max_bounds.x = @max(max_bounds.x, block_ptr.*.x);
        max_bounds.y = @max(max_bounds.y, block_ptr.*.y);
    }

    const potential_positions = try move_around(allocator, start_position, blocks.*);

    var n_repeating: i32 = 0;
    var checked: i32 = 0;
    var it = potential_positions.keyIterator();
    while (it.next()) |key_ptr| {
        checked += 1;
        if (@rem(checked, 1000) == 0) {
            std.debug.print("checked: {}\n", .{checked});
        }
        const v = key_ptr.*;

        if (!(v.x == start_position.x and v.y == start_position.y) and !blocks.contains(v)) {
            try blocks.put(v, true);

            if (try check_repeating(allocator, start_position, blocks.*, .{ .x = max_bounds.x, .y = max_bounds.y })) {
                n_repeating += 1;
            }
            _ = blocks.remove(v);
        }
    }
    return n_repeating;
}

const PointDir = struct { x: i32, y: i32, dir_x: i32, dir_y: i32 };

fn check_repeating(allocator: std.mem.Allocator, start_position: Point, blocks: std.AutoHashMap(Point, bool), bounds: Point) !bool {
    var visited = std.AutoHashMap(PointDir, bool).init(allocator);
    defer visited.deinit();

    var current_position = start_position;

    var direction = Point{ .x = 0, .y = -1 };

    while (current_position.x >= 0 and current_position.y >= 0 and current_position.x <= bounds.x and current_position.y <= bounds.y) {
        const point_dir = PointDir{ .x = current_position.x, .y = current_position.y, .dir_x = direction.x, .dir_y = direction.y };
        if (visited.contains(point_dir)) {
            return true;
        }
        try visited.put(point_dir, true);

        const next_position = .{ .x = current_position.x + direction.x, .y = current_position.y + direction.y };

        if (blocks.contains(next_position)) {
            direction = try rotate(direction);
            continue;
        } else {
            current_position = next_position;
        }
    }

    return false;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day6");
    var state = try parse(allocator, input);
    defer state.blocks.deinit();

    const n_visited = (try move_around(allocator, state.start_pos, state.blocks)).count();

    std.debug.print("Part one {}\n", .{n_visited});

    const n_repeating = try check_all_repeating(allocator, state.start_pos, &state.blocks);

    std.debug.print("Part two {}\n", .{n_repeating});
}

pub fn main() !void {
    try solve();
}

test "repeating" {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    const input =
        \\....#.....
        \\.........#
        \\..........
        \\..#.......
        \\.......#..
        \\..........
        \\.#..^.....
        \\........#.
        \\#.........
        \\......#...
    ;
    var state = try parse(allocator, input);
    defer state.blocks.deinit();

    try state.blocks.put(.{ .x = 3, .y = 6 }, true);

    try std.testing.expect(try check_repeating(allocator, state.start_pos, state.blocks));
}
