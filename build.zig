const std = @import("std");
const builtin = @import("builtin");
const Build = std.Build;
const LazyPath = Build.LazyPath;

pub fn build(b: *Build) void {
    const target = b.standardTargetOptions(.{});
    const root_module = b.addModule("main", .{
        .root_source_file = LazyPath{ .src_path = .{ .owner = b, .sub_path = "main.zig" } },
        .target = target,
    });
    root_module.addLibraryPath(LazyPath{ .cwd_relative = "target/release" });
    root_module.linkSystemLibrary("astronomy_engine", .{});
    root_module.link_libc = true;
    if (builtin.os.tag == .linux) {
        root_module.linkSystemLibrary("gcc_s", .{});
        root_module.linkSystemLibrary("unwind", .{});
    }
    if (builtin.os.tag == .windows) {
        root_module.linkSystemLibrary("ws2_32", .{});
        root_module.linkSystemLibrary("userenv", .{});
    }

    const exe = b.addExecutable(.{
        .name = "solarmap",
        .root_module = root_module,
    });

    const cargo = b.addSystemCommand(&.{ "cargo", "build", "--release" });
    exe.step.dependOn(&cargo.step);

    b.installArtifact(exe);

    const run_cmd = b.addRunArtifact(exe);
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }
    const run_step = b.step("run", "Run solarmap");
    run_step.dependOn(&run_cmd.step);
}
