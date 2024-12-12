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
const Antennas = std.AutoHashMap(u8, std.ArrayList(Point));

fn in_bounds(x: i32, y: i32, width: i32, height: i32) bool {
    return x >= 0 and x <= width and y >= 0 and y < height;
}

fn compute_antinodes(allocator: std.mem.Allocator, input: []const u8, with_resonance: bool) !u32 {
    var lines = std.mem.splitSequence(u8, input, "\n");

    var antennas = Antennas.init(allocator);
    defer antennas.deinit();

    // Last values of these are the bounds of the grid.
    var y: i32 = 0;
    var x: i32 = 0;

    while (lines.next()) |line| : (y += 1) {
        if (line.len == 0) {
            continue;
        }
        x = -1;
        for (line) |c| {
            x += 1;
            if (c == '.') {
                continue;
            }
            const entry = try antennas.getOrPut(c);
            if (!entry.found_existing) {
                entry.value_ptr.* = std.ArrayList(Point).init(allocator);
            }
            try entry.value_ptr.*.append(.{ .x = x, .y = y });
        }
    }

    var antinode_locations = std.AutoHashMap(Point, void).init(allocator);

    var ant_it = antennas.valueIterator();
    while (ant_it.next()) |antennas_ptrs| {
        // std.debug.print("Antenna {any}\n", .{antennas_ptrs.*.items});

        for (antennas_ptrs.*.items) |antenna| {
            for (antennas_ptrs.*.items) |antenna_2| {
                if (antenna.x == antenna_2.x and antenna.y == antenna_2.y) {
                    continue;
                }

                const dx: i32 = antenna.x - antenna_2.x;
                const dy: i32 = antenna.y - antenna_2.y;

                var antinode_x = antenna.x;
                var antinode_y = antenna.y;
                if (in_bounds(antinode_x + dx, antinode_y + dy, x, y)) {
                    try antinode_locations.put(.{ .x = antinode_x + dx, .y = antinode_y + dy }, {});
                }

                if (with_resonance) {
                    while (in_bounds(antinode_x, antinode_y, x, y)) : ({
                        antinode_x += dx;
                        antinode_y += dy;
                    }) {
                        try antinode_locations.put(.{ .x = antinode_x, .y = antinode_y }, {});
                    }
                }
            }
        }
    }

    return antinode_locations.count();
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day8");
    const n_antinodes = try compute_antinodes(allocator, input, false);

    std.debug.print("Part one {}\n", .{n_antinodes});

    const n_antinodes_with_resonance = try compute_antinodes(allocator, input, true);

    std.debug.print("Part two {}\n", .{n_antinodes_with_resonance});
}

pub fn main() !void {
    try solve();
}
