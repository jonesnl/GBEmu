use crate::emu_log;
use crate::hw::memory::{Bus, BusWidth};
use rgb::RGBA8;

const OAM_TICKS: u16 = 80;
const OAM_AND_VRAM_TICKS: u16 = 172;
const HBLANK_TICKS: u16 = 204;
const VBLANK_TICKS: u16 = 4560;

pub struct Point {
    x: u8,
    y: u8,
}

pub enum SpriteSize {
    SS8x16,
    SS8x8,
}

pub enum Transparency {
    Solid,
    Transparent,
}

pub enum CoincidenceFlag {
    Coincidence,
    NoCoincidence,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LcdControllerMode {
    OamAccess(u16),
    OamAndVramAccess(u16),
    HorizontalBlank(u16),
    VerticalBlank(u16),
}

// TODO should vram be put in here?
pub struct LCD {
    vram: Vec<u8>,
    oam: Vec<u8>,
    lcdram: Vec<u8>,
    pub lcd_display: Vec<RGBA8>,
    pub drawing_state: LcdControllerMode,
}

impl Bus for LCD {
    fn write8(&mut self, addr: BusWidth, data: u8) {
        match addr {
            0x8000..=0x9FFF => {
                self.vram[(addr-0x8000) as usize] = data;
            },
            0xFE00..=0xFE9F => {
                self.oam[(addr-0xFE00) as usize] = data;
            },
            0xFF46 => panic!("Should not have DMA addr in LCD"),
            0xFF41 => println!("Writing to LCDSTAT not currently supported!"),
            0xFF40..=0xFF4B => {
                self.lcdram[(addr as usize) - 0xFF40] = data
            },
            _ => panic!("Illegal write address {} for LCD", addr),
        }
    }

    fn read8(&self, addr: BusWidth) -> u8 {
        match addr {
            0x8000..=0x9FFF => {
                self.vram[(addr-0x8000) as usize]
            },
            0xFE00..=0xFE9F => {
                self.oam[(addr-0xFE00) as usize]
            },
            0xFF46 => panic!("Should not have DMA addr in LCD"),
            0xFF41 => {
                self.lcdstat()
            },
            0xFF40..=0xFF4B => {
                self.lcdram[(addr as usize) - 0xFF40]
            },
            _ => panic!("Illegal read address {} for LCD", addr),
        }
    }
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            vram: vec![0u8; 8 * 1024],
            oam: vec![0u8; 100],
            lcdram: vec![0u8; 0xC], // FF40 - FF4B
            lcd_display: vec![RGBA8{r:0, g:0, b:0, a: 0}; 160 * 144],
            drawing_state: LcdControllerMode::OamAccess(0),
        }
    }

    pub fn tick_update(&mut self) {
        use self::LcdControllerMode::*;
        self.drawing_state = match self.drawing_state {
            OamAccess(cnt) => {
                if cnt >= OAM_TICKS {
                    OamAndVramAccess(0)
                } else {
                    OamAccess(cnt + 1)
                }
            },
            OamAndVramAccess(cnt) => {
                if cnt >= OAM_AND_VRAM_TICKS {
                    if self.curline() == 144 {
                        VerticalBlank(0)
                    } else {
                        self.update_bg_map();
                        HorizontalBlank(0)
                    }
                } else {
                    OamAndVramAccess(cnt + 1)
                }
            },
            HorizontalBlank(cnt) => {
                if cnt >= HBLANK_TICKS {
                    *self.curline_mut() += 1;
                    OamAccess(0)
                } else {
                    HorizontalBlank(cnt + 1)
                }
            },
            VerticalBlank(cnt) => {
                if cnt >= VBLANK_TICKS {
                    *self.curline_mut() = 0;
                    OamAccess(0)
                } else {
                    VerticalBlank(cnt + 1)
                }
            },
        };
    }

    fn set_lcd_pixel(&mut self, point: Point, rgb: RGBA8) {
        let y = point.y as usize;
        let x = point.x as usize;
        self.lcd_display[y * 160 + x] = rgb;
    }

    fn get_bg_pixel(&self, bg_map: Point) -> u8 {
        let bg_tt_addr = self.background_tile_table_base_addr();
        let tpt_addr = self.tile_pattern_table_base_addr();
        let tile_col = (bg_map.x / 8) as u16;
        let pixel_col = (bg_map.x % 8) as u16;
        let tile_row = (bg_map.y / 8) as u16;
        let pixel_row = (bg_map.y % 8) as u16;

        let pixel_from_tile = |tile_base_addr, (x, y)| -> u8 {
            let addr = tile_base_addr + (y as BusWidth / 8 * 2);
            let upper_byte = self.read8(addr);
            let lower_byte = self.read8(addr+1);
            let upper_result = (upper_byte>>(7-x)) & 1;
            let lower_result = (lower_byte>>(7-x)) & 1;
            (upper_result<<1) | (lower_result)
        };

        match tpt_addr {
            0x8000 => {
                let tt_addr = bg_tt_addr + tile_row * 32 + tile_col;
                let tt_entry = self.read8(tt_addr) as i32;
                let tile_base_addr = tpt_addr as i32 + tt_entry as i32 * 16;
                let tile_base_addr = tile_base_addr as u16;
                pixel_from_tile(
                    tile_base_addr, (pixel_col, pixel_row))
            },
            0x9000 => {
                let tt_addr = bg_tt_addr + tile_row * 32 + tile_col;
                let tt_entry = self.read8(tt_addr) as i8;
                let tile_base_addr = tpt_addr + (tt_entry as u16) * 16;
                pixel_from_tile(
                    tile_base_addr, (pixel_col, pixel_row))
            },
            _ => panic!("tile_pattern_table {}", tpt_addr),
        }
    }

    fn update_bg_map(&mut self) {
        // TODO figure out what we should be returning

        let scx = self.scrollx();
        let scy = self.scrolly();
        let curline = self.curline();

        // TODO translate background_tile_table entries into a full
        // background map (probably in gameboy greyscale format?)
        for x in 0..160 {
            let bg_map_x = scx.wrapping_add(x);
            let bg_map_y = scy.wrapping_add(curline);
            let bg_point = Point{x: bg_map_x, y: bg_map_y};
            let pixel = self.get_bg_pixel(bg_point);
            let rgb = RGBA8{r: pixel * 100, g: pixel * 100, b: pixel * 100, a: 255};
            let lcd_point = Point{x, y: curline};
            self.set_lcd_pixel(lcd_point, rgb);
        }
    }

    // LCDCONT register accessors
    pub fn lcdcont(&self) -> u8 {
        self.lcdram[0x0] 
    }

    pub fn lcd_operation(&self) -> bool {
        (self.lcdcont()>>7) & 1 == 1
    }

    pub fn window_tile_table_base_addr(&self) -> BusWidth {
        let bit = (self.lcdcont()>>6) & 1;
        match bit {
            0 => 0x9800,
            1 => 0x9C00,
            _ => panic!("window_table_addr {}", bit),
        }
    }

    pub fn window_display(&self) -> bool {
        (self.lcdcont()>>5) & 1 == 1
    }

    pub fn tile_pattern_table_base_addr(&self) -> BusWidth {
        let bit = (self.lcdcont()>>4) & 1;
        match bit {
            0 => 0x9000,
            1 => 0x8000,
            _ => panic!("tile_pattern_table {}", bit),
        }
    }

    pub fn background_tile_table_base_addr(&self) -> BusWidth {
        let bit = (self.lcdcont()>>3) & 1;
        match bit {
            0 => 0x9800,
            1 => 0x9C00,
            _ => panic!("background_tile_table {}", bit),
        }
    }

    pub fn sprite_size(&self) -> SpriteSize {
        let bit = (self.lcdcont()>>2) & 1;
        match bit {
            0 => SpriteSize::SS8x8,
            1 => SpriteSize::SS8x16,
            _ => panic!("sprite_size {}", bit),
        }
    }

    pub fn color_0_window_transparency(&self) -> Transparency {
        let bit = (self.lcdcont()>>1) & 1;
        match bit {
            0 => Transparency::Transparent,
            1 => Transparency::Solid,
            _ => panic!("color_0_window_transparency {}", bit),
        }
    }

    pub fn background_display(&self) -> bool {
        self.lcdcont() & 1 == 1
    }

    // LCDSTAT register accessors
    pub fn lcdstat(&self) -> u8 {
        // TODO add interrupt and coincidence flag to result
        use self::LcdControllerMode::*;
        match self.drawing_state {
            OamAccess(_) => 2,
            OamAndVramAccess(_) => 3,
            HorizontalBlank(_) => 0,
            VerticalBlank(_) => 1,
        }
    }

    pub fn scanline_coincidence_interrupt(&self) -> bool {
        (self.lcdstat()>>6) & 1 == 1
    }

    pub fn controller_mode_10_interrupt(&self) -> bool {
        (self.lcdstat()>>5) & 1 == 1
    }

    pub fn controller_mode_01_interrupt(&self) -> bool {
        (self.lcdstat()>>4) & 1 == 1
    }

    pub fn controller_mode_00_interrupt(&self) -> bool {
        (self.lcdstat()>>3) & 1 == 1
    }

    pub fn scanline_coincidence_flag(&self) -> CoincidenceFlag {
        let bit = (self.lcdstat()>>2) & 1;
        match bit {
            0 => CoincidenceFlag::NoCoincidence,
            1 => CoincidenceFlag::Coincidence,
            _ => panic!("scanline_coincidence_flag {}", bit),
        }
    }

    pub fn scrolly(&self) -> u8 {
        self.lcdram[0x2]
    }

    pub fn scrollx(&self) -> u8 {
        self.lcdram[0x3]
    }

    pub fn curline(&self) -> u8 {
        self.lcdram[0x4]
    }

    pub fn curline_mut(&mut self) -> &mut u8 {
        &mut self.lcdram[0x4]
    }

    pub fn cmpline(&self) -> u8 {
        self.lcdram[0x5]
    }

    pub fn bgrdpal(&self) -> u8 {
        self.lcdram[0x7]
    }

    pub fn obj0pal(&self) -> u8 {
        self.lcdram[0x8]
    }

    pub fn obj1pal(&self) -> u8 {
        self.lcdram[0x9]
    }

    pub fn wndposy(&self) -> u8 {
        self.lcdram[0xA]
    }

    pub fn wndposx(&self) -> u8 {
        self.lcdram[0xB]
    }
}
