#![feature(rust_2018_preview)]
#![feature(rust_2018_idioms)]
#![feature(try_from)]
#![warn(missing_docs, missing_debug_implementations)]

//! Experiments in decoding Game Boy machine code.

#[macro_use]
extern crate derive_more;


/// Encoding/decoding individual CPU instructions.
pub mod instruction;

/// Decoding and disassembling assembled ROMs binaries.
pub mod assembled;

/// Assembling and manipulating disassembled ROM programs.
pub mod disassembled;

/// Re-exports important traits and types for glob importing.
pub mod prelude {
    pub use crate::assembled::prelude::*;
    pub use crate::disassembled::prelude::*;
    pub use crate::instruction::prelude::*;
}

use self::prelude::*;

fn block(address: u16, value: impl Into<crate::disassembled::RomBlockContent>) -> RomBlock {
    RomBlock {
        address: Some(address),
        content: value.into()
    }
}

pub fn demo() -> DisassembledRom {
    let disassembled = make_demo();

    println!("=== Demo Rom Source ===");
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    println!("=== Assembled ===");
    let assembled = disassembled.assemble();
    println!("{:?}\n", assembled.to_bytes());

    println!("=== Redisassembled (using metadata) ===");
    let redisassembled = assembled.disassemble();
    println!("{:?}\n", redisassembled);
    println!("{}\n", redisassembled);

    println!("=== Redisassembled (just from the bytes) ===");
    let really_disassembled = AssembledRom::new(assembled.to_bytes()).disassemble();
    println!("{:?}\n", really_disassembled);
    println!("{}\n", really_disassembled);

    disassembled
}

/// Returns a DisassembledRom with our demo program.
pub fn make_demo() -> DisassembledRom {
    let generic_header = vec![
        // Game ROM entry point, from which we jump to our main function.
        block(0x0100, vec![JP(0x0150)]),
        
        // Nintendo logo, must be exactly this or boot ROM will freeze.
        block(0x0104, vec![
            0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
            0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
            0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
            0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
            0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
            0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
        ]),

        // Game metadata.
        // Since we only require the minimal feature set we can leave this zeroed.
        block(0x0134, vec![0x00; 25]),

        // Metadata checksum, must be sum of metadata bytes + 0xE7 or boot ROM will freeze.
        block(0x014D, vec![0xE7]),

        // Global checksum of all other bytes in the ROM... but not verified, so'll neglect it.
        block(0x014E, vec![0x00, 0x00]),
    ];

    let demo_body = vec![
        // Main function.
        block(0x0150, vec![
            // Set background palette
            LD_16_IMMEDIATE(HL, 0xFF47),
            // to [black, dark gray, light gray, white]
            LD_8_IMMEDIATE(A, 0b_00_01_10_11),
            LD_8_INTERNAL(AT_HL, A),
        //     // Set first tile to black.
        //     LD(HL, 0x8000),
        //     LD(A, 0xFF),
        //     LD(AtHLPostIncrement, A), // *(HL++) = A
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),
        //     LD(AtHLPostIncrement, A),

        //     // Set tiles to draw my logo in the corner.
        //     // We're using the second tile (0x01), which is white by default.
        //     LD(A, 0x01),
        //     LD(HL, 0x9800),
            
        //     LD (HL+), A
        //     and INC HL
        //     // mixed to set/skip tiles to display logo
        //     // 0x23, 0x23, 0x23, 0x22, 0x22, 0x23, 0x23, 0x23,
        //     // // LD HL, 0x9820
        //     // 0x21, 0x20, 0x98,
        //     // 0x23, 0x23, 0x22, 0x23, 0x23, 0x22, 0x23, 0x23,
        //     // 0x21, 0x40, 0x98,
        //     // 0x23, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x23,
        //     // 0x21, 0x60, 0x98,
        //     // 0x22, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x22,
        //     // 0x21, 0x80, 0x98,
        //     // 0x23, 0x22, 0x23, 0x23, 0x23, 0x23, 0x22, 0x23,
        //     // 0x21, 0xA0, 0x98,
        //     // 0x22, 0x22, 0x22, 0x23, 0x23, 0x22, 0x22, 0x22,
        //     ///
        //     // // scroll background into middle of screen
        //     // // LD A, -32
        //     // 0x3E, 0xFF - 0x32 + 1,
        //     // // LD HL, 0xFF42 ; y-scroll register
        //     // 0x21, 0x42, 0xFF,
        //     // // LD (HL), A
        //     // 0x77,
        //     // // LD HL, 0xFF43 ; x-scroll register
        //     // 0x21, 0x43, 0xFF,
        //     // // LD (HL), A
        //     // 0x77,
        //     //
        //     // // infinite loop
        //     // // JR -2
        //     // 0x18, (0xFF - 2 + 1),
        ]),

        // block(0x0200, vec![
        //     // // Set background palette to [black, dark gray, light gray, white].
        //     // // LD HL, 0xFF47
        //     // 0x21, 0x47, 0xFF,
        //     // // LD A, 0b00011011
        //     // 0x3E, 0b00011011,
        //     // // LD (HL), A
        //     // 0x77,
        //     //
        //     // // Set first tile to black.
        //     // // LD HL, 0x8000
        //     // 0x21, 0x00, 0x80,
        //     // // LD A, 0xFF
        //     // 0x3E, 0xFF,
        //     // // LD (HL+), A
        //     // // repeated 16 times
        //     // 0x22, 0x22, 0x22, 0x22,
        //     // 0x22, 0x22, 0x22, 0x22,
        //     // 0x22, 0x22, 0x22, 0x22,
        //     // 0x22, 0x22, 0x22, 0x22,
        //     ///
        //     // // Set tiles to draw my logo in the corner.
        //     // // LD A, 0x01
        //     // 0x3E, 0x01,
        //     // // LD HL, 0x9800
        //     // 0x21, 0x00, 0x98,
        //     // // LD (HL+), A
        //     // // and INC HL
        //     // // mixed to set/skip tiles to display logo
        //     // 0x23, 0x23, 0x23, 0x22, 0x22, 0x23, 0x23, 0x23,
        //     // // LD HL, 0x9820
        //     // 0x21, 0x20, 0x98,
        //     // 0x23, 0x23, 0x22, 0x23, 0x23, 0x22, 0x23, 0x23,
        //     // 0x21, 0x40, 0x98,
        //     // 0x23, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x23,
        //     // 0x21, 0x60, 0x98,
        //     // 0x22, 0x23, 0x23, 0x23, 0x23, 0x23, 0x23, 0x22,
        //     // 0x21, 0x80, 0x98,
        //     // 0x23, 0x22, 0x23, 0x23, 0x23, 0x23, 0x22, 0x23,
        //     // 0x21, 0xA0, 0x98,
        //     // 0x22, 0x22, 0x22, 0x23, 0x23, 0x22, 0x22, 0x22,
        //     ///
        //     // // scroll background into middle of screen
        //     // // LD A, -32
        //     // 0x3E, 0xFF - 0x32 + 1,
        //     // // LD HL, 0xFF42 ; y-scroll register
        //     // 0x21, 0x42, 0xFF,
        //     // // LD (HL), A
        //     // 0x77,
        //     // // LD HL, 0xFF43 ; x-scroll register
        //     // 0x21, 0x43, 0xFF,
        //     // // LD (HL), A
        //     // 0x77,
        //     //

        
        RomBlock {
            address: None,
            content: Code(vec![JP(0x0180)])
        },
        block(0x0180, vec![
            // Mess with the pallet forever:
            LD_16_IMMEDIATE(HL, 0xFF47),
            LD_8_INTERNAL(A, AT_HL),
            INC(A),
            LD_8_INTERNAL(AT_HL, A),
            JP(0x0180),
        ]),

        RomBlock {
            address: None,
            // Loop back to main.
            content: Code(vec![JP(0x0150)])
        }
    ];

    DisassembledRom::from(vec![generic_header, demo_body].concat())
}

#[test]
pub fn test() -> Result<(), Box<std::any::Any + Send>> {
    let disassembled = demo();

    println!("=== Input ===");
    println!("{:?}\n", disassembled);
    println!("{}\n", disassembled);

    println!("=== Assembled ===");
    let assembled = disassembled.assemble();
    println!("{:?}\n", assembled.to_bytes());

    println!("=== Redisassembled (using metadata) ===");
    let redisassembled = assembled.disassemble();
    println!("{:?}\n", redisassembled);
    println!("{}\n", redisassembled);

    println!("=== Redisassembled (just from the bytes) ===");
    let really_disassembled = AssembledRom::new(assembled.to_bytes()).disassemble();
    println!("{:?}\n", really_disassembled);
    println!("{}\n", really_disassembled);

    Ok(())
}
