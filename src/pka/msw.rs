#[doc = "Register `MSW` reader"]
pub struct R(crate::R<MSW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT_IS_ZERO` reader - The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
pub type RESULT_IS_ZERO_R = crate::BitReader<bool>;
#[doc = "Field `MSW_ADDRESS` reader - Address of the most-significant nonzero 32-bit word of the result vector in PKA RAM"]
pub type MSW_ADDRESS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 15 - The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
    #[inline(always)]
    pub fn result_is_zero(&self) -> RESULT_IS_ZERO_R {
        RESULT_IS_ZERO_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 0:10 - Address of the most-significant nonzero 32-bit word of the result vector in PKA RAM"]
    #[inline(always)]
    pub fn msw_address(&self) -> MSW_ADDRESS_R {
        MSW_ADDRESS_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "PKA most-significant-word of result vector This register indicates the (word) address in the PKA RAM where the most significant nonzero 32-bit word of the result is stored. Should be ignored for modulo operations. For basic PKCP operations, this register is updated when the run bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex-sequencer controlled operations, updating of the final value matching the actual result is done near the end of the operation; note that the result is only meaningful if no errors were detected and that for ECC operations, the PKA_MSW register will provide information for the x-coordinate of the result point only.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msw](index.html) module"]
pub struct MSW_SPEC;
impl crate::RegisterSpec for MSW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msw::R](R) reader structure"]
impl crate::Readable for MSW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSW to value 0"]
impl crate::Resettable for MSW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
