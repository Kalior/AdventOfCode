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

const Rules = std.AutoHashMap(i32, std.ArrayList(i32));

fn parse(allocator: std.mem.Allocator, input: []u8) !struct { rules: Rules, orders: std.ArrayList(std.ArrayList(i32)) } {
    var parts = std.mem.split(u8, input, "\n\n");
    var rules = std.mem.split(u8, parts.next().?, "\n");
    var orders = std.mem.split(u8, parts.next().?, "\n");

    var parsed_rules = Rules.init(allocator);
    errdefer parsed_rules.deinit();

    while (rules.next()) |line| {
        if (line.len == 0) {
            continue;
        }

        var vs = std.mem.split(u8, line, "|");
        const key = try std.fmt.parseInt(i32, vs.next().?, 10);
        const value = try std.fmt.parseInt(i32, vs.next().?, 10);

        const entry = try parsed_rules.getOrPut(key);
        if (!entry.found_existing) {
            entry.value_ptr.* = std.ArrayList(i32).init(allocator);
        }
        try entry.value_ptr.*.append(value);
    }

    var parsed_orders = std.ArrayList(std.ArrayList(i32)).init(allocator);
    errdefer parsed_orders.deinit();

    while (orders.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        var it = std.mem.splitSequence(u8, line, ",");
        var order = std.ArrayList(i32).init(allocator);

        while (it.next()) |v| {
            const v_parsed = try std.fmt.parseInt(i32, v, 10);
            try order.append(v_parsed);
        }
        try parsed_orders.append(order);
    }

    return .{ .rules = parsed_rules, .orders = parsed_orders };
}

fn violates_rules(rules: Rules, order: i32, i: usize, orders: std.ArrayList(i32)) bool {
    for (orders.items[i + 1 ..]) |coming_order| {
        if (rules.contains(coming_order)) {
            if (std.mem.containsAtLeast(i32, rules.get(coming_order).?.items, 1, &[_]i32{order})) {
                return true;
            }
        }
    }
    return false;
}

fn check_orders(rules: Rules, orders: std.ArrayList(i32)) bool {
    for (orders.items, 0..orders.items.len) |order, i| {
        if (violates_rules(rules, order, i, orders)) {
            return false;
        }
    }
    return true;
}

fn fix_orders(rules: Rules, orders: std.ArrayList(i32)) std.ArrayList(i32) {
    outer: while (!check_orders(rules, orders)) {
        for (orders.items, 0..orders.items.len) |order, i| {
            for (orders.items[i + 1 ..], i + 1..orders.items.len) |coming_order, j| {
                if (rules.contains(coming_order)) {
                    if (std.mem.containsAtLeast(i32, rules.get(coming_order).?.items, 1, &[_]i32{order})) {
                        orders.items[i] = coming_order;
                        orders.items[j] = order;
                        continue :outer;
                    }
                }
            }
        }
    }

    return orders;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day5");
    var rules_orders = try parse(allocator, input);
    defer rules_orders.rules.deinit();
    defer rules_orders.orders.deinit();

    var incorrect_orders = std.ArrayList(std.ArrayList(i32)).init(allocator);
    defer incorrect_orders.deinit();

    var midpoints: i32 = 0;
    for (rules_orders.orders.items) |orders| {
        if (check_orders(rules_orders.rules, orders)) {
            const middle_index = @as(usize, orders.items.len / 2);
            midpoints += orders.items[middle_index];
        } else {
            try incorrect_orders.append(orders);
        }
    }

    std.debug.print("Part one {}\n", .{midpoints});

    var fixed_midpoints: i32 = 0;
    for (incorrect_orders.items) |orders| {
        const fixed = fix_orders(rules_orders.rules, orders);

        const middle_index = @as(usize, fixed.items.len / 2);
        fixed_midpoints += fixed.items[middle_index];
    }

    std.debug.print("Part two {}\n", .{fixed_midpoints});
}

pub fn main() !void {
    try solve();
}
