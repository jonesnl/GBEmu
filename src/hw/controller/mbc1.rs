use hw::memory::{BusWidth, Bus};

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
    pub fn new(rom: Vec<u8>) -> Box<Bus> {
        let ramsize = match rom[0x149] {
            0x0 => 0,
            0x1 => 2048,
            0x2 => 8192,
            0x3 => 32768,
            _ => panic!("invalid ram size!"),
        };

        let mut ram_vec = vec![0u8; ramsize];

        Box::new(MBC1 {
            rom: rom,
            rom_bank_num: 1,
            ram: ram_vec,
            ram_enable: false,
            ram_bank_num: 0,
            mode_select: 0,
        })
    }
}

impl Bus for MBC1 {
    fn write8(&mut self, addr: BusWidth, data: u8) {
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
                    self.rom_bank_num = 0x1;
                } else {
                    self.rom_bank_num = data;
                }
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

    fn read8(&self, addr: BusWidth) -> u8 {
        match addr {
            0x0...0x3FFF => {
                self.rom[addr as usize]
            },
            0x4000...0x7FFF => {
                let bank_num = match self.mode_select {
                    0 => ((self.ram_bank_num as u16) << 5) | (self.rom_bank_num as u16),
                    _ => self.rom_bank_num as u16,
                };
                self.rom[((addr-0x4000) + 0x4000 * bank_num) as usize]
            },
            0xA000...0xBFFF => {
                self.ram[((addr-0xA000) + 0x2000 * self.ram_bank_num as u16) as usize]
            },
            _ => panic!("Illegal read from {}", addr),
        }
    }
}

#[test]
fn mbc1_rom_bank_test() {
    let mut rom_vec = vec![0u8; 1024 * 64];
    for i in 0..4 {
        rom_vec[i << 14] = i as u8;
    }
    let mut mbc1 = MBC1::new(rom_vec);
    for i in 1..4 {
        mbc1.write8(0x2000, i);
        assert!(mbc1.read8(0x4000) == i);
    }
}
