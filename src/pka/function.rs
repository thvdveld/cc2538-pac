#[doc = "Register `FUNCTION` reader"]
pub type R = crate::R<FunctionSpec>;
#[doc = "Register `FUNCTION` writer"]
pub type W = crate::W<FunctionSpec>;
#[doc = "Field `MULTIPLY` reader - Perform multiply operation"]
pub type MultiplyR = crate::BitReader;
#[doc = "Field `MULTIPLY` writer - Perform multiply operation"]
pub type MultiplyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDSUB` reader - Perform combined add/subtract operation"]
pub type AddsubR = crate::BitReader;
#[doc = "Field `ADDSUB` writer - Perform combined add/subtract operation"]
pub type AddsubW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS_ONE` reader - Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\]
of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
pub type MsOneR = crate::BitReader;
#[doc = "Field `MS_ONE` writer - Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\]
of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
pub type MsOneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - Perform add operation"]
pub type AddR = crate::BitReader;
#[doc = "Field `ADD` writer - Perform add operation"]
pub type AddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUBTRACT` reader - Perform subtract operation"]
pub type SubtractR = crate::BitReader;
#[doc = "Field `SUBTRACT` writer - Perform subtract operation"]
pub type SubtractW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSHIFT` reader - Perform right shift operation"]
pub type RshiftR = crate::BitReader;
#[doc = "Field `RSHIFT` writer - Perform right shift operation"]
pub type RshiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSHIFT` reader - Perform left shift operation"]
pub type LshiftR = crate::BitReader;
#[doc = "Field `LSHIFT` writer - Perform left shift operation"]
pub type LshiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVIDE` reader - Perform divide operation"]
pub type DivideR = crate::BitReader;
#[doc = "Field `DIVIDE` writer - Perform divide operation"]
pub type DivideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODULO` reader - Perform modulo operation"]
pub type ModuloR = crate::BitReader;
#[doc = "Field `MODULO` writer - Perform modulo operation"]
pub type ModuloW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPARE` reader - Perform compare operation"]
pub type CompareR = crate::BitReader;
#[doc = "Field `COMPARE` writer - Perform compare operation"]
pub type CompareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COPY` reader - Perform copy operation"]
pub type CopyR = crate::BitReader;
#[doc = "Field `COPY` writer - Perform copy operation"]
pub type CopyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQUENCER_OPERATIONS` reader - These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
pub type SequencerOperationsR = crate::FieldReader;
#[doc = "Field `SEQUENCER_OPERATIONS` writer - These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
pub type SequencerOperationsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RUN` reader - The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
pub type RunR = crate::BitReader;
#[doc = "Field `RUN` writer - The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
pub type RunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL_RESULT` reader - When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
pub type StallResultR = crate::BitReader;
#[doc = "Field `STALL_RESULT` writer - When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
pub type StallResultW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Perform multiply operation"]
    #[inline(always)]
    pub fn multiply(&self) -> MultiplyR {
        MultiplyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Perform combined add/subtract operation"]
    #[inline(always)]
    pub fn addsub(&self) -> AddsubR {
        AddsubR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\]
of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
    #[inline(always)]
    pub fn ms_one(&self) -> MsOneR {
        MsOneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Perform add operation"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Perform subtract operation"]
    #[inline(always)]
    pub fn subtract(&self) -> SubtractR {
        SubtractR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Perform right shift operation"]
    #[inline(always)]
    pub fn rshift(&self) -> RshiftR {
        RshiftR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Perform left shift operation"]
    #[inline(always)]
    pub fn lshift(&self) -> LshiftR {
        LshiftR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Perform divide operation"]
    #[inline(always)]
    pub fn divide(&self) -> DivideR {
        DivideR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Perform modulo operation"]
    #[inline(always)]
    pub fn modulo(&self) -> ModuloR {
        ModuloR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Perform compare operation"]
    #[inline(always)]
    pub fn compare(&self) -> CompareR {
        CompareR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Perform copy operation"]
    #[inline(always)]
    pub fn copy(&self) -> CopyR {
        CopyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
    #[inline(always)]
    pub fn sequencer_operations(&self) -> SequencerOperationsR {
        SequencerOperationsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
    #[inline(always)]
    pub fn stall_result(&self) -> StallResultR {
        StallResultR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Perform multiply operation"]
    #[inline(always)]
    pub fn multiply(&mut self) -> MultiplyW<FunctionSpec> {
        MultiplyW::new(self, 0)
    }
    #[doc = "Bit 1 - Perform combined add/subtract operation"]
    #[inline(always)]
    pub fn addsub(&mut self) -> AddsubW<FunctionSpec> {
        AddsubW::new(self, 1)
    }
    #[doc = "Bit 3 - Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\]
of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
    #[inline(always)]
    pub fn ms_one(&mut self) -> MsOneW<FunctionSpec> {
        MsOneW::new(self, 3)
    }
    #[doc = "Bit 4 - Perform add operation"]
    #[inline(always)]
    pub fn add(&mut self) -> AddW<FunctionSpec> {
        AddW::new(self, 4)
    }
    #[doc = "Bit 5 - Perform subtract operation"]
    #[inline(always)]
    pub fn subtract(&mut self) -> SubtractW<FunctionSpec> {
        SubtractW::new(self, 5)
    }
    #[doc = "Bit 6 - Perform right shift operation"]
    #[inline(always)]
    pub fn rshift(&mut self) -> RshiftW<FunctionSpec> {
        RshiftW::new(self, 6)
    }
    #[doc = "Bit 7 - Perform left shift operation"]
    #[inline(always)]
    pub fn lshift(&mut self) -> LshiftW<FunctionSpec> {
        LshiftW::new(self, 7)
    }
    #[doc = "Bit 8 - Perform divide operation"]
    #[inline(always)]
    pub fn divide(&mut self) -> DivideW<FunctionSpec> {
        DivideW::new(self, 8)
    }
    #[doc = "Bit 9 - Perform modulo operation"]
    #[inline(always)]
    pub fn modulo(&mut self) -> ModuloW<FunctionSpec> {
        ModuloW::new(self, 9)
    }
    #[doc = "Bit 10 - Perform compare operation"]
    #[inline(always)]
    pub fn compare(&mut self) -> CompareW<FunctionSpec> {
        CompareW::new(self, 10)
    }
    #[doc = "Bit 11 - Perform copy operation"]
    #[inline(always)]
    pub fn copy(&mut self) -> CopyW<FunctionSpec> {
        CopyW::new(self, 11)
    }
    #[doc = "Bits 12:14 - These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
    #[inline(always)]
    pub fn sequencer_operations(&mut self) -> SequencerOperationsW<FunctionSpec> {
        SequencerOperationsW::new(self, 12)
    }
    #[doc = "Bit 15 - The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
    #[inline(always)]
    pub fn run(&mut self) -> RunW<FunctionSpec> {
        RunW::new(self, 15)
    }
    #[doc = "Bit 24 - When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
    #[inline(always)]
    pub fn stall_result(&mut self) -> StallResultW<FunctionSpec> {
        StallResultW::new(self, 24)
    }
}
#[doc = "PKA function This register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. During the execution of sequencer-controlled complex operations, this register is modified; the run and stall result bits are set to zero at the conclusion, but other bits are undefined. Attention: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations.\n\nYou can [`read`](crate::Reg::read) this register and get [`function::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`function::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FunctionSpec;
impl crate::RegisterSpec for FunctionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`function::R`](R) reader structure"]
impl crate::Readable for FunctionSpec {}
#[doc = "`write(|w| ..)` method takes [`function::W`](W) writer structure"]
impl crate::Writable for FunctionSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FUNCTION to value 0"]
impl crate::Resettable for FunctionSpec {
    const RESET_VALUE: u32 = 0;
}
