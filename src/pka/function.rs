#[doc = "Register `FUNCTION` reader"]
pub struct R(crate::R<FUNCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNCTION` writer"]
pub struct W(crate::W<FUNCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FUNCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STALL_RESULT` reader - When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
pub type STALL_RESULT_R = crate::BitReader<bool>;
#[doc = "Field `STALL_RESULT` writer - When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
pub type STALL_RESULT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `RUN` reader - The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
pub type RUN_R = crate::BitReader<bool>;
#[doc = "Field `RUN` writer - The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
pub type RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `SEQUENCER_OPERATIONS` reader - These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
pub type SEQUENCER_OPERATIONS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQUENCER_OPERATIONS` writer - These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
pub type SEQUENCER_OPERATIONS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FUNCTION_SPEC, u8, u8, 3, O>;
#[doc = "Field `COPY` reader - Perform copy operation"]
pub type COPY_R = crate::BitReader<bool>;
#[doc = "Field `COPY` writer - Perform copy operation"]
pub type COPY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `COMPARE` reader - Perform compare operation"]
pub type COMPARE_R = crate::BitReader<bool>;
#[doc = "Field `COMPARE` writer - Perform compare operation"]
pub type COMPARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `MODULO` reader - Perform modulo operation"]
pub type MODULO_R = crate::BitReader<bool>;
#[doc = "Field `MODULO` writer - Perform modulo operation"]
pub type MODULO_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `DIVIDE` reader - Perform divide operation"]
pub type DIVIDE_R = crate::BitReader<bool>;
#[doc = "Field `DIVIDE` writer - Perform divide operation"]
pub type DIVIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `LSHIFT` reader - Perform left shift operation"]
pub type LSHIFT_R = crate::BitReader<bool>;
#[doc = "Field `LSHIFT` writer - Perform left shift operation"]
pub type LSHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `RSHIFT` reader - Perform right shift operation"]
pub type RSHIFT_R = crate::BitReader<bool>;
#[doc = "Field `RSHIFT` writer - Perform right shift operation"]
pub type RSHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `SUBTRACT` reader - Perform subtract operation"]
pub type SUBTRACT_R = crate::BitReader<bool>;
#[doc = "Field `SUBTRACT` writer - Perform subtract operation"]
pub type SUBTRACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `ADD` reader - Perform add operation"]
pub type ADD_R = crate::BitReader<bool>;
#[doc = "Field `ADD` writer - Perform add operation"]
pub type ADD_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `MS_ONE` reader - Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\]
of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
pub type MS_ONE_R = crate::BitReader<bool>;
#[doc = "Field `MS_ONE` writer - Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\]
of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
pub type MS_ONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `ADDSUB` reader - Perform combined add/subtract operation"]
pub type ADDSUB_R = crate::BitReader<bool>;
#[doc = "Field `ADDSUB` writer - Perform combined add/subtract operation"]
pub type ADDSUB_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
#[doc = "Field `MULTIPLY` reader - Perform multiply operation"]
pub type MULTIPLY_R = crate::BitReader<bool>;
#[doc = "Field `MULTIPLY` writer - Perform multiply operation"]
pub type MULTIPLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, FUNCTION_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
    #[inline(always)]
    pub fn stall_result(&self) -> STALL_RESULT_R {
        STALL_RESULT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 15 - The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 12:14 - These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
    #[inline(always)]
    pub fn sequencer_operations(&self) -> SEQUENCER_OPERATIONS_R {
        SEQUENCER_OPERATIONS_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 11 - Perform copy operation"]
    #[inline(always)]
    pub fn copy(&self) -> COPY_R {
        COPY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Perform compare operation"]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Perform modulo operation"]
    #[inline(always)]
    pub fn modulo(&self) -> MODULO_R {
        MODULO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Perform divide operation"]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Perform left shift operation"]
    #[inline(always)]
    pub fn lshift(&self) -> LSHIFT_R {
        LSHIFT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Perform right shift operation"]
    #[inline(always)]
    pub fn rshift(&self) -> RSHIFT_R {
        RSHIFT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Perform subtract operation"]
    #[inline(always)]
    pub fn subtract(&self) -> SUBTRACT_R {
        SUBTRACT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Perform add operation"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\]
of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
    #[inline(always)]
    pub fn ms_one(&self) -> MS_ONE_R {
        MS_ONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 1 - Perform combined add/subtract operation"]
    #[inline(always)]
    pub fn addsub(&self) -> ADDSUB_R {
        ADDSUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Perform multiply operation"]
    #[inline(always)]
    pub fn multiply(&self) -> MULTIPLY_R {
        MULTIPLY_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - When written with a 1b, updating of the PKA_COMPARE, PKA_MSW and PKA_DIVMSW registers, as well as resetting the run bit is stalled beyond the point that a running operation is actually finished. Use this to allow software enough time to read results from a previous operation when the newly started operation is known to take only a short amount of time. If a result is waiting, the result registers is updated and the run bit is reset in the clock cycle following writing the stall result bit back to 0b. The Stall result function may only be used for basic PKCP operations."]
    #[inline(always)]
    pub fn stall_result(&mut self) -> STALL_RESULT_W<24> {
        STALL_RESULT_W::new(self)
    }
    #[doc = "Bit 15 - The host sets this bit to instruct the PKA module to begin processing the basic PKCP or complex sequencer operation. This bit is reset low automatically when the operation is complete. The complement of this bit is output as interrupts\\[1\\]. After a reset, the run bit is always set to 1b. Depending on the option, program ROM or program RAM, the following applies: Program ROM - The first sequencer instruction sets the bit to 0b. This is done immediately after the hardware reset is released. Program RAM - The sequencer must set the bit to 0b. As a valid firmware may not have been loaded, the sequencer is held in software reset after the hardware reset is released (the reset bit in PKA_SEQ_CRTL is set to 1b). After the FW image is loaded and the Reset bit is cleared, the sequencer starts to execute the FW. The first instruction clears the run bit. In both cases a few clock cycles are needed before the first instruction is executed and the run bit state has been propagated."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W<15> {
        RUN_W::new(self)
    }
    #[doc = "Bits 12:14 - These bits select the complex sequencer operation to perform: 000b: None 001b: ExpMod-CRT 010b: ExpMod-ACT4 (compatible with EIP2315) 011b: ECC-ADD (if available in firmware, otherwise reserved) 100b: ExpMod-ACT2 (compatible with EIP2316) 101b: ECC-MUL (if available in firmware, otherwise reserved) 110b: ExpMod-variable 111b: ModInv (if available in firmware, otherwise reserved) The encoding of these operations is determined by sequencer firmware."]
    #[inline(always)]
    pub fn sequencer_operations(&mut self) -> SEQUENCER_OPERATIONS_W<12> {
        SEQUENCER_OPERATIONS_W::new(self)
    }
    #[doc = "Bit 11 - Perform copy operation"]
    #[inline(always)]
    pub fn copy(&mut self) -> COPY_W<11> {
        COPY_W::new(self)
    }
    #[doc = "Bit 10 - Perform compare operation"]
    #[inline(always)]
    pub fn compare(&mut self) -> COMPARE_W<10> {
        COMPARE_W::new(self)
    }
    #[doc = "Bit 9 - Perform modulo operation"]
    #[inline(always)]
    pub fn modulo(&mut self) -> MODULO_W<9> {
        MODULO_W::new(self)
    }
    #[doc = "Bit 8 - Perform divide operation"]
    #[inline(always)]
    pub fn divide(&mut self) -> DIVIDE_W<8> {
        DIVIDE_W::new(self)
    }
    #[doc = "Bit 7 - Perform left shift operation"]
    #[inline(always)]
    pub fn lshift(&mut self) -> LSHIFT_W<7> {
        LSHIFT_W::new(self)
    }
    #[doc = "Bit 6 - Perform right shift operation"]
    #[inline(always)]
    pub fn rshift(&mut self) -> RSHIFT_W<6> {
        RSHIFT_W::new(self)
    }
    #[doc = "Bit 5 - Perform subtract operation"]
    #[inline(always)]
    pub fn subtract(&mut self) -> SUBTRACT_W<5> {
        SUBTRACT_W::new(self)
    }
    #[doc = "Bit 4 - Perform add operation"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<4> {
        ADD_W::new(self)
    }
    #[doc = "Bit 3 - Loads the location of the Most Significant one bit within the result word indicated in the PKA_MSW register into bits \\[4:0\\]
of the PKA_DIVMSW register - can only be used with basic PKCP operations, except for Divide, Modulo and Compare."]
    #[inline(always)]
    pub fn ms_one(&mut self) -> MS_ONE_W<3> {
        MS_ONE_W::new(self)
    }
    #[doc = "Bit 1 - Perform combined add/subtract operation"]
    #[inline(always)]
    pub fn addsub(&mut self) -> ADDSUB_W<1> {
        ADDSUB_W::new(self)
    }
    #[doc = "Bit 0 - Perform multiply operation"]
    #[inline(always)]
    pub fn multiply(&mut self) -> MULTIPLY_W<0> {
        MULTIPLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA function This register contains the control bits to start basic PKCP as well as complex sequencer operations. The run bit can be used to poll for the completion of the operation. Modifying bits \\[11:0\\]
is made impossible during the execution of a basic PKCP operation. During the execution of sequencer-controlled complex operations, this register is modified; the run and stall result bits are set to zero at the conclusion, but other bits are undefined. Attention: Continuously reading this register to poll the run bit is not allowed when executing complex sequencer operations (the sequencer cannot access the PKCP when this is done). Leave at least one sysclk cycle between poll operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [function](index.html) module"]
pub struct FUNCTION_SPEC;
impl crate::RegisterSpec for FUNCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [function::R](R) reader structure"]
impl crate::Readable for FUNCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [function::W](W) writer structure"]
impl crate::Writable for FUNCTION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNCTION to value 0"]
impl crate::Resettable for FUNCTION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
