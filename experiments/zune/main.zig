const std = @import("std");

const TokenKind = enum {
  identifier,
  number,
  binaryOperator,
  eof,
};

const Token = struct {
  kind: TokenKind,
  value: []const u8,
};

pub fn main() void {
  const tokens = [_]Token{
    Token{.kind = TokenKind.number, .value="1"},
    Token{.kind = TokenKind.binaryOperator, .value="+"},
    Token{.kind = TokenKind.number, .value="2"},
  };

  for (tokens) |e| {
    switch (e.kind) {
      .number => std.debug.print("number", .{}),
      .identifier => std.debug.print("identifier", .{}),
      .binaryOperator => std.debug.print("binaryOperator", .{}),
      .eof => std.debug.print("eof", .{}),
    }
    std.debug.print("\n", .{});
  }

  std.debug.print("hello {any}", .{tokens});
}