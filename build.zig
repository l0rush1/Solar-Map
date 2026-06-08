const std = @import("std");
const builtin = @import("builtin");
const Build = std.Build;
const LazyPath = Build.LazyPath;
const process = std.process;
fn runCargo(allocator: std.mem.Allocator) !void {
    var child = process.Child.init(&.{ "cargo", "build", "--release" }, allocator);
    child.stdin_behavior = .Inherit;
    child.stdout_behavior = .Inherit;
    child.stderr_behavior = .Inherit;
    try child.spawn();
    _ = try child.wait();
}
pub fn build(b: *Build) void {
    runCargo(b.allocator) catch |err| std.debug.panic("cargo build failed: {}\n", .{err});
    const target = b.standardTargetOptions(.{});
    const root_module = b.addModule("main", .{
        .root_source_file = LazyPath{ .src_path = .{ .owner = b, .sub_path = "main.zig" } },
        .target = target,
    });
    const exe = b.addExecutable(.{
        .name = "solarmap",
        .root_module = root_module,
    });
    exe.addLibraryPath(LazyPath{ .cwd_relative = "target/release" });
    exe.linkSystemLibrary("astronomy_engine");
    exe.linkLibC();
    if (builtin.os.tag == .windows) {
        exe.linkSystemLibrary("ws2_32");
        exe.linkSystemLibrary("userenv");
    }
    b.installArtifact(exe);
}
