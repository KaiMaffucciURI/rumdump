use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bitpack;
type Umi = u32;

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field {width: 3, lsb: 6};
static RB: Field = Field {width: 3, lsb: 3};
static RC: Field = Field {width: 3, lsb: 0};
static RL: Field = Field {width: 3, lsb: 25};
static VL: Field = Field {width: 25, lsb: 0};
static OP: Field = Field {width: 4, lsb: 28};

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    Map,
    Unmap,
    Out,
    In,
    LoadP,
    LoadV,
}

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    bitpack::bitpack::getu(instruction as u64, field.width as u64, field.lsb as u64).unwrap() as u32
}

/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32(bitpack::bitpack::getu(instruction as u64, OP.width as u64, OP.lsb as u64).unwrap() as u32)
}

pub fn disassemble(inst: Umi) -> String {
    match op(inst) {
        Some(Opcode::CMov) => {
            format!(
                "if (r{} != 0) r{} := r{};",
                get(&RC, inst),
                get(&RA, inst),
                get(&RB, inst)
            )
        }
        Some(Opcode::Load) => {
            format!(
                "r{} := M[r{} + {}];",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Store) => {
            format!(
                "M[r{} + {}] := r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Add) => {
            format!(
                "r{} := r{} + r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Mul) => {
            format!(
                "r{} := r{} * r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Div) => {
            format!(
                "r{} := r{} / r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Nand) => {
            format!(
                "r{} := ~(r{} & r{});",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Halt) => {
            format!("halt;")
        }
        Some(Opcode::Map) => {
            format!(
                "r{} := map(r{});",
                get(&RA, inst),
                get(&RB, inst)
            )
        }
        Some(Opcode::Unmap) => {
            format!(
                "unmap(r{});",
                get(&RA, inst)
            )
        }
        Some(Opcode::Out) => {
            format!(
                "out(r{});",
                get(&RA, inst)
            )
        }
        Some(Opcode::In) => {
            format!(
                "r{} := in();",
                get(&RA, inst)
            )
        }
        Some(Opcode::LoadP) => {
            format!(
                "r{} := M[r{} + r{}];",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::LoadV) => {
            format!(
                "r{} := {};",
                get(&RL, inst),
                get(&VL, inst)
            )
        }
        None => {
            format!("unknown opcode")
        }
    }
}