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

const Reports = std.ArrayList(Levels);
const Levels = std.ArrayList(i32);

fn parseReports(allocator: std.mem.Allocator, input: []u8) !Reports {
    var lines = std.mem.split(u8, input, "\n");

    var reports = Reports.init(allocator);
    errdefer reports.deinit();

    while (lines.next()) |line| {
        if (line.len == 0) {
            continue;
        }

        var it = std.mem.splitSequence(u8, line, " ");
        var levels = Levels.init(allocator);

        while (it.next()) |v| {
            const v_parsed = try std.fmt.parseInt(i32, v, 10);
            try levels.append(v_parsed);
        }
        try reports.append(levels);
    }

    return reports;
}

fn isOrdered(report: Levels, ignore_i: usize, comptime orderFn: fn (void, lhs: i32, rhs: i32) bool) bool {
    for (0..report.items.len - 1) |i| {
        if (i == ignore_i) {
            continue;
        }

        var next_i = i + 1;
        if (i + 1 == ignore_i) {
            next_i = i + 2;
            if (next_i >= report.items.len) {
                continue;
            }
        }

        const diff = @abs(report.items[i] - report.items[next_i]);

        if (orderFn({}, report.items[i], report.items[next_i]) or !(0 < diff and diff < 4)) {
            return false;
        }
    }

    return true;
}
fn isIncreasing(report: Levels, ignore_i: usize) bool {
    return isOrdered(report, ignore_i, comptime std.sort.asc(i32));
}

fn isDecreasing(report: Levels, ignore_i: usize) bool {
    return isOrdered(report, ignore_i, comptime std.sort.desc(i32));
}

fn isSafe(report: Levels, ignore_i: usize) bool {
    return isIncreasing(report, ignore_i) or isDecreasing(report, ignore_i);
}

fn isSafeWithBad(report: Levels) bool {
    for (0..report.items.len) |i| {
        if (isSafe(report, i)) {
            return true;
        }
    }
    return false;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day2");
    const reports = try parseReports(allocator, input);
    defer reports.deinit();

    var safe_reports: i32 = 0;
    for (reports.items) |report| {
        if (isSafe(report, 9999999999999)) {
            safe_reports += 1;
        }
    }

    std.debug.print("Safe reports {}\n", .{safe_reports});

    var safe_reports_with_bad: i32 = 0;
    for (reports.items) |report| {
        if (isSafeWithBad(report)) {
            safe_reports_with_bad += 1;
        }
    }

    std.debug.print("Safe reports with bad levels {}\n", .{safe_reports_with_bad});
}

pub fn main() !void {
    try solve();
}
