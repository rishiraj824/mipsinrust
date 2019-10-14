use std::collections::{BitSet, BitVec};


#[derive(Debug)]
struct RF {
    Registers: BitSet,
    ReadData1: Bitset,
    ReadData2: Bitset
}

impl RF {
    // Read File
    fn RF() {
        Registers = BitSet::from_bytes(&[0b00000000]);
    }
    // Read and Write
    fn ReadWrite(&self. RdReg1: BitSet::with_capacity(5), RdReg2: BitSet::with_capacity(5), WrtReg: BitSet::with_capacity(5), WrtData: BitSet::with_capacity(32), WrtEnable: BitSet::with_capacity(1) ) {
        if(WrtEnable[0]) {
            self.Registers[WrtReg.]
        }
    }
}
fn main() {
    // instructions
    let mut pc = BitSet::with_capacity(32);
    let mut instruction = BitSet::with_capacity(32);
    let mut opcode = BitSet::with_capacity(6);
    let mut funct = BitSet::with_capacity(6);
    let mut imm = BitSet::with_capacity(16);
    
    //control signals
    let mut IType = BitSet::with_capacity(1);
    let mut JType = BitSet::with_capacity(1);
    let mut RType = BitSet::with_capacity(1);
    let mut IsBranch = BitSet::with_capacity(1);
    let mut IsStore = BitSet::with_capacity(1);
    let mut IsLoad = BitSet::with_capacity(1);
    let mut WrtEnable = BitSet::with_capacity(1);

    // RF signals
    let mut RReg1 = BitSet::with_capacity(5);
    let mut RReg2 = BitSet::with_capacity(5);
    let mut Wreg = BitSet::with_capacity(5);
    let mut WData = BitSet::with_capacity(32);
    
    // ALU signals
    let mut ALUop = BitSet::with_capacity(3);
    let mut ALUin1 = BitSet::with_capacity(32);
    let mut ALUin2 = BitSet::with_capacity(32);
    let mut signext = BitSet::with_capacity(32);
    let mut ALUOut = BitSet::with_capacity(32);

    // DMEM signals
    let mut DMAddr = BitSet::with_capacity(32);
    let mut WriteData = BitSet::with_capacity(32);
    let mut ReadMem = BitSet::with_capacity(1);
    let mut WriteMem = BitSet::with_capacity(1);

    // pc signals
    let mut pcplusfour = BitSet::with_capacity(32);
    let mut jaddr = BitSet::with_capacity(32);
    let mut baddr = BitSet::with_capacity(32);
    let mut IsEq = BitSet::with_capacity(1);


}
