#[doc = "Register `DIVMSW` reader"]
pub struct R(crate::R<DIVMSW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIVMSW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIVMSW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIVMSW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RESULT_IS_ZERO` reader - The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
pub struct RESULT_IS_ZERO_R(crate::FieldReader<bool, bool>);
impl RESULT_IS_ZERO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESULT_IS_ZERO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_IS_ZERO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSW_ADDRESS` reader - Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
pub struct MSW_ADDRESS_R(crate::FieldReader<u16, u16>);
impl MSW_ADDRESS_R {
    pub(crate) fn new(bits: u16) -> Self {
        MSW_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSW_ADDRESS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 15 - The result vector is all zeroes, ignore the address returned in bits \\[10:0\\]"]
    #[inline(always)]
    pub fn result_is_zero(&self) -> RESULT_IS_ZERO_R {
        RESULT_IS_ZERO_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Address of the most significant nonzero 32-bit word of the remainder result vector in PKA RAM"]
    #[inline(always)]
    pub fn msw_address(&self) -> MSW_ADDRESS_R {
        MSW_ADDRESS_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "PKA most-significant-word of divide remainder This register indicates the (32-bit word) address in the PKA RAM where the most significant nonzero 32-bit word of the remainder result for the basic divide and modulo operations is stored. Bits \\[4:0\\]
are loaded with the bit number of the most-significant nonzero bit in the most-significant nonzero word when MS one control bit is set. For divide, modulo, and MS one reporting, this register is updated when the RUN bit in the PKA_FUNCTION register is reset at the end of the operation. For the complex sequencer controlled operations, updating of bits \\[4:0\\]
of this register with the most-significant bit location of the actual result is done near the end of the operation. The result is meaningful only if no errors were detected and that for ECC operations; the PKA_DIVMSW register provides information for the x-coordinate of the result point only.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divmsw](index.html) module"]
pub struct DIVMSW_SPEC;
impl crate::RegisterSpec for DIVMSW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [divmsw::R](R) reader structure"]
impl crate::Readable for DIVMSW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIVMSW to value 0"]
impl crate::Resettable for DIVMSW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
