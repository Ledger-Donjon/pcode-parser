/*
    Credits and Acknowledgments
    
    This Rust program was developed with the assistance and contributions of the following individuals and organizations:
    
    - @cchr-ledger : For providing the template for this parser, which was crucial for the successful completion of this project.
    - Michael Chesser and Rubens Brandao : For their extensive knowledge in binaries parsing and Rust language, which greatly enhanced the functionality of this program.

    Their expertise and support have been invaluable.
*/

#![allow(dead_code)]

pub mod parser_impl;
use std::collections::BTreeMap;

// update on Feb 5, 2024
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Opcode {
    Blank,
    BoolAnd,
    BoolNegate,
    BoolOr,
    BoolXor,
    Branch,
    BranchInd,  
    Build,
    Call,
    CallInd,  
    CallOther,  
    CBranch,  
    Ceil,
    Copy,
    CPoolRef,  
    CrossBuild,  
    DelaySlot,  
    Float2Float,  
    FloatAbs,
    FloatAdd,
    FloatDiv,
    FloatEqual,
    FloatLess,
    FloatLessEqual,  
    FloatMult,
    FloatNaN,  
    FloatNeg,
    FloatNotEqual, 
    FloatSqrt,
    FloatSub,
    FloatFloor,
    Int2Float,  
    Int2Comp, 
    IntAdd,
    IntAnd,
    IntCarry,
    IntDiv,
    IntEqual,
    IntLeft,
    IntLess,
    IntLessEqual,  
    IntMult,
    IntNegate,
    IntNotEqual,  
    IntOr,
    IntRem,
    IntRight,
    IntSBorrow,  
    IntSCarry,  
    IntSDiv, 
    IntSExt,  
    IntSLess,  
    IntSLessEqual,  
    IntSRem,  
    IntSRight,  
    IntSub,
    IntXor,
    IntZExt, 
    Label,
    Load,
    LZCount, 
    New,
    Piece,
    PopCount,  
    Return,
    Round,
    SegmentOp, 
    Store,
    SubPiece,  
    Trunc,
    Unused1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Hash)]
pub enum Size {
    Byte,
    Half,
    Word,
    Quad,
    Size9,
    Size10,
    DoubleQuad,
    QuadQuad,
}

impl Size {
    pub fn to_bitvector_size(self) -> u32 {
        match self {
            Size::Byte => 8,
            Size::Half => 16,
            Size::Word => 32,
            Size::Quad => 64,
            Size::Size9 => 72,
            Size::Size10 => 80,
            Size::DoubleQuad => 128,
            Size::QuadQuad => 256,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Varnode {
    pub var: Var,
    pub size: Size,
}

impl std::fmt::Debug for Varnode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Varnode {{ var: {:?}, size: {:?} }}", self.var, self.size)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Var {
    Const(String),
    Unique(Addr),
    Register(Addr, Size),
    Memory(Addr),
    MemoryRam, // Represents the case where the memory is referenced as (ram)
}

impl std::fmt::Debug for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Var::Const(val) => write!(f, "Const({})", val),
            Var::Unique(addr) => write!(f, "Unique(0x{:x})", addr),
            Var::Register(addr, size) => write!(f, "Register({:x}, {:?})", addr, size),
            Var::Memory(addr) => write!(f, "Memory(0x{:x})", addr),
            Var::MemoryRam => write!(f, "MemoryRam"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Inst {
    pub opcode: Opcode,
    pub output: Option<Varnode>,
    pub inputs: Vec<Varnode>,
}

pub type Addr = u64;

#[derive(Debug)]
pub struct CodeListing(BTreeMap<Addr, Vec<Inst>>);

impl CodeListing {
    pub fn new() -> Self {
        CodeListing(BTreeMap::new())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub data: i32,
}

impl Value {
    pub fn from_quad(input: u64) -> Self {
        // Assuming the input is i32,
        Self { data: input as i32 }
    }

    pub fn from_word(input: u32) -> Self {
        Self { data: input as i32 }
    }

    pub fn from_half(input: u16) -> Self {
        Self { data: input as i32 }
    }

    pub fn from_byte(input: u8) -> Self {
        Self { data: input as i32 }
    }
}
