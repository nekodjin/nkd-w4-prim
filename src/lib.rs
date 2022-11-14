// Platform Constants

/// The height and width of the screen, in pixels.
pub const SCREEN_SIZE: u32 = 160;

// MMIO Addresses

/// The color palette.
///
/// Each cell of the array represents one of the four colors in the palette.
/// Each color is encoded in the following manner:
/// <br>
/// `0bXXXXXXXX_RRRRRRRR_GGGGGGGG_BBBBBBBB`
/// <br>
/// Where `R` is the red channel, `G` is the green channel, `B` is the blue
/// channel, and `X` is unused.
pub const PALETTE: *mut [u32; 4] = 0x04 as _;

/// The drawing colors, used by all drawing functions.
///
/// There are four drawing colors, encoded in the following manner:
/// <br>
/// `0b44443333_22221111`
/// <br>
/// Where `1` is the first drawing color, `2` is the second drawing color, etc.
/// Each color may be a number from 1 to 4 to indicate the corresponding color
/// from the color palette, or 0 to indicate transparency.
pub const DRAW_COLORS: *mut u16 = 0x14 as _;

/// The four gamepads.
///
/// Each gamepad is encoded in the following manner:
/// <br>
/// `0bDURLxxZX`
/// <br>
/// Where `x` is unused, `X` represents the X button, `Z` the Z button, `L` the
/// Left button, `R` the Right button, `U` the Up button, and `D` the Down
/// button.
///
/// It is recommended to use the `GAMEPAD_*` constant bitmasks to mask off
/// buttons, rather than using magic numbers. See [`GAMEPAD_X`], [`GAMEPAD_Z`],
/// [`GAMEPAD_LEFT`], [`GAMEPAD_RIGHT`], [`GAMEPAD_UP`], and [`GAMEPAD_DOWN`].
pub const GAMEPADS: *const [u8; 4] = 0x16 as _;

/// The x position of the mouse.
pub const MOUSE_X: *const i16 = 0x1a as _;
/// The y position of the mouse.
pub const MOUSE_Y: *const i16 = 0x1c as _;

/// The state of the mouse buttons.
///
/// Mouse button state is encoded in the following manner:
/// <br>
/// `0bXXXXXMRL`
/// <br>
/// Where `X` is unused, `L` represents the left button, `R` the right button,
/// and `M` the middle button.
///
/// It is recommended to use the `MOUSE_*` constant bitmasks to mask off
/// buttons, rather than using magic numbers. See [`MOUSE_LEFT`],
/// [`MOUSE_RIGHT`], and [`MOUSE_MIDDLE`].
pub const MOUSE_BUTTONS: *const u8 = 0x1e as _;

/// Flags that modify the system behavior.
///
/// Flags are encoded in the following manner:
/// <br>
/// `0bXXXXXX10`
/// <br>
/// Where `X` is unused, `1` represents the `HIDE_GAMEPAD_OVERLAY` flag, and `0`
/// represents the `PRESERVE_FRAMEBUFFER` flag.
///
/// The `HIDE_GAMEPAD_OVERLAY` flag will disable the gamepad overlay on mobile.
///
/// The `PRESERVE_FRAMEBUFFER` flag will prevent the framebuffer from clearing
/// between frames.
///
/// It is recommended to use the `SYSTEM_*` constant bitmasks to mask off flags
/// rather than using magic numbers. See [`SYSTEM_HIDE_GAMEPAD_OVERLAY`] and
/// [`SYSTEM_PRESERVE_FRAMEBUFFER`].
pub const SYSTEM_FLAGS: *mut u8 = 0x1f as _;

/// The NetPlay multiplayer state.
///
/// NetPlay state is encoded in the following manner:
/// <br>
/// `0bXXXXXEII`
/// <br>
/// Where `X` is unused, `E` is set if NetPlay is currently active, and `I` is
/// a number from 0 to 3 representing the local player index (if NetPlay is
/// active).
pub const NETPLAY: *const u8 = 0x20 as _;

/// The framebuffer.
///
/// This array allows direct modification of the framebuffer. It is a packed
/// array of pixels, where each pixel is a 2-bit value representing one of the
/// four colors from the palette.
///
/// A contiguous segment of pixels in the framebuffer corresponds to a row of
/// pixels in the rendered image; in other words, FRAMEBUFFER should be indexed
/// using the desired y-coordinate, and the resultant array should be indexed
/// using a calculated x-coordinate (according to the 2-bit packing).
pub const FRAMEBUFFER: *mut [[u8; 40]; 160] = 0xa0 as _;

// Gamepad Bitmasks

/// Bitmask for the `GAMEPAD_X` value. See [`GAMEPADS`].
pub const GAMEPAD_X: u8 = 0b0000_0001;
/// Bitmask for the `GAMEPAD_Y` value. See [`GAMEPADS`].
pub const GAMEPAD_Z: u8 = 0b0000_0010;
/// Bitmask for the `GAMEPAD_LEFT` value. See [`GAMEPADS`].
pub const GAMEPAD_LEFT: u8 = 0b0001_0000;
/// Bitmask for the `GAMEPAD_RIGHT` value. See [`GAMEPADS`].
pub const GAMEPAD_RIGHT: u8 = 0b0010_0000;
/// Bitmask for the `GAMEPAD_UP` value. See [`GAMEPADS`].
pub const GAMEPAD_UP: u8 = 0b0100_0000;
/// Bitmask for the `GAMEPAD_DOWN` value. See [`GAMEPADS`].
pub const GAMEPAD_DOWN: u8 = 0b1000_0000;

// Mouse Bitmasks

/// Bitmask for the `MOUSE_LEFT` value. See [`MOUSE_BUTTONS`].
pub const MOUSE_LEFT: u8 = 1;
/// Bitmask for the `MOUSE_RIGHT` value. See [`MOUSE_BUTTONS`].
pub const MOUSE_RIGHT: u8 = 2;
/// Bitmask for the `MOUSE_MIDDLE` value. See [`MOUSE_BUTTONS`].
pub const MOUSE_MIDDLE: u8 = 4;

// System Flags Bitmasks

/// Bitmask for the `PRESERVE_FRAMEBUFFER` system flag. See [`SYSTEM_FLAGS`].
pub const SYSTEM_PRESERVE_FRAMEBUFFER: u8 = 0b0000_0001;
/// Bitmask for the `HIDE_GAMEPAD_OVERLAY` system flag. See [`SYSTEM_FLAGS`].
pub const SYSTEM_HIDE_GAMEPAD_OVERLAY: u8 = 0b0000_0010;

// Drawing Functions

extern "C" {
    /// Copies pixels from a sprite to the framebuffer.
    ///
    /// The `sprite` parameter is a pointer to the sprite, which may be encoded
    /// in either the [1BPP](https://wasm4.org/docs/guides/sprites#1bpp-format)
    /// format or the [2BPP](https://wasm4.org/docs/guides/sprites#2bpp-format)
    /// format.
    ///
    /// The `x` and `y` parameters are the coordinates at which to write the
    /// sprite into the framebuffer.
    ///
    /// The `width` and `height` parameters correspond to the width and height
    /// of the sprite.
    ///
    /// The `flags` parameter contains a set of flags, encoded in the following
    /// manner:
    /// <br>
    /// `0bXXXXXXXX_XXXXXXXX_XXXXXXXX_XXXXRVHF`
    /// <br>
    /// Where `X` is unused, `R` will rotate the sprite 90° counterclockwise,
    /// `V` will invert the sprite vertically, `H` will invert the sprite
    /// horizontally, and `F` denotes which format is used to encode the sprite
    /// (`0` for 1PBB and `1` for 2BPP).
    ///
    /// It is recommended to use the `BLIT_*` constant bitmasks to construct the
    /// flags argument rather than using magic numbers. See [`BLIT_1BPP`],
    /// [`BLIT_2BPP`], [`BLIT_FLIP_H`], [`BLIT_FLIP_V`], and [`BLIT_ROTATE`].
    #[link_name = "blit"]
    pub fn blit(
        sprite: *const u8,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        flags: u32,
    );

    /// Copies a pixels from a subregion of a sprite to the framebuffer.
    ///
    /// The `sprite` parameter is a pointer to the sprite, which may be encoded
    /// in either the [1BPP](https://wasm4.org/docs/guides/sprites#1bpp-format)
    /// format or the [2BPP](https://wasm4.org/docs/guides/sprites#2bpp-format)
    /// format.
    ///
    /// The `x` and `y` parameters are the coordinates at which to write the
    /// sprite into the framebuffer.
    ///
    /// The `width` and `height` parameters correspond to the width and height
    /// of the subregion of the sprite being drawn.
    ///
    /// The `src_x` and `src_y` parameters are the coordinates within the sprite
    /// at which the subregion begins.
    ///
    /// The `stride` parameter corresponds to the total width of the sprite,
    /// including both pixels within the subregion and pixels outside of it.
    ///
    /// The `flags` parameter contains a set of flags, encoded in the following
    /// manner:
    /// <br>
    /// `0bXXXXXXXX_XXXXXXXX_XXXXXXXX_XXXXRVHF`
    /// <br>
    /// Where `X` is unused, `R` will rotate the sprite 90° counterclockwise,
    /// `V` will invert the sprite vertically, `H` will invert the sprite
    /// horizontally, and `F` denotes which format is used to encode the sprite
    /// (`0` for 1PBB and `1` for 2BPP).
    ///
    /// It is recommended to use the `BLIT_*` constant bitmasks to construct the
    /// flags argument rather than using magic numbers. See [`BLIT_1BPP`],
    /// [`BLIT_2BPP`], [`BLIT_FLIP_H`], [`BLIT_FLIP_V`], and [`BLIT_ROTATE`].
    #[link_name = "blitSub"]
    pub fn blit_sub(
        sprite: *const u8,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
        src_x: u32,
        src_y: u32,
        stride: u32,
        flags: u32,
    );

    /// Draws a line between two points.
    ///
    /// The parameters `x1` and `y1` represent the coordinates of the first
    /// point.
    ///
    /// The parameters `x2` and `y2` represent the coordinates of the second
    /// point.
    ///
    /// Drawing color #1 will be used as the line color.
    #[link_name = "line"]
    pub fn line(x1: i32, y1: i32, x2: i32, y2: i32);

    /// Draws a vertical line.
    ///
    /// The parameters `x` and `y` represent the coordinates at which the line
    /// will begin. The line will go downwards `len` pixels.
    ///
    /// Drawing colors #1 will be used as the line color.
    ///
    /// `vline(x, y, len)` is equivalent to `line(x, y, x, y + len - 1)`.
    #[link_name = "vline"]
    pub fn vline(x: i32, y: i32, len: u32);

    /// Draws a horizontal line.
    ///
    /// The parameters `x` and `y` represent the coordinates at which the line
    /// will begin. The line will go rightwards `len` pixels.
    ///
    /// Drawing color #1 will be used as the line color.
    ///
    /// `hline(x, y, len)` is equivalent to `line(x, y, x + len - 1, y)`.
    #[link_name = "hline"]
    pub fn hline(x: i32, y: i32, len: u32);

    /// Draws an oval (or a circle).
    ///
    /// The parameters `x` and `y` represent the coordinates of the center of
    /// the oval or circle. The parameters `width` and `height` represent the
    /// width and height of the circle.
    ///
    /// Drawing color #1 will be used as the fill color, and drawing color #2
    /// will be used as the outline color.
    #[link_name = "oval"]
    pub fn oval(x: i32, y: i32, width: u32, height: u32);

    /// Draws a rectangle.
    ///
    /// The parameters `x` and `y` represent the coordinates of the top-left
    /// corner of the rectangle. The parameters `width` and `heigh` represent
    /// the width and height of the rectangle. Drawing color #1 will be used as
    /// the fill color, and drawing color #2 will be used as the outline color.
    #[link_name = "rect"]
    pub fn rect(x: i32, y: i32, width: u32, height: u32);

    /// Draws text using the built-in system font.
    ///
    /// The `text` parameter should be a pointer to the text being written. The
    /// `length` prameter should be the length of that text.
    ///
    /// The `x` and `y` parameters represent the coordinates at which the first
    /// character of the string will be written. Specifically, the top-left
    /// corner of that character will be written to those coordinates.
    ///
    /// Text should be encoded using ASCII. No nul bytes should be present. The
    /// string may contain line breaks.
    ///
    /// The characters of the built-in font are each 8x8 pixels in size.
    #[link_name = "textUtf8"]
    pub fn text(text: *const u8, length: usize, x: i32, y: i32);
}

// Blit Flag Bitmasks

/// Bitmask for the 1BPP sprite encoding. Should not be used with the
/// [`BLIT_2BPP`] bitmask. See [`blit`] and [`blit_sub`].
pub const BLIT_1BPP: u32 = 0b0000;
/// Bitmask for the 2BPP sprite encoding. Should not be used with the
/// [`BLIT_1BPP`] bitmask. See [`blit`] and [`blit_sub`].
pub const BLIT_2BPP: u32 = 0b0001;
/// Bitmask for flipping a sprite horizontally. See [`blit`] and [`blit_sub`].
pub const BLIT_FLIP_H: u32 = 0b0010;
/// Bitmask for flipping a sprite vertically. See [`blit`] and [`blit_sub`].
pub const BLIT_FLIP_V: u32 = 0b0100;
/// Bitmask for rotating a sprite 90° counterclockwise. See [`blit`] and
/// [`blit_sub`].
pub const BLIT_ROTATE: u32 = 0b1000;

// Sound Funtions

extern "C" {
    /// Plays a sound tone.
    ///
    /// The `frequency` parameter represents the frequency of the tone. It is
    /// encoded in the following manner:
    /// <br>
    /// `0bEEEEEEEE_EEEEEEEE_SSSSSSSS_SSSSSSSS`
    /// <br>
    /// Where `S` is a 16-bit unsigned integer representing the start frequency
    /// of the tone in Hz, and `E` is a 16-bit unsigned integer representing the
    /// end frequency of the tone in Hz. If the end frequency is non-zero, then
    /// the pitch of the tone will change linearly across the duration of the
    /// tone, beginning at the start frequency and ending at the end frequency.
    /// Otherwise, if the end frequency is zero, the tone will remain constant
    /// at the start frequency for its full duration.
    ///
    /// The `duration` parameter represents the duration of the tone. It is
    /// encoded in the following manner:
    /// <br>
    /// `0bAAAAAAAA_DDDDDDDD_SSSSSSSS_RRRRRRRR`
    /// <br>
    /// This encoding represents a standard
    /// [ADSR Envelope](https://en.wikipedia.org/wiki/Envelope_(music)#ADSR)
    /// envelope. Each field shall be interpreted as an 8-bit unsigned integer,
    /// and represents a duration in 60th's of a second.
    ///
    /// The `volume` parameter represents the volume of the tone. It is encoded
    /// in the following manner:
    /// <br>
    /// `0bAAAAAAAA_AAAAAAAA_SSSSSSSS_SSSSSSSS`
    /// <br>
    /// Where `A` represents the Attack volume, and `S` represents the Sustain
    /// volume (see the description for the `duration` parameter). The volume of
    /// the tone will begin at zero, rise to the attack volume across the
    /// duration of the attack, lower to the sustain volume across the duration
    /// of the decay, remains constant across the duration of the sustain, and
    /// then falls back to zero across the duration of the release. If the
    /// attack volume is 0, it will default to 100. Both volumes are represented
    /// as 16-bit unsigned integers, and may hold values between 0 and 100,
    /// where 0 represents silence and 100 represents the greatest possible
    /// volume.
    ///
    /// The `flags` parameter modifies the behavior of the tone. It is encoded
    /// in the following manner:
    /// <br>
    /// `0bXXPPMMCC`
    /// <br>
    /// Where `X` is unused, `P` represents the pan, `M` represents the mode,
    /// and `C` represents the channel.
    ///
    /// The pan may be: `0` for center-pan, `1` for left-pan, or `2` for
    /// right-pan. `3` is not valid pan value.
    ///
    /// The mode affects pulse channels. For pulse channels, it sets the pulse
    /// wave duty cycle according to the following relations: `0` = 12.5%, `1` =
    /// 25%, `2` = 50%, `3` = 75%.
    ///
    /// The channel affects the waveform of the tone. WASM-4 provides four
    /// distinct waveforms: two pulse waves (square waves), one triangle wave,
    /// and one "noise" wave. Channels `0` and `1` represent the two pulse
    /// waves, while channel `2` represents the triangle wave and channel `3`
    /// represents the "noise" wave.
    ///
    /// It is recommended to use the `TONE_*` constant bitmasks to construct
    /// the `flags` argument, rather than using magic numbers. See
    /// [`TONE_PULSE1`], [`TONE_PULSE2`], [`TONE_TRIANGLE`], [`TONE_NOISE`],
    /// [`TONE_DC_12_5`], [`TONE_DC_25`], [`TONE_DC_50`], [`TONE_DC_75`],
    /// [`TONE_PAN_CENTER`], [`TONE_PAN_LEFT`], and [`TONE_PAN_RIGHT`].
    #[link_name = "tone"]
    pub fn tone(frequency: u32, duration: u32, volume: u32, flags: u32);
}

/// Bitmask for the Pulse1 waveform. Should not be used with other waveform
/// bitmasks. See [`tone`].
pub const TONE_PULSE1: u32 = 0b000000;
/// Bitmask for the Pulse2 waveform. Should not be used with other waveform
/// bitmasks. See [`tone`].
pub const TONE_PULSE2: u32 = 0b000001;
/// Bitmask for the Triangle waveform. Should not be used with other waveform
/// bitmasks. See [`tone`].
pub const TONE_TRIANGLE: u32 = 0b000010;
/// Bitmask for the Noise waveform. Should not be used with other waveform
/// bitmasks. See [`tone`].
pub const TONE_NOISE: u32 = 0b000011;
/// Bitmask for the 12.5% pulse wave duty cycle. Should only be used with Pulse1
/// or Pulse2 waveforms. See [`tone`].
#[doc(alias = "Mode1")]
pub const TONE_DC_12_5: u32 = 0b000000;
/// Bitmask for the 25% pulse wave duty cycle. Should only be used with Pulse1
/// or Pulse2 waveforms. Should not be used with other duty cycle bitmasks. See
/// [`tone`].
#[doc(alias = "Mode2")]
pub const TONE_DC_25: u32 = 0b000100;
/// Bitmask for the 50% pulse wave duty cycle. Should only be used with Pulse1
/// or Pulse2 waveforms. Should not be used with other duty cycle bitmasks. See
/// [`tone`].
#[doc(alias = "Mode3")]
pub const TONE_DC_50: u32 = 0b001000;
/// Bitmask for the 75% pulse wave duty cycle. Should only be used with Pulse1
/// or Pulse2 waveforms. Should not be used with other duty cycle bitmasks. See
/// [`tone`].
#[doc(alias = "Mode4")]
pub const TONE_DC_75: u32 = 0b001100;
/// Bitmask for center-pan. Should not be used with other pan bitmasks. See
/// [`tone`].
pub const TONE_PAN_CENTER: u32 = 0b000000;
/// Bitmask for left-pan. Should not be used with other pan bitmasks. See
/// [`tone`].
pub const TONE_PAN_LEFT: u32 = 0b010000;
/// Bitmask for right-pan. Should not be used with other pan bitmasks. See
/// [`tone`].
pub const TONE_PAN_RIGHT: u32 = 0b100000;

// Storage Functions.

extern "C" {
    /// Reads data from persistent storage.
    ///
    /// The `dest` argument should be a pointer to a byte buffer where the data
    /// read from persistent memory will be written.
    ///
    /// The `size` argument indicates the maximum number of bytes that should be
    /// read.
    ///
    /// `diskr` will return the number of bytes that have been written to the
    /// provided buffer. This number may be less than `size`.
    ///
    /// In the interest of memory safety, the `dest` pointer should have
    /// provenance over the slice delineated by
    /// `slice::<u8>::from_raw_parts_mut(dest, size)`.
    ///
    /// WASM-4 does not support storage of more than 1024 bytes. Therefore,
    /// you should never expect to successfully more than 1024 bytes.
    pub fn diskr(dest: *mut u8, size: u32) -> u32;

    /// Writes data to persistent storage.
    ///
    /// The `src` argument should be a pointer to a byte buffer containing the
    /// data to be written.
    ///
    /// `diskw` will return the number of bytes that have been written from the
    /// provided buffer. This number may be less than `size`.
    ///
    /// In the interest of memory safety, the `std` pointer should have
    /// provenance over the slice delineated by
    /// `slice::<u8>::from_raw_parts(src, size)`.
    ///
    /// WASM-4 does not support storage of more than 1024 bytes. Therefore,
    /// you should never expect to successfully store more than 1024 bytes.
    pub fn diskw(src: *const u8, size: u32) -> u32;
}

// Other Functions

extern "C" {
    /// Prints a message to the debug console.
    ///
    /// The `string` parameter should be a pointer to the message, and the
    /// `length` parameter should be the length of the message in bytes. The
    /// string should be ASCII text with no nul bytes.
    #[link_name = "traceUtf8"]
    pub fn trace(string: *const u8, length: usize);
}
