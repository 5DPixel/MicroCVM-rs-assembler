use std::fmt::Display;

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum OpcodeType {
    Load = 0x01,
    Store = 0x02,
    Add = 0x03,
    Sub = 0x04,
    Jmp = 0x05,
    Hlt = 0xFF,
    Mov = 0x06,
    Inc = 0x07,
    Div = 0x08,
    Mul = 0x09,
    Nop = 0x90,
    Call = 0x0A,
    Je = 0x0B,
    Jne = 0x0C,
    Cmp = 0x0D,
    And = 0x0E,
    Or = 0x0F,
    Xor = 0x10,
    Not = 0x11,
    Shl = 0x12,
    Shr = 0x13,
    //Stack opcodes
    Push = 0x14,
    Pop = 0x15,
    Ret = 0x16,

    Load8 = 0x17,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum Register {
    R0 = 0x1001,
    R1 = 0x1002,
    R2 = 0x1003,
    R3 = 0x1004,
    R4 = 0x1005,
    R5 = 0x1006,
    R6 = 0x1007,
    R7 = 0x1008,

    // Video argument registers
    V0 = 0x2001,  // Red, BMP file path
    V1 = 0x2002,  // Green
    V2 = 0x2003,  // Blue
    V3 = 0x2004,  // Line thickness
    V4 = 0x2005,  // Starting x coordinate, Rectangle centre x
    V5 = 0x2006,  // Starting y coordinate, Rectangle centre y
    V6 = 0x2007,  // Ending x coordinate, Rectangle size
    V7 = 0x2008,  // Ending y coordinate
    V8 = 0x2009,  // Quadrilateral point 1 x,
    V9 = 0x200A,  //Quadrilateral point 1 y,
    V10 = 0x200B, //Quadrilateral point 2 x,
    V11 = 0x200C, //Quadrilateral point 2 y,
    V12 = 0x200D, //Quadrilteral point 3 x,
    V13 = 0x200E, //Quadrilateral point 3 y,
    V14 = 0x200F, //Quadrilateral point 4 x,
    V15 = 0x2010, //Quadrilateral point 4 y
    //Keycode
    K0 = 0x3001,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FunctionCall {
    SetPixel = 0x13,
    DrawLine = 0x14,
    FillScreen = 0x15,
    ClearScreen = 0x16,
    LoadBMP = 0x17,
    FillRect = 0x18,
    FillQuad = 0x19,
    DrawCharacter = 0x1A,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InvalidOpcode(pub u16);

impl Display for InvalidOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Opcode: {}", self.0)
    }
}

pub struct InvalidOpcodeString(pub String);

impl Display for InvalidOpcodeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Opcode: {}", self.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InvalidFunctionCall(pub u16);

impl Display for InvalidFunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Function Call: {}", self.0)
    }
}

pub struct InvalidRegister(pub u16);

impl Display for InvalidRegister {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Register: {}", self.0)
    }
}

pub struct InvalidRegisterString(pub String);

impl Display for InvalidRegisterString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Register: {}", self.0)
    }
}

pub struct InvalidFunctionCallString(pub String);

impl Display for InvalidFunctionCallString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Function Call: {}", self.0)
    }
}

impl TryFrom<&str> for OpcodeType {
    type Error = InvalidOpcodeString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "load" => Ok(OpcodeType::Load),
            "store" => Ok(OpcodeType::Store),
            "add" => Ok(OpcodeType::Add),
            "sub" => Ok(OpcodeType::Sub),
            "jmp" => Ok(OpcodeType::Jmp),
            "hlt" => Ok(OpcodeType::Hlt),
            "mov" => Ok(OpcodeType::Mov),
            "inc" => Ok(OpcodeType::Inc),
            "div" => Ok(OpcodeType::Div),
            "mul" => Ok(OpcodeType::Mul),
            "nop" => Ok(OpcodeType::Nop),
            "call" => Ok(OpcodeType::Call),
            "je" => Ok(OpcodeType::Je),
            "jne" => Ok(OpcodeType::Jne),
            "cmp" => Ok(OpcodeType::Cmp),
            "and" => Ok(OpcodeType::And),
            "or" => Ok(OpcodeType::Or),
            "xor" => Ok(OpcodeType::Xor),
            "not" => Ok(OpcodeType::Not),
            "shl" => Ok(OpcodeType::Shl),
            "shr" => Ok(OpcodeType::Shr),
            "push" => Ok(OpcodeType::Push),
            "pop" => Ok(OpcodeType::Pop),
            "ret" => Ok(OpcodeType::Ret),
            "load8" => Ok(OpcodeType::Load8),
            invalid => return Err(InvalidOpcodeString(invalid.to_string())),
        }
    }
}

impl TryFrom<&str> for Register {
    type Error = InvalidRegisterString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "r0" => Ok(Register::R0),
            "r1" => Ok(Register::R1),
            "r2" => Ok(Register::R2),
            "r3" => Ok(Register::R3),
            "r4" => Ok(Register::R4),
            "r5" => Ok(Register::R5),
            "r6" => Ok(Register::R6),
            "r7" => Ok(Register::R7),
            //Video opcodes
            "v0" => Ok(Register::V0),
            "v1" => Ok(Register::V1),
            "v2" => Ok(Register::V2),
            "v3" => Ok(Register::V3),
            "v4" => Ok(Register::V4),
            "v5" => Ok(Register::V5),
            "v6" => Ok(Register::V6),
            "v7" => Ok(Register::V7),
            "v8" => Ok(Register::V8),
            "v9" => Ok(Register::V9),
            "v10" => Ok(Register::V10),
            "v11" => Ok(Register::V11),
            "v12" => Ok(Register::V12),
            "v13" => Ok(Register::V13),
            "v14" => Ok(Register::V14),
            "v15" => Ok(Register::V15),
            //Keycodes
            "k0" => Ok(Register::K0),
            invalid => Err(InvalidRegisterString(invalid.to_string())),
        }
    }
}

impl TryFrom<&str> for FunctionCall {
    type Error = InvalidFunctionCallString;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "set_pixel" => Ok(FunctionCall::SetPixel),
            "draw_line" => Ok(FunctionCall::DrawLine),
            "fill_screen" => Ok(FunctionCall::FillScreen),
            "clear_screen" => Ok(FunctionCall::ClearScreen),
            "load_bmp" => Ok(FunctionCall::LoadBMP),
            "fill_rect" => Ok(FunctionCall::FillRect),
            "fill_quad" => Ok(FunctionCall::FillQuad),
            "draw_character" => Ok(FunctionCall::DrawCharacter),
            invalid => Err(InvalidFunctionCallString(invalid.to_string())),
        }
    }
}
