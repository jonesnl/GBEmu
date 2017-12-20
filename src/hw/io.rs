use hw::memory::{Bus, BusWidth};

pub struct IO {
    ioram: Vec<u8>,
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

pub enum LcdControllerMode {
    HorizontalBlankingImpulse,
    VerticalBlankingImpulse,
    LcdControllerOamAccess,
    LcdControllerOamAndVramAccess,
}

impl IO {
    pub fn new() -> IO {
        IO {
            ioram: vec![0u8; 0x80], // FF00-FF7F
        }
    }

    // TODO joypad
    
    // TODO sound
    
    // XXX Should these functions be put in an LCD module?

    // LCDCONT register accessors
    pub fn lcdcont(&self) -> u8 {
        self.ioram[0x40] 
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
            0 => 0x8800,
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
        self.ioram[0x41]
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

    pub fn lcd_controller_mode(&self) -> LcdControllerMode {
        let bits = self.lcdstat() & 0b11;
        match bits {
            0b00 => LcdControllerMode::HorizontalBlankingImpulse,
            0b01 => LcdControllerMode::VerticalBlankingImpulse,
            0b10 => LcdControllerMode::LcdControllerOamAccess,
            0b11 => LcdControllerMode::LcdControllerOamAndVramAccess,
            ____ => panic!("lcd_controller_mode {}", bits),
        }
    }

    pub fn scrolly(&self) -> u8 {
        self.ioram[0x42]
    }

    pub fn scrollx(&self) -> u8 {
        self.ioram[0x43]
    }

    pub fn curline(&self) -> u8 {
        self.ioram[0x44]
    }

    pub fn cmpline(&self) -> u8 {
        self.ioram[0x45]
    }

    pub fn bgrdpal(&self) -> u8 {
        self.ioram[0x47]
    }

    pub fn obj0pal(&self) -> u8 {
        self.ioram[0x48]
    }

    pub fn obj1pal(&self) -> u8 {
        self.ioram[0x49]
    }

    pub fn wndposy(&self) -> u8 {
        self.ioram[0x4A]
    }

    pub fn wndposx(&self) -> u8 {
        self.ioram[0x4B]
    }

    // TODO dma control
}

impl Bus for IO {
    fn write8(&mut self, addr: BusWidth, data: u8) {
        self.ioram[addr as usize] = data;
    }

    fn read8(&self, addr: BusWidth) -> u8 {
        self.ioram[addr as usize]
    }

    fn write16(&mut self, addr: BusWidth, data: u16) {
        self._write16_using_write8(addr, data);
    }

    fn read16(&self, addr: BusWidth) -> u16 {
        self._read16_using_read8(addr)
    }
}
