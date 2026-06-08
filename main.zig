const std = @import("std");
const builtin = @import("builtin");
const Io = std.Io;
const posix = std.posix;

const Vec3 = extern struct { x: f64, y: f64, z: f64 };
const AzEl = extern struct { az: f64, alt: f64 };
const StarAzEl = extern struct { az: f64, alt: f64, mag: f32 };

const STAR_CATALOG_LEN: usize = 200;
const DEG2RAD: f64 = std.math.pi / 180.0;
const VMIN_IDX: usize = 6;
const VTIME_IDX: usize = 5;

extern fn sat_altaz(
    tle1: [*:0]const u8,
    tle2: [*:0]const u8,
    lat: f64,
    lon: f64,
    alt: f64,
    time: f64,
) AzEl;
extern fn star_positions(lat: f64, lon: f64, time: f64, out: [*]StarAzEl) usize;

const iss_tle1 = "1 25544U 98067A   26158.90128688  .00007994  00000+0  14961-3 0  9996";
const iss_tle2 = "2 25544  51.6338 346.0598 0006926 145.2709 214.8733 15.49660544570312";

const Color = enum(u8) {
    star_bright,
    star_mid,
    star_faint,
    horizon,
    satellite,
    hud,
    ground,
};

const Cell = struct {
    ch: u8 = ' ',
    color: Color = .hud,
    depth: f32 = -math.inf(f32),
};

const Camera = struct {
    yaw: f64 = 0,
    pitch: f64 = 0,

    fn viewDir(self: Camera) Vec3f {
        const cy = @cos(self.yaw);
        const sy = @sin(self.yaw);
        const cx = @cos(self.pitch);
        const sx = @sin(self.pitch);
        return normalize(.{
            .x = -sx * sy,
            .y = sx * cy,
            .z = cx,
        });
    }
};

const Vec3f = struct { x: f64, y: f64, z: f64 };

const math = struct {
    pub fn inf(comptime T: type) T {
        return std.math.inf(T);
    }
};

fn azAltToDir(az_deg: f64, alt_deg: f64) Vec3f {
    const az = az_deg * DEG2RAD;
    const alt = alt_deg * DEG2RAD;
    return .{
        .x = @cos(alt) * @sin(az),
        .y = @cos(alt) * @cos(az),
        .z = @sin(alt),
    };
}

fn starGlyph(mag: f32) u8 {
    if (mag < 1.0) return '*';
    if (mag < 2.5) return '+';
    if (mag < 4.0) return ',';
    return ':';
}

fn dot(a: Vec3f, b: Vec3f) f64 {
    return a.x * b.x + a.y * b.y + a.z * b.z;
}

fn cross(a: Vec3f, b: Vec3f) Vec3f {
    return .{
        .x = a.y * b.z - a.z * b.y,
        .y = a.z * b.x - a.x * b.z,
        .z = a.x * b.y - a.y * b.x,
    };
}

fn normalize(v: Vec3f) Vec3f {
    const len = @sqrt(dot(v, v));
    if (len < 1e-12) return .{ .x = 0, .y = 0, .z = 1 };
    return .{ .x = v.x / len, .y = v.y / len, .z = v.z / len };
}

fn project(
    dir: Vec3f,
    view: Vec3f,
    east: Vec3f,
    north: Vec3f,
    cx: i32,
    cy: i32,
    scale: f64,
) ?struct { x: i32, y: i32, depth: f32 } {
    const d = dot(dir, view);
    if (d < 0.05) return null;
    const px = dot(dir, east) / d;
    const py = dot(dir, north) / d;
    return .{
        .x = cx + @as(i32, @intFromFloat(px * scale)),
        .y = cy - @as(i32, @intFromFloat(py * scale)),
        .depth = @floatCast(d),
    };
}

fn starColor(mag: f32) Color {
    if (mag < 1.0) return .star_bright;
    if (mag < 3.0) return .star_mid;
    return .star_faint;
}

fn colorEscape(color: Color) []const u8 {
    return switch (color) {
        .star_bright => "\x1b[1;97m",
        .star_mid => "\x1b[37m",
        .star_faint => "\x1b[2;90m",
        .horizon => "\x1b[36m",
        .satellite => "\x1b[1;31m",
        .hud => "\x1b[90m",
        .ground => "\x1b[40m\x1b[90m",
    };
}

const Frame = struct {
    cols: usize,
    rows: usize,
    cells: []Cell,

    fn init(allocator: std.mem.Allocator, cols: usize, rows: usize) !Frame {
        const cells = try allocator.alloc(Cell, cols * rows);
        for (cells) |*cell| {
            cell.* = .{ .depth = -math.inf(f32) };
        }
        return .{ .cols = cols, .rows = rows, .cells = cells };
    }

    fn deinit(self: *Frame, allocator: std.mem.Allocator) void {
        allocator.free(self.cells);
    }

    fn clear(self: *Frame) void {
        for (self.cells) |*cell| {
            cell.* = .{ .depth = -math.inf(f32) };
        }
    }

    fn idx(self: Frame, x: i32, y: i32) ?usize {
        if (x < 0 or y < 0) return null;
        const ux: usize = @intCast(x);
        const uy: usize = @intCast(y);
        if (ux >= self.cols or uy >= self.rows) return null;
        return uy * self.cols + ux;
    }

    fn plot(self: *Frame, x: i32, y: i32, ch: u8, color: Color, depth: f32) void {
        const i = self.idx(x, y) orelse return;
        if (depth <= self.cells[i].depth) return;
        self.cells[i] = .{ .ch = ch, .color = color, .depth = depth };
    }
};

fn terminalSize(io: Io) struct { cols: u16, rows: u16 } {
    var size = posix.winsize{ .row = 24, .col = 80, .xpixel = 0, .ypixel = 0 };
    const file = Io.File.stdout();
    const err = (io.operate(.{ .device_io_control = .{
        .file = file,
        .code = posix.T.IOCGWINSZ,
        .arg = &size,
    } }) catch return .{ .cols = 80, .rows = 24 }).device_io_control;
    if (err < 0) return .{ .cols = 80, .rows = 24 };
    return .{ .cols = @max(size.col, 40), .rows = @max(size.row, 15) };
}

const RawTerminal = struct {
    fd: posix.fd_t,
    saved: posix.termios,

    fn enable() !RawTerminal {
        if (builtin.os.tag == .windows) return error.UnsupportedTerminal;
        const fd = Io.File.stdin().handle;
        const saved = try posix.tcgetattr(fd);
        var raw = saved;
        raw.lflag.ICANON = false;
        raw.lflag.ECHO = false;
        raw.lflag.ISIG = false;
        raw.cc[VMIN_IDX] = 0;
        raw.cc[VTIME_IDX] = 1;
        try posix.tcsetattr(fd, .NOW, raw);
        return .{ .fd = fd, .saved = saved };
    }

    fn restore(self: RawTerminal) void {
        posix.tcsetattr(self.fd, .NOW, self.saved) catch {};
    }
};

const Input = struct {
    buf: [16]u8 = undefined,
    len: usize = 0,

    fn feed(self: *Input, data: []const u8) void {
        const space = self.buf.len - self.len;
        const n = @min(space, data.len);
        @memcpy(self.buf[self.len..][0..n], data[0..n]);
        self.len += n;
    }

    fn poll(self: *Input, camera: *Camera, time_offset: *f64, quit: *bool) void {
        while (self.len > 0) {
            const b0 = self.buf[0];
            if (b0 == 0x1b and self.len >= 3 and self.buf[1] == '[') {
                switch (self.buf[2]) {
                    'A' => camera.pitch += 0.08,
                    'B' => camera.pitch -= 0.08,
                    'C' => camera.yaw -= 0.08,
                    'D' => camera.yaw += 0.08,
                    else => {},
                }
                self.shift(3);
                continue;
            }
            switch (b0) {
                'q', 'Q', 0x03 => quit.* = true,
                'a', 'A' => camera.yaw += 0.08,
                'd', 'D' => camera.yaw -= 0.08,
                'w', 'W' => camera.pitch += 0.08,
                's', 'S' => camera.pitch -= 0.08,
                ']' => time_offset.* += 300,
                '[' => time_offset.* -= 300,
                'r' => {
                    camera.yaw = 0;
                    camera.pitch = 0;
                    time_offset.* = 0;
                },
                else => {},
            }
            self.shift(1);
        }
    }

    fn shift(self: *Input, n: usize) void {
        if (n >= self.len) {
            self.len = 0;
            return;
        }
        std.mem.copyForwards(u8, self.buf[0..], self.buf[n..self.len]);
        self.len -= n;
    }
};

fn drawSky(
    frame: *Frame,
    camera: Camera,
    stars: []const StarAzEl,
    iss: AzEl,
    visible_stars: *usize,
) void {
    visible_stars.* = 0;
    const sky_rows: i32 = @intCast(frame.rows - 2);
    const cols: i32 = @intCast(frame.cols);
    const cx = @divTrunc(cols, 2);
    const cy = @divTrunc(sky_rows, 2);
    const scale = @as(f64, @floatFromInt(@min(cols, sky_rows * 2))) * 0.42;

    const view = camera.viewDir();
    const up_ref: Vec3f = if (@abs(view.z) < 0.9) .{ .x = 0, .y = 0, .z = 1 } else .{ .x = 0, .y = 1, .z = 0 };
    const east = normalize(cross(up_ref, view));
    const north = cross(view, east);

    var az: f64 = 0;
    while (az < 360.0) : (az += 6) {
        const p = azAltToDir(az, 0);
        if (project(p, view, east, north, cx, cy, scale)) |pt| {
            frame.plot(pt.x, pt.y, '-', .horizon, pt.depth);
        }
    }

    var alt_step: f64 = 15;
    while (alt_step < 90) : (alt_step += 15) {
        az = 0;
        while (az < 360.0) : (az += 8) {
            const p = azAltToDir(az, alt_step);
            if (project(p, view, east, north, cx, cy, scale)) |pt| {
                frame.plot(pt.x, pt.y, '.', .horizon, pt.depth - 0.001);
            }
        }
    }

    for (0..frame.cols) |x| {
        const row: i32 = @intCast(sky_rows);
        frame.plot(@intCast(x), row, '_', .ground, 0);
    }

    for (stars) |star| {
        if (star.alt <= 0) continue;
        visible_stars.* += 1;
        const p = azAltToDir(star.az, star.alt);
        if (project(p, view, east, north, cx, cy, scale)) |pt| {
            frame.plot(pt.x, pt.y, starGlyph(star.mag), starColor(star.mag), pt.depth + 1.0);
        }
    }

    if (!std.math.isNan(iss.az) and iss.alt > 0) {
        const p = azAltToDir(iss.az, iss.alt);
        if (project(p, view, east, north, cx, cy, scale)) |pt| {
            frame.plot(pt.x, pt.y, '@', .satellite, pt.depth + 0.01);
        }
    }

    const markers = [_]struct { az: f64, label: u8 }{
        .{ .az = 0, .label = 'N' },
        .{ .az = 90, .label = 'E' },
        .{ .az = 180, .label = 'S' },
        .{ .az = 270, .label = 'W' },
    };
    for (markers) |m| {
        const p = azAltToDir(m.az, 0);
        if (project(p, view, east, north, cx, cy, scale)) |pt| {
            frame.plot(pt.x, pt.y, m.label, .horizon, pt.depth + 0.001);
        }
    }
}

fn renderFrame(
    writer: *Io.Writer,
    frame: *Frame,
    hud: []const u8,
) !void {
    try writer.writeAll("\x1b[H");
    var prev_color: ?Color = null;
    var y: usize = 0;
    while (y < frame.rows) : (y += 1) {
        var x: usize = 0;
        while (x < frame.cols) : (x += 1) {
            const cell = frame.cells[y * frame.cols + x];
            if (prev_color != cell.color) {
                try writer.writeAll(colorEscape(cell.color));
                prev_color = cell.color;
            }
            try writer.writeByte(cell.ch);
        }
        if (y + 1 < frame.rows) try writer.writeByte('\n');
    }
    try writer.writeAll("\x1b[0m");
    try writer.writeAll(hud);
    try writer.flush();
}

pub fn main(init: std.process.Init) !void {
    if (builtin.os.tag == .windows) {
        std.debug.print("solarmap TUI requires a POSIX terminal (Linux/macOS).\n", .{});
        return;
    }

    const io = init.io;
    const gpa = init.gpa;

    const raw = RawTerminal.enable() catch |err| switch (err) {
        error.NotATerminal => {
            std.debug.print("solarmap needs an interactive terminal — run: zig build run\n", .{});
            return;
        },
        else => return err,
    };
    defer raw.restore();

    var stdout_buffer: [64 * 1024]u8 = undefined;
    var stdout_writer = Io.File.stdout().writer(io, &stdout_buffer);
    const stdout = &stdout_writer.interface;

    try stdout.writeAll("\x1b[?1049h\x1b[?25l");
    defer stdout.writeAll("\x1b[?1049l\x1b[?25h\x1b[0m") catch {};

    const lat: f64 = 48.8566;
    const lon: f64 = 2.3522;
    const alt_m: f64 = 35.0;

    var camera: Camera = .{};
    var time_offset: f64 = 0;
    var input: Input = .{};
    var quit = false;

    var stars: [STAR_CATALOG_LEN]StarAzEl = undefined;
    var hud_buf: [256]u8 = undefined;

    while (!quit) {
        const term = terminalSize(io);
        const cols: usize = term.cols;
        const rows: usize = term.rows;

        var frame = try Frame.init(gpa, cols, rows);
        defer frame.deinit(gpa);
        frame.clear();

        const now = Io.Timestamp.now(io, .real).toSeconds();
        const time: f64 = @as(f64, @floatFromInt(now)) + time_offset;

        _ = star_positions(lat, lon, time, &stars);
        const iss = sat_altaz(iss_tle1, iss_tle2, lat, lon, alt_m, time);

        var visible_stars: usize = 0;
        drawSky(&frame, camera, stars[0..STAR_CATALOG_LEN], iss, &visible_stars);

        const hud = try std.fmt.bufPrint(&hud_buf, "\x1b[{d};1H\x1b[0m\x1b[90msolar map  |  {d} stars up  |  WASD rotate  |  [/] time  |  r reset  |  q quit  |  yaw {d:.0} pitch {d:.0}\x1b[0m", .{
            rows,
            visible_stars,
            camera.yaw * 180.0 / std.math.pi,
            camera.pitch * 180.0 / std.math.pi,
        });

        try renderFrame(stdout, &frame, hud);

        var key_buf: [32]u8 = undefined;
        const n = Io.File.stdin().readStreaming(io, &.{&key_buf}) catch 0;
        if (n > 0) input.feed(key_buf[0..n]);
        input.poll(&camera, &time_offset, &quit);

        camera.pitch = std.math.clamp(camera.pitch, -1.45, 1.45);

        try Io.sleep(io, Io.Duration.fromMilliseconds(33), .awake);
    }
}
