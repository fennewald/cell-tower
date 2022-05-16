const std = @import("std");
const tree = @import("tree.zig");

const WIDTH = 7;
const HEIGHT = 12;

const Board = struct {
    cells: [WIDTH][HEIGHT]u8,
};

pub fn main() anyerror!void {
    const wordMap = try tree.build_dictionary_tree();
}
