const std = @import("std");
const print = std.debug.print;
const Allocator = std.mem.Allocator;
const expect = std.testing.expect;
const dictionary = @import("dictionary.zig");

const Leaf = struct {
    n_children: u32 = 0,
    is_word: bool = false,
    children: [26]?*Leaf = [26]?*Leaf {
        null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null,
        null, null, null, null, null, null, null, null,
        null, null
    },

    fn init(self: *Leaf) void {
        self.n_children = 0;
        self.is_word = false;
        self.children = [26]?*Leaf {
            null, null, null, null, null, null, null, null,
            null, null, null, null, null, null, null, null,
            null, null, null, null, null, null, null, null,
            null, null
        };
    }

    fn add_word(self: *Leaf, allocator: *Allocator, word: [*:0]const u8) anyerror!void {
        const index = word[0] - 'a';
        self.n_children += 1;

        if (self.children[index] == null) {
            self.children[index] = try allocator.create(Leaf);
            self.children[index].?.init();
        }

        if (word[1] == 0) {
            self.children[index].?.is_word = true;
        } else {
            try self.children[index].?.add_word(allocator, word + 1);
        }
    }

    fn valid_prefix(self: *const Leaf, word: [*:0]const u8) bool {
        const index = word[0] - 'a';

        if (self.children[index] == null) {
            return false;
        } else if (word[1] == 0) {
            return true;
        } else {
            return self.children[index].?.valid_prefix(word + 1);
        }
    }

    fn has_word(self: *const Leaf, word: [*:0]const u8) bool {
        const index = word[0] - 'a';

        if (self.children[index] == null) {
            return false;
        } else if (word[1] == 0) {
            return self.children[index].?.is_word;
        } else {
            return self.children[index].?.has_word(word + 1);
        }
    }

    fn n_sub_words(self: *const Leaf, word: [*:0]const u8) u32 {
        const index = word[0] - 'a';

        if (self.children[index] == null) {
            return 0;
        } else if (word[1] == 0) {
            return self.children[index].?.n_children;
        } else {
            return self.children[index].?.n_sub_words(word + 1);
        }
    }
};

pub fn build_dictionary_tree(allocator: * Allocator) !*Leaf {
    var root: *Leaf = try allocator.create(Leaf);
    root.init();

    for (dictionary.DICTIONARY) |word| {
        try tree.add_word(allocator, word);
    }

    return root;
}

test "All words" {
    var arena = std.heap.ArenaAllocator.init(std.testing.allocator);
    defer arena.deinit();
    var allocator = &arena.allocator;

    var tree: Leaf = Leaf{};

    for (dictionary.DICTIONARY) |word| {
        print("Adding word {s}\n", .{word});
        try expect(tree.has_word(word) == false);
        try tree.add_word(allocator, word);
        try expect(tree.has_word(word));
    }

    for (dictionary.DICTIONARY) |word| {
        print("Has word {s}: {d} sub fields\n", .{word, tree.n_sub_words(word)});
        try expect(tree.has_word(word));
    }
}

