#[doc = "Register `DIVMSW` reader"]
pub type R = crate::R<DivmswSpec>;
#[doc = "Field `MSW_ADDRESS` reader - Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
pub type MswAddressR = crate::FieldReader<u16>;
#[doc = "Field `RESULT_IS_ZERO` reader - The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
pub type ResultIsZeroR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
    #[inline(always)]
    pub fn msw_address(&self) -> MswAddressR {
        MswAddressR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
    #[inline(always)]
    pub fn result_is_zero(&self) -> ResultIsZeroR {
        ResultIsZeroR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when the RUN bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; the PKA_DIVMSW register provides information for the x-coordinate of the result point only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divmsw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivmswSpec;
impl crate::RegisterSpec for DivmswSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divmsw::R`](R) reader structure"]
impl crate::Readable for DivmswSpec {}
#[doc = "`reset()` method sets DIVMSW to value 0"]
impl crate::Resettable for DivmswSpec {
    const RESET_VALUE: u32 = 0;
}
