const std = @import("std");
const print = @import("std").debug.print;

pub fn main() !void {
    print("Enter the string you want reversed: ", .{});
    const stdin = std.io.getStdIn().reader();
    var buf: [10]u8 = undefined;
    var input = stdin.readUntilDelimiterOrEof(buf[0..], '\n');
    print("{s}", .{input});
    for (buf) |_, i| {
        const idx = (buf.len - 1) - i;
        print("{c}", .{buf[idx]});
    }
}