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

pub fn partOne(input: []u8) !usize {
    var checksum: u64 = 0;

    var i: usize = 0;
    var position: usize = 0;

    var end_i = input.len - 1;
    var remaining_last_values: usize = @intCast(try std.fmt.parseInt(i32, &[_]u8{input[end_i]}, 10));

    while (i <= input.len and i < end_i) : (i += 2) {
        const allocated_space: usize = @intCast(try std.fmt.parseInt(i32, &[_]u8{input[i]}, 10));
        const free_space: usize = @intCast(try std.fmt.parseInt(i32, &[_]u8{input[i + 1]}, 10));

        // Add checksum from current values
        for (position..position + allocated_space) |j| {
            checksum += j * i / 2;
        }
        position += allocated_space;

        // Add checksum from current values
        for (position..position + free_space) |j| {
            if (remaining_last_values == 0) {
                end_i -= 2;
                remaining_last_values = @intCast(try std.fmt.parseInt(i32, &[_]u8{input[end_i]}, 10));
            }
            if (end_i <= i) {
                break;
            }

            checksum += j * end_i / 2;
            remaining_last_values -= 1;
        }
        position += free_space;
    }
    for (0..remaining_last_values) |j| {
        if (end_i < i) {
            break;
        }
        checksum += (position + j) * end_i / 2;
    }
    return checksum;
}

pub fn partTwo(allocator: std.mem.Allocator, input: []u8) !usize {
    var remaining_free_space = std.ArrayList(usize).init(allocator);
    errdefer remaining_free_space.deinit();

    for (0..input.len - 1) |i| {
        // std.debug.print("{any}", .{input[i]});
        if (i % 2 == 1) {
            try remaining_free_space.append(@intCast(try std.fmt.parseInt(i32, &[_]u8{input[i]}, 10)));
        }
    }
    var checksum: u64 = 0;

    var end_i = input.len - 1;

    outer: while (end_i > 0) : (end_i -= 2) {
        const block_values: usize = @intCast(try std.fmt.parseInt(i32, &[_]u8{input[end_i]}, 10));

        var position: usize = 0;

        for (0..end_i + 1) |block_i| {
            if (block_i == end_i) {
                for (position..position + block_values) |j| {
                    checksum += j * end_i / 2;
                }
            } else {
                const n_values: usize = @intCast(try std.fmt.parseInt(i32, &[_]u8{input[block_i]}, 10));

                if (block_i % 2 == 0) {
                    position += n_values;
                    continue;
                }

                const free_values = remaining_free_space.items[block_i / 2];
                // Add position of any previously pushed values.
                position += (n_values - free_values);
                if (free_values >= block_values) {
                    for (position..position + block_values) |j| {
                        checksum += j * end_i / 2;
                    }
                    remaining_free_space.items[block_i / 2] -= block_values;
                    continue :outer;
                }

                position += free_values;
            }
        }
    }

    return checksum;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day9");
    const checksum = try partOne(input);

    std.debug.print("Part one {}\n", .{checksum});

    const checksum_2 = try partTwo(allocator, input);

    std.debug.print("Part two {}\n", .{checksum_2});
}

pub fn main() !void {
    try solve();
}
