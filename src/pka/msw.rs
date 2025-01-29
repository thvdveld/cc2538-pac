#[doc = "Register `MSW` reader"]
pub type R = crate::R<MswSpec>;
#[doc = "Field `MSW_ADDRESS` reader - Address of the most-significant nonzero 32-bit word of the result vector in PKA RAM"]
pub type MswAddressR = crate::FieldReader<u16>;
#[doc = "Field `RESULT_IS_ZERO` reader - The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
pub type ResultIsZeroR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Address of the most-significant nonzero 32-bit word of the result vector in PKA RAM"]
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
#[doc = "PKA most-significant-word of result vector This register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated when the run bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, the PKA_MSW register will provide information for the x-coordinate of the result point only.\n\nYou can [`read`](crate::Reg::read) this register and get [`msw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MswSpec;
impl crate::RegisterSpec for MswSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msw::R`](R) reader structure"]
impl crate::Readable for MswSpec {}
#[doc = "`reset()` method sets MSW to value 0"]
impl crate::Resettable for MswSpec {
    const RESET_VALUE: u32 = 0;
}
