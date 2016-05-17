use game_data::{BusType, Cartridge};

pub struct MBC1 {
    rom: Vec<u8>,
    rom_bank_num: u8,
    ram: Vec<u8>,
    ram_enable: bool,
    ram_bank_num: u8, // also the upper bits of rom bank num 
                      // if mode select is 0
    mode_select: u8, // TODO make enum?
}

impl MBC1 {
    pub fn new(rom: Vec<u8>) -> Box<Cartridge> {
        let ramsize = match rom[0x149] {
            0x0 => 0,
            0x1 => 2048,
            0x2 => 8192,
            0x3 => 32768,
            _ => panic!("invalid ram size!"),
        };

        let mut ram_vec = Vec::new();
        ram_vec.resize(ramsize, 0);

        Box::new(MBC1 {
            rom: rom,
            rom_bank_num: 0,
            ram: ram_vec,
            ram_enable: false,
            ram_bank_num: 0,
            mode_select: 0,
        })
    }
}

impl Cartridge for MBC1 {
    // TODO actual cartridge mapping
    fn write8(&mut self, addr: BusType, mut data: u8) {
        match addr {
            0x0...0x1FFF => {
                if data & 0xF == 0xA {
                    self.ram_enable = true;
                } else {
                    self.ram_enable = false;
                }
            },
            0x2000...0x3FFF => {
                if data == 0x0 {
                    data = 0x1;
                }
                self.rom_bank_num = data;
            },
            0x4000...0x5FFF => {
                self.ram_bank_num = data;
            },
            0x6000...0x7FFF => {
                self.mode_select = data;
            },
            0xA000...0xBFFF => {
                let translated_addr = if self.mode_select == 0 {
                    (self.ram_bank_num as u16) << 10 & addr - 0xA000
                } else {
                    addr - 0xA000
                } as usize;

                self.ram[translated_addr] = data;
            },
            _ => panic!("Illegal write to {}", addr),
        };
    }

    fn read8(&self, addr: BusType) -> u8 {
        self.rom[addr as usize]
    }
}
