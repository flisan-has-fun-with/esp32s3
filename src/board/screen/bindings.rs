/* automatically generated by rust-bindgen 0.71.1 */

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8).offset(byte_index as isize);
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            byte | mask
        } else {
            byte & !mask
        }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte =
            (core::ptr::addr_of_mut!((*this).storage) as *mut u8).offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
    }
}
pub const CONFIG_ESP_MAIN_TASK_STACK_SIZE: u32 = 32768;
pub const CONFIG_FREERTOS_MAX_TASK_NAME_LEN: u32 = 16;
pub const CONFIG_FREERTOS_TASK_NOTIFICATION_ARRAY_ENTRIES: u32 = 1;
pub const __bool_true_false_are_defined: u32 = 1;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const EPD_WIDTH: u32 = 960;
pub const EPD_HEIGHT: u32 = 540;
pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_fast32_t = i32;
pub type uint_fast32_t = u32;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_fast16_t = i16;
pub type uint_fast16_t = u16;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[doc = " @brief An area on the display."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rect_t {
    pub x: i32,
    #[doc = " Horizontal position."]
    pub y: i32,
    #[doc = " Vertical position."]
    pub width: i32,
    #[doc = " Area / image width, must be positive."]
    pub height: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Rect_t"][::std::mem::size_of::<Rect_t>() - 16usize];
    ["Alignment of Rect_t"][::std::mem::align_of::<Rect_t>() - 4usize];
    ["Offset of field: Rect_t::x"][::std::mem::offset_of!(Rect_t, x) - 0usize];
    ["Offset of field: Rect_t::y"][::std::mem::offset_of!(Rect_t, y) - 4usize];
    ["Offset of field: Rect_t::width"][::std::mem::offset_of!(Rect_t, width) - 8usize];
    ["Offset of field: Rect_t::height"][::std::mem::offset_of!(Rect_t, height) - 12usize];
};
pub const DrawMode_t_BLACK_ON_WHITE: DrawMode_t = 1;
#[doc = " Draw black / grayscale image on a white display."]
pub const DrawMode_t_WHITE_ON_WHITE: DrawMode_t = 2;
#[doc = " \"Draw with white ink\" on a white display."]
pub const DrawMode_t_WHITE_ON_BLACK: DrawMode_t = 4;
#[doc = " @brief The image drawing mode."]
pub type DrawMode_t = ::std::os::raw::c_uint;
pub const DrawFlags_DRAW_BACKGROUND: DrawFlags = 1;
#[doc = " @brief Font drawing flags."]
pub type DrawFlags = ::std::os::raw::c_uint;
#[doc = " @brief Font properties."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FontProperties {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    #[doc = " Background color"]
    pub fallback_glyph: u32,
    #[doc = " Use the glyph for this codepoint for missing glyphs."]
    pub flags: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of FontProperties"][::std::mem::size_of::<FontProperties>() - 12usize];
    ["Alignment of FontProperties"][::std::mem::align_of::<FontProperties>() - 4usize];
    ["Offset of field: FontProperties::fallback_glyph"]
        [::std::mem::offset_of!(FontProperties, fallback_glyph) - 4usize];
    ["Offset of field: FontProperties::flags"]
        [::std::mem::offset_of!(FontProperties, flags) - 8usize];
};
impl FontProperties {
    #[inline]
    pub fn fg_color(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_fg_color(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn fg_color_raw(this: *const Self) -> u8 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                0usize,
                4u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_fg_color_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn bg_color(&self) -> u8 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u8) }
    }
    #[inline]
    pub fn set_bg_color(&mut self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn bg_color_raw(this: *const Self) -> u8 {
        unsafe {
            ::std::mem::transmute(<__BindgenBitfieldUnit<[u8; 1usize]>>::raw_get(
                ::std::ptr::addr_of!((*this)._bitfield_1),
                4usize,
                4u8,
            ) as u8)
        }
    }
    #[inline]
    pub unsafe fn set_bg_color_raw(this: *mut Self, val: u8) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<[u8; 1usize]>>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                4usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(fg_color: u8, bg_color: u8) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let fg_color: u8 = unsafe { ::std::mem::transmute(fg_color) };
            fg_color as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let bg_color: u8 = unsafe { ::std::mem::transmute(bg_color) };
            bg_color as u64
        });
        __bindgen_bitfield_unit
    }
}
unsafe extern "C" {
    #[doc = " @brief Initialize the ePaper display"]
    pub fn epd_init();
}
unsafe extern "C" {
    #[doc = " @brief Enable display power supply."]
    pub fn epd_poweron();
}
unsafe extern "C" {
    #[doc = " @brief Disable display power supply."]
    pub fn epd_poweroff();
}
unsafe extern "C" {
    #[doc = " @brief Clear the whole screen by flashing it."]
    pub fn epd_clear();
}
unsafe extern "C" {
    pub fn epd_poweroff_all();
}
unsafe extern "C" {
    #[doc = " @brief Clear an area by flashing it.\n\n @param area The area to clear."]
    pub fn epd_clear_area(area: Rect_t);
}
unsafe extern "C" {
    #[doc = " @brief Clear an area by flashing it.\n\n @param area       The area to clear.\n @param cycles     The number of black-to-white clear cycles.\n @param cycle_time Length of a cycle. Default: 50 (us)."]
    pub fn epd_clear_area_cycles(area: Rect_t, cycles: i32, cycle_time: i32);
}
unsafe extern "C" {
    #[doc = " @brief Darken / lighten an area for a given time.\n\n @param area  The area to darken / lighten.\n @param time  The time in us to apply voltage to each pixel.\n @param color 1: lighten, 0: darken."]
    pub fn epd_push_pixels(area: Rect_t, time: i16, color: i32);
}
unsafe extern "C" {
    #[doc = " @brief Draw a picture to a given area. The image area is not cleared and\n        assumed to be white before drawing.\n\n @param area The display area to draw to. `width` and `height` of the area\n             must correspond to the image dimensions in pixels.\n @param data The image data, as a buffer of 4 bit wide brightness values.\n             Pixel data is packed (two pixels per byte). A byte cannot wrap\n             over multiple rows, images of uneven width must add a padding\n             nibble per line."]
    pub fn epd_draw_grayscale_image(area: Rect_t, data: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a picture to a given area, with some draw mode.\n\n @note The image area is not cleared before drawing. For example, this can be\n       used for pixel-aligned clearing.\n\n @param area The display area to draw to. `width` and `height` of the area\n             must correspond to the image dimensions in pixels.\n @param data The image data, as a buffer of 4 bit wide brightness values.\n             Pixel data is packed (two pixels per byte). A byte cannot wrap\n             over multiple rows, images of uneven width must add a padding\n             nibble per line."]
    pub fn epd_draw_image(area: Rect_t, data: *mut u8, mode: DrawMode_t);
}
unsafe extern "C" {
    pub fn epd_draw_frame_1bit(area: Rect_t, ptr: *mut u8, mode: DrawMode_t, time: i32);
}
unsafe extern "C" {
    #[doc = " @brief Rectancle representing the whole screen area."]
    pub fn epd_full_screen() -> Rect_t;
}
unsafe extern "C" {
    #[doc = " @brief Draw a picture to a given framebuffer.\n\n @param image_area  The area to copy to. `width` and `height` of the area must\n                    correspond to the image dimensions in pixels.\n @param image_data  The image data, as a buffer of 4 bit wide brightness values.\n                    Pixel data is packed (two pixels per byte). A byte cannot\n                    wrap over multiple rows, images of uneven width must add a\n                    padding nibble per line.\n @param framebuffer The framebuffer object, which must\n                    be `EPD_WIDTH / 2 * EPD_HEIGHT` large."]
    pub fn epd_copy_to_framebuffer(image_area: Rect_t, image_data: *mut u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a pixel a given framebuffer.\n\n @param x           Horizontal position in pixels.\n @param y           Vertical position in pixels.\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to."]
    pub fn epd_draw_pixel(x: i32, y: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a horizontal line to a given framebuffer.\n\n @param x           Horizontal start position in pixels.\n @param y           Vertical start position in pixels.\n @param length      Length of the line in pixels.\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to, which must\n                    be `EPD_WIDTH / 2 * EPD_HEIGHT` bytes large."]
    pub fn epd_draw_hline(x: i32, y: i32, length: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a horizontal line to a given framebuffer.\n\n @param x           Horizontal start position in pixels.\n @param y           Vertical start position in pixels.\n @param length      Length of the line in pixels.\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to, which must\n                    be `EPD_WIDTH / 2 * EPD_HEIGHT` bytes large."]
    pub fn epd_draw_vline(x: i32, y: i32, length: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a circle with given center and radius\n\n @param x0          Center-point x coordinate\n @param y0          Center-point y coordinate\n @param r           Radius of the circle in pixels\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to"]
    pub fn epd_draw_circle(x: i32, y: i32, r: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a circle with fill with given center and radius\n\n @param x0          Center-point x coordinate\n @param y0          Center-point y coordinate\n @param r           Radius of the circle in pixels\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to,"]
    pub fn epd_fill_circle(x: i32, y: i32, r: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a rectanle with no fill color\n\n @param x           Top left corner x coordinate\n @param y           Top left corner y coordinate\n @param w           Width in pixels\n @param h           Height in pixels\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to,"]
    pub fn epd_draw_rect(x: i32, y: i32, w: i32, h: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a rectanle with fill color\n\n @param x           Top left corner x coordinate\n @param y           Top left corner y coordinate\n @param w           Width in pixels\n @param h           Height in pixels\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to"]
    pub fn epd_fill_rect(x: i32, y: i32, w: i32, h: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Write a line.  Bresenham's algorithm - thx wikpedia\n\n @param x0          Start point x coordinate\n @param y0          Start point y coordinate\n @param x1          End point x coordinate\n @param y1          End point y coordinate\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to"]
    pub fn epd_write_line(x0: i32, y0: i32, x1: i32, y1: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a line\n\n @param x0          Start point x coordinate\n @param y0          Start point y coordinate\n @param x1          End point x coordinate\n @param y1          End point y coordinate\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to"]
    pub fn epd_draw_line(x0: i32, y0: i32, x1: i32, y1: i32, color: u8, framebuffer: *mut u8);
}
unsafe extern "C" {
    #[doc = " @brief Draw a triangle with no fill color\n\n @param x0          Vertex #0 x coordinate\n @param y0          Vertex #0 y coordinate\n @param x1          Vertex #1 x coordinate\n @param y1          Vertex #1 y coordinate\n @param x2          Vertex #2 x coordinate\n @param y2          Vertex #2 y coordinate\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to"]
    pub fn epd_draw_triangle(
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: u8,
        framebuffer: *mut u8,
    );
}
unsafe extern "C" {
    #[doc = " @brief Draw a triangle with color-fill\n\n @param x0          Vertex #0 x coordinate\n @param y0          Vertex #0 y coordinate\n @param x1          Vertex #1 x coordinate\n @param y1          Vertex #1 y coordinate\n @param x2          Vertex #2 x coordinate\n @param y2          Vertex #2 y coordinate\n @param color       The gray value of the line (0-255);\n @param framebuffer The framebuffer to draw to"]
    pub fn epd_fill_triangle(
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: u8,
        framebuffer: *mut u8,
    );
}
#[doc = " @brief Font data stored PER GLYPH"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GFXglyph {
    pub width: u8,
    #[doc = " Bitmap dimensions in pixels"]
    pub height: u8,
    #[doc = " Bitmap dimensions in pixels"]
    pub advance_x: u8,
    #[doc = " Distance to advance cursor (x axis)"]
    pub left: i16,
    #[doc = " X dist from cursor pos to UL corner"]
    pub top: i16,
    #[doc = " Y dist from cursor pos to UL corner"]
    pub compressed_size: u16,
    #[doc = " Size of the zlib-compressed font data."]
    pub data_offset: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GFXglyph"][::std::mem::size_of::<GFXglyph>() - 16usize];
    ["Alignment of GFXglyph"][::std::mem::align_of::<GFXglyph>() - 4usize];
    ["Offset of field: GFXglyph::width"][::std::mem::offset_of!(GFXglyph, width) - 0usize];
    ["Offset of field: GFXglyph::height"][::std::mem::offset_of!(GFXglyph, height) - 1usize];
    ["Offset of field: GFXglyph::advance_x"][::std::mem::offset_of!(GFXglyph, advance_x) - 2usize];
    ["Offset of field: GFXglyph::left"][::std::mem::offset_of!(GFXglyph, left) - 4usize];
    ["Offset of field: GFXglyph::top"][::std::mem::offset_of!(GFXglyph, top) - 6usize];
    ["Offset of field: GFXglyph::compressed_size"]
        [::std::mem::offset_of!(GFXglyph, compressed_size) - 8usize];
    ["Offset of field: GFXglyph::data_offset"]
        [::std::mem::offset_of!(GFXglyph, data_offset) - 12usize];
};
#[doc = " @brief Glyph interval structure"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UnicodeInterval {
    pub first: u32,
    #[doc = " The first unicode code point of the interval"]
    pub last: u32,
    #[doc = " The last unicode code point of the interval"]
    pub offset: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of UnicodeInterval"][::std::mem::size_of::<UnicodeInterval>() - 12usize];
    ["Alignment of UnicodeInterval"][::std::mem::align_of::<UnicodeInterval>() - 4usize];
    ["Offset of field: UnicodeInterval::first"]
        [::std::mem::offset_of!(UnicodeInterval, first) - 0usize];
    ["Offset of field: UnicodeInterval::last"]
        [::std::mem::offset_of!(UnicodeInterval, last) - 4usize];
    ["Offset of field: UnicodeInterval::offset"]
        [::std::mem::offset_of!(UnicodeInterval, offset) - 8usize];
};
#[doc = " @brief Data stored for FONT AS A WHOLE"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GFXfont {
    pub bitmap: *mut u8,
    #[doc = " Glyph bitmaps, concatenated"]
    pub glyph: *mut GFXglyph,
    #[doc = " Glyph array"]
    pub intervals: *mut UnicodeInterval,
    #[doc = " Valid unicode intervals for this font"]
    pub interval_count: u32,
    #[doc = " Number of unicode intervals."]
    pub compressed: bool,
    #[doc = " Does this font use compressed glyph bitmaps?"]
    pub advance_y: u8,
    #[doc = " Newline distance (y axis)"]
    pub ascender: i32,
    #[doc = " Maximal height of a glyph above the base line"]
    pub descender: i32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GFXfont"][::std::mem::size_of::<GFXfont>() - 28usize];
    ["Alignment of GFXfont"][::std::mem::align_of::<GFXfont>() - 4usize];
    ["Offset of field: GFXfont::bitmap"][::std::mem::offset_of!(GFXfont, bitmap) - 0usize];
    ["Offset of field: GFXfont::glyph"][::std::mem::offset_of!(GFXfont, glyph) - 4usize];
    ["Offset of field: GFXfont::intervals"][::std::mem::offset_of!(GFXfont, intervals) - 8usize];
    ["Offset of field: GFXfont::interval_count"]
        [::std::mem::offset_of!(GFXfont, interval_count) - 12usize];
    ["Offset of field: GFXfont::compressed"][::std::mem::offset_of!(GFXfont, compressed) - 16usize];
    ["Offset of field: GFXfont::advance_y"][::std::mem::offset_of!(GFXfont, advance_y) - 17usize];
    ["Offset of field: GFXfont::ascender"][::std::mem::offset_of!(GFXfont, ascender) - 20usize];
    ["Offset of field: GFXfont::descender"][::std::mem::offset_of!(GFXfont, descender) - 24usize];
};
unsafe extern "C" {
    #[doc = " @brief Get the text bounds for string, when drawn at (x, y).\n        Set font properties to NULL to use the defaults."]
    pub fn get_text_bounds(
        font: *const GFXfont,
        string: *const ::std::os::raw::c_char,
        x: *mut i32,
        y: *mut i32,
        x1: *mut i32,
        y1: *mut i32,
        w: *mut i32,
        h: *mut i32,
        props: *const FontProperties,
    );
}
unsafe extern "C" {
    #[doc = " @brief Write text to the EPD."]
    pub fn writeln(
        font: *const GFXfont,
        string: *const ::std::os::raw::c_char,
        cursor_x: *mut i32,
        cursor_y: *mut i32,
        framebuffer: *mut u8,
    );
}
unsafe extern "C" {
    #[doc = " @brief Write text to the EPD.\n\n @note If framebuffer is NULL, draw mode `mode` is used for direct drawing."]
    pub fn write_mode(
        font: *const GFXfont,
        string: *const ::std::os::raw::c_char,
        cursor_x: *mut i32,
        cursor_y: *mut i32,
        framebuffer: *mut u8,
        mode: DrawMode_t,
        properties: *const FontProperties,
    );
}
unsafe extern "C" {
    #[doc = " @brief Get the font glyph for a unicode code point."]
    pub fn get_glyph(font: *const GFXfont, code_point: u32, glyph: *mut *mut GFXglyph);
}
unsafe extern "C" {
    #[doc = " @brief Write a (multi-line) string to the EPD."]
    pub fn write_string(
        font: *const GFXfont,
        string: *const ::std::os::raw::c_char,
        cursor_x: *mut i32,
        cursor_y: *mut i32,
        framebuffer: *mut u8,
    );
}