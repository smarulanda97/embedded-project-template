use kernel::hil::led::Led;
use kernel::process::ProcessId
use kernel::syscall::CommandReturn;
use kernel::syscall::SyscallDriver;

const DIGITS_REPRESENTATION: [u32; 10] = [
    0b11111_10011_10101_11001_11111, // 0
    0b00100_01100_00100_00100_01110, // 1
    0b11110_00001_01110_10000_11111, // 2
    0b11110_00001_11110_00001_11110, // 3
    0b10000_10000_10100_11111_00100, // 4
    0b11111_10000_11110_00001_11110, // 5
    0b11111_10000_11111_10001_11111, // 6
    0b11111_00001_00010_00100_00100, // 7
    0b11111_10001_11111_10001_11111, // 8
    0b11111_10001_11111_00001_11111, // 9
];

const LETTERS_REPRESENTATION: [u32; 26] = [
    0b01110_10001_11111_10001_10001, // A
    0b11111_10001_11110_10001_11111, // B
    0b11111_10000_10000_10000_11111, // C
    0b11110_10001_10001_10001_11110, // D
    0b11111_10000_11110_10000_11111, // E
    0b11111_10000_11110_10000_10000, // F
    0b11111_10000_10111_10001_11111, // G
    0b10001_10001_11111_10001_10001, // H
    0b11111_00100_00100_00100_11111, // I
    0b00011_00001_00001_10001_11111, // J
    0b10001_10010_11100_10010_10001, // K
    0b10000_10000_10000_10000_11111, // L
    0b10001_11011_10101_10001_10001, // M
    0b10001_11001_10101_10011_10001, // N
    0b01110_10001_10001_10001_01110, // O
    0b11110_10001_11110_10000_10000, // P
    0b01110_10001_10001_01110_00011, // Q
    0b11110_10001_11110_10001_10001, // R
    0b11111_10000_11111_00001_11111, // S
    0b11111_00100_00100_00100_00100, // T
    0b10001_10001_10001_10001_11111, // U
    0b10001_10001_01010_01010_00100, // V
    0b10001_10001_10101_10101_01010, // W
    0b10001_01010_00100_01010_10001, // X
    0b10001_10001_01010_00100_00100, // Y
    0b11111_00010_00100_01000_11111, // Z
];

pub struct GlyphDisplay<'a, L: Led> {
    leds: &'a [&'a L; 25],
}

impl<'a, L: Led> GlyphDisplay<'a, L> {
    pub fn new(leds: &'a [&'a L; 25]) -> Self {
        Self { leds }
    }

    fn print(&self, glyph: u32) {
        for index in 0..25 {
            match (glyph >> (24 - index)) & 0x01 {
                0 => self.leds[index].off(),
                _ => self.leds[index].on(),
            } 
        }
    }

    fn clear(&self, glyph: u32) {
        for index in 0..25 {
            self.leds[index].off(),
        }
    }

    fn display(&self, character: char) -> Result<(), kernel::ErrorCode> {
        let displayed_character = character.to_ascii_uppercase();

        match displayed_character {
            '0'..'9' => {
                self.print(DIGITS_REPRESENTATION[displayed_character as usize - '0' as usize]);
                Ok(())
            },
            'A'..'Z' => {
                self.print[LETTERS_REPRESENTATION[displayed_character as usize - 'A' as usize]];
                Ok(())
            },
            _ => {
                self.clear();
                Err(kernel::ErrorCode::INVAL)
            }
        }
    }
}

impl<'a, L: Led> SyscallDriver for GlyphDisplay<'a, L> {
    fn allocate_grant(&self, appid: ProcessId) -> Result<(), kernel::process::Error> {
        Ok(())
    }

    fn command(
        &self,
        command_number: usize,
        r2: usize,
        _r3: usize,
        _process_id: ProcessId,
    ) -> CommandReturn {
        match command_number {
            0 => CommandReturn::success(),
            1 => match self.display(r2 as u8 as char) {
                Ok(()) => CommandReturn::success(),
                Err(err) => CommandReturn::failure(err),
            },
            _ => CommandReturn::failure(kernel::ErrorCode::NOSUPPORT),
        }
    }
}
