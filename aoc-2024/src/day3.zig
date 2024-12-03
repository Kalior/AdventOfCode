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

const ParseError = error{
    NotMulError,
    NotDoError,
    NotDontError,
};

const MulArgs = struct { left: i32, right: i32 };

fn valid_mul_at(memory: []const u8, start_i: usize) !MulArgs {
    if (!std.mem.eql(u8, memory[start_i .. start_i + 3], "mul")) {
        return ParseError.NotMulError;
    }

    if (memory[start_i + 3] != '(') {
        return ParseError.NotMulError;
    }

    var next_comma_i: usize = start_i + 4;

    while (next_comma_i < memory.len and memory[next_comma_i] != ',') : (next_comma_i += 1) {}

    if (memory[next_comma_i] != ',') {
        return ParseError.NotMulError;
    }

    const left = std.fmt.parseInt(i32, memory[start_i + 4 .. next_comma_i], 10) catch return ParseError.NotMulError;

    var next_parentheses_i: usize = next_comma_i + 1;
    while (next_parentheses_i < memory.len and memory[next_parentheses_i] != ')') : (next_parentheses_i += 1) {}

    if (memory[next_parentheses_i] != ')') {
        return ParseError.NotMulError;
    }

    const right = std.fmt.parseInt(i32, memory[next_comma_i + 1 .. next_parentheses_i], 10) catch return ParseError.NotMulError;

    return .{ .left = left, .right = right };
}

fn valid_do_at(memory: []const u8, start_i: usize) !bool {
    if (!std.mem.eql(u8, memory[start_i .. start_i + 4], "do()")) {
        return ParseError.NotDoError;
    }
    return true;
}

fn valid_dont_at(memory: []const u8, start_i: usize) !bool {
    if (start_i + 7 > memory.len) {
        return ParseError.NotDontError;
    }
    if (!std.mem.eql(u8, memory[start_i .. start_i + 7], "don't()")) {
        return ParseError.NotDontError;
    }
    return true;
}

fn apply(memory: []const u8) !i32 {
    var sum: i32 = 0;
    var i: usize = 0;

    while (i < memory.len - 5) : (i += 1) {
        const args = valid_mul_at(memory, i) catch MulArgs{ .left = 0, .right = 0 };
        sum += args.left * args.right;
    }
    return sum;
}

fn applyWithConditionals(memory: []const u8) !i32 {
    var sum: i32 = 0;
    var i: usize = 0;

    var mul_active = true;

    while (i < memory.len - 5) : (i += 1) {
        if (mul_active) {
            const args = valid_mul_at(memory, i) catch MulArgs{ .left = 0, .right = 0 };
            sum += args.left * args.right;
        }

        if (valid_do_at(memory, i) catch false) {
            mul_active = true;
        }
        if (valid_dont_at(memory, i) catch false) {
            mul_active = false;
        }
    }
    return sum;
}

pub fn solve() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try readFile(allocator, "inputs/day3");
    const sum = try apply(input);

    std.debug.print("Part one {}\n", .{sum});

    const sum_with_conditionals = try applyWithConditionals(input);

    std.debug.print("Part two {}\n", .{sum_with_conditionals});
}

pub fn main() !void {
    try solve();
}

test "simple example" {
    const test_case = try valid_mul_at("mul(5,7)", 0);
    try std.testing.expect(test_case.left == 5 and test_case.right == 7);
}

test "case one" {
    const test_case = try valid_mul_at("mul(2,4)", 0);
    try std.testing.expect(test_case.left == 2 and test_case.right == 4);
}

test "case two" {
    const test_case = try valid_mul_at("mul(5,5)", 0);
    try std.testing.expect(test_case.left == 5 and test_case.right == 5);
}

test "case three" {
    const test_case = try valid_mul_at("mul(11,8)", 0);
    try std.testing.expect(test_case.left == 11 and test_case.right == 8);
}

test "case four" {
    const test_case = try valid_mul_at("mul(8,5)", 0);
    try std.testing.expect(test_case.left == 8 and test_case.right == 5);
}

test "test errors" {
    try std.testing.expectError(ParseError.NotMulError, valid_mul_at("mul(5,7)", 2));
    try std.testing.expectError(ParseError.NotMulError, valid_mul_at("mul(5 ,7)", 0));
    try std.testing.expectError(ParseError.NotMulError, valid_mul_at("xmul(5 ,7)", 0));
}

test "case do" {
    const test_case = try valid_do_at("do()", 0);
    try std.testing.expect(test_case);
    try std.testing.expectError(ParseError.NotDoError, valid_do_at("asds", 0));
}

test "case dont" {
    const test_case = try valid_dont_at("don't()", 0);
    try std.testing.expect(test_case);

    try std.testing.expectError(ParseError.NotDontError, valid_dont_at("don't((", 0));
}
