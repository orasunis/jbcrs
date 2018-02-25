use std::collections::{BTreeMap, HashMap};

use super::*;
use self::decode::Decoder;

/// Parses the code attribute
pub fn parse_code(decoder: &mut Decoder, constant_pool: &Pool) -> Result<Attribute> {
    let max_stack = decoder.read_u16()?;
    let max_locals = decoder.read_u16()?;

    let code_length = decoder.read_u32()?;
    let mut instructions = HashMap::new();

    // Read the instructions
    // Using an extra block so we don't have to enable NLL on nightly
    // A method doesn't seem necessary to me, too
    {
        let mut code_decoder = decoder.limit(code_length as usize)?;

        let mut code_location = 0;
        loop {
            let (end, instruction) = parse_instruction(&mut code_decoder, code_location)?;
            instructions.insert(code_location, instruction);

            // we have read all instructions
            if end == code_length {
                break;
            }

            code_location = end;
        }

        code_decoder.remove_limit()?;
    }

    // Read all exceptions
    let exception_count = decoder.read_u16()?;
    let mut exceptions = Vec::with_capacity(exception_count as usize);
    for _ in 0..exception_count {
        let start = decoder.read_u16()?;
        let end = decoder.read_u16()?;
        let handler = decoder.read_u16()?;
        let catch_type = decoder.read_u16()?;
        exceptions.push(Exception {
            start,
            end,
            handler,
            catch_type,
        });
    }

    let attributes = parse_attributes(decoder, constant_pool)?;

    Ok(Attribute::Code {
        max_stack,
        max_locals,
        instructions,
        exceptions,
        attributes,
    })
}

/// Parses a single instruction and then returns its end
fn parse_instruction(decoder: &mut Decoder, at: u32) -> Result<(u32, Instruction)> {
    use self::Instruction::*;

    let prev_cursor = decoder.cursor();
    let op_code = decoder.read_u8()?;

    let insn = match op_code {
        0x00 => NOP,
        0x01 => AConstNull,
        0x02 => IConstM1,
        0x03 => IConst0,
        0x04 => IConst1,
        0x05 => IConst2,
        0x06 => IConst3,
        0x07 => IConst4,
        0x08 => IConst5,
        0x09 => LConst0,
        0x0A => LConst1,
        0x0B => FConst0,
        0x0C => FConst1,
        0x0D => FConst2,
        0x0E => DConst0,
        0x0F => DConst1,
        0x10 => BIPush(decoder.read_i8()?),
        0x11 => SIPush(decoder.read_i16()?),
        0x12 => LDC(u16::from(decoder.read_u8()?)), // ldc
        0x13 | 0x14 => LDC(decoder.read_u16()?),    // ldc_w
        0x15 => ILoad(u16::from(decoder.read_u8()?)),
        0x16 => LLoad(u16::from(decoder.read_u8()?)),
        0x17 => FLoad(u16::from(decoder.read_u8()?)),
        0x18 => DLoad(u16::from(decoder.read_u8()?)),
        0x19 => ALoad(u16::from(decoder.read_u8()?)),
        0x1A => ILoad0,
        0x1B => ILoad1,
        0x1C => ILoad2,
        0x1D => ILoad3,
        0x1E => LLoad0,
        0x1F => LLoad1,
        0x20 => LLoad2,
        0x21 => LLoad3,
        0x22 => FLoad0,
        0x23 => FLoad1,
        0x24 => FLoad2,
        0x25 => FLoad3,
        0x26 => DLoad0,
        0x27 => DLoad1,
        0x28 => DLoad2,
        0x29 => DLoad3,
        0x2A => ALoad0,
        0x2B => ALoad1,
        0x2C => ALoad2,
        0x2D => ALoad3,
        0x2E => IALoad,
        0x2F => LALoad,
        0x30 => FALoad,
        0x31 => DALoad,
        0x32 => AALoad,
        0x33 => BALoad,
        0x34 => CALoad,
        0x35 => SALoad,
        0x36 => IStore(u16::from(decoder.read_u8()?)),
        0x37 => LStore(u16::from(decoder.read_u8()?)),
        0x38 => FStore(u16::from(decoder.read_u8()?)),
        0x39 => DStore(u16::from(decoder.read_u8()?)),
        0x3A => AStore(u16::from(decoder.read_u8()?)),
        0x3B => IStore0,
        0x3C => IStore1,
        0x3D => IStore2,
        0x3E => IStore3,
        0x3F => LStore0,
        0x40 => LStore1,
        0x41 => LStore2,
        0x42 => LStore3,
        0x43 => FStore0,
        0x44 => FStore1,
        0x45 => FStore2,
        0x46 => FStore3,
        0x47 => DStore0,
        0x48 => DStore1,
        0x49 => DStore2,
        0x4A => DStore3,
        0x4B => AStore0,
        0x4C => AStore1,
        0x4D => AStore2,
        0x4E => AStore3,
        0x4F => IAStore,
        0x50 => LAStore,
        0x51 => FAStore,
        0x52 => DAStore,
        0x53 => AAStore,
        0x54 => BAStore,
        0x55 => CAStore,
        0x56 => SAStore,
        0x57 => Pop,
        0x58 => Pop2,
        0x59 => Dup,
        0x5A => DupX1,
        0x5B => DupX2,
        0x5C => Dup2,
        0x5D => Dup2X1,
        0x5E => Dup2X2,
        0x5F => Swap,
        0x60 => IAdd,
        0x61 => LAdd,
        0x62 => FAdd,
        0x63 => DAdd,
        0x64 => ISub,
        0x65 => LSub,
        0x66 => FSub,
        0x67 => DSub,
        0x68 => IMul,
        0x69 => LMul,
        0x6A => FMul,
        0x6B => DMul,
        0x6C => IDiv,
        0x6D => LDiv,
        0x6E => FDiv,
        0x6F => DDiv,
        0x70 => IRem,
        0x71 => LRem,
        0x72 => FRem,
        0x73 => DRem,
        0x74 => INeg,
        0x75 => LNeg,
        0x76 => FNeg,
        0x77 => DNeg,
        0x78 => IShL,
        0x79 => LShL,
        0x7A => IShR,
        0x7B => LShR,
        0x7C => IUShR,
        0x7D => LUShR,
        0x7E => IAnd,
        0x7F => LAnd,
        0x80 => IOr,
        0x81 => LOr,
        0x82 => IXOr,
        0x83 => LXOr,
        0x84 => {
            let index = u16::from(decoder.read_u8()?);
            let value = i16::from(decoder.read_i8()?);
            IInc(index, value)
        }
        0x85 => I2L,
        0x86 => I2F,
        0x87 => I2D,
        0x88 => L2I,
        0x89 => L2F,
        0x8A => L2D,
        0x8B => F2I,
        0x8C => F2L,
        0x8D => F2D,
        0x8E => D2I,
        0x8F => D2L,
        0x90 => D2F,
        0x91 => I2B,
        0x92 => I2C,
        0x93 => I2S,
        0x94 => LCmp,
        0x95 => FCmpL,
        0x96 => FCmpG,
        0x97 => DCmpL,
        0x98 => DCmpG,
        0x99 => IfEq(decoder.read_i16()?),
        0x9A => IfNE(decoder.read_i16()?),
        0x9B => IfLT(decoder.read_i16()?),
        0x9C => IfGE(decoder.read_i16()?),
        0x9D => IfGT(decoder.read_i16()?),
        0x9E => IfLE(decoder.read_i16()?),
        0x9F => IfICmpEq(decoder.read_i16()?),
        0xA0 => IfICmpNE(decoder.read_i16()?),
        0xA1 => IfICmpLT(decoder.read_i16()?),
        0xA2 => IfICmpGE(decoder.read_i16()?),
        0xA3 => IfICmpGT(decoder.read_i16()?),
        0xA4 => IfICmpLE(decoder.read_i16()?),
        0xA5 => IfACmpEq(decoder.read_i16()?),
        0xA6 => IfACmpNE(decoder.read_i16()?),
        0xA7 => GoTo(i32::from(decoder.read_i16()?)),
        0xA8 => JSR(i32::from(decoder.read_i16()?)),
        0xA9 => Ret(u16::from(decoder.read_u8()?)),
        0xAA => {
            // skip padding
            decoder.skip(3 - (at & 3) as usize)?;
            let default = decoder.read_i32()?;
            let low = decoder.read_i32()?;
            let high = decoder.read_i32()?;

            let mut offsets = Vec::with_capacity((high - low + 1) as usize);
            for _ in low..(high + 1) {
                offsets.push(decoder.read_i32()?);
            }

            TableSwitch {
                default,
                low,
                high,
                offsets,
            }
        }
        0xAB => {
            // skip padding
            decoder.skip(3 - (at & 3) as usize)?;
            let default = decoder.read_i32()?;

            let count = decoder.read_u32()?;
            let mut offsets = BTreeMap::new();
            for _ in 0..count {
                let key = decoder.read_i32()?;
                let offset = decoder.read_i32()?;
                offsets.insert(key, offset);
            }

            LookupSwitch { default, offsets }
        }
        0xAC => IReturn,
        0xAD => LReturn,
        0xAE => FReturn,
        0xAF => DReturn,
        0xB0 => AReturn,
        0xB1 => Return,
        0xB2 => GetStatic(decoder.read_u16()?),
        0xB3 => PutStatic(decoder.read_u16()?),
        0xB4 => GetField(decoder.read_u16()?),
        0xB5 => PutField(decoder.read_u16()?),
        0xB6 => InvokeVirtual(decoder.read_u16()?),
        0xB7 => InvokeSpecial(decoder.read_u16()?),
        0xB8 => InvokeStatic(decoder.read_u16()?),
        0xB9 => {
            let index = decoder.read_u16()?;
            let count = decoder.read_u8()?;
            decoder.skip(1)?;
            InvokeInterface(index, count)
        }
        0xBA => {
            let index = decoder.read_u16()?;
            decoder.skip(2)?;
            InvokeDynamic(index)
        }
        0xBB => New(decoder.read_u16()?),
        0xBC => NewArray(match decoder.read_u8()? {
            0x04 => ArrayType::Boolean,
            0x05 => ArrayType::Char,
            0x06 => ArrayType::Float,
            0x07 => ArrayType::Double,
            0x08 => ArrayType::Byte,
            0x09 => ArrayType::Short,
            0x0A => ArrayType::Int,
            0x0B => ArrayType::Long,

            _ => return Err(Error::InvalidInstruction { op_code: 0xBC, at }),
        }),
        0xBD => ANewArray(decoder.read_u16()?),
        0xBE => ArrayLength,
        0xBF => AThrow,
        0xC0 => CheckCast(decoder.read_u16()?),
        0xC1 => InstanceOf(decoder.read_u16()?),
        0xC2 => MonitorEnter,
        0xC3 => MonitorExit,
        0xC4 => match decoder.read_u8()? {
            0x15 => ILoad(decoder.read_u16()?),
            0x16 => LLoad(decoder.read_u16()?),
            0x17 => FLoad(decoder.read_u16()?),
            0x18 => DLoad(decoder.read_u16()?),
            0x19 => ALoad(decoder.read_u16()?),
            0x36 => IStore(decoder.read_u16()?),
            0x37 => LStore(decoder.read_u16()?),
            0x38 => FStore(decoder.read_u16()?),
            0x39 => DStore(decoder.read_u16()?),
            0x40 => AStore(decoder.read_u16()?),
            0x84 => {
                let index = decoder.read_u16()?;
                let value = decoder.read_i16()?;
                IInc(index, value)
            }
            0xA9 => Ret(decoder.read_u16()?),

            _ => return Err(Error::InvalidInstruction { op_code: 0xC4, at }),
        },
        0xC5 => {
            let array_type = decoder.read_u16()?;
            let dimensions = decoder.read_u8()?;
            MultiANewArray(array_type, dimensions)
        }
        0xC6 => IfNull(decoder.read_i16()?),
        0xC7 => IfNonNull(decoder.read_i16()?),
        0xC8 => GoTo(decoder.read_i32()?),
        0xC9 => JSR(decoder.read_i32()?),
        0xCA => BreakPoint,
        0xFE => ImpDep1,
        0xFF => ImpDep2,

        _ => return Err(Error::InvalidInstruction { op_code, at }),
    };

    Ok(((at as usize + decoder.cursor() - prev_cursor) as u32, insn))
}
