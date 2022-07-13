#[doc = "Register `ALTBASE` reader"]
pub struct R(crate::R<ALTBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR` reader - Alternate channel address pointer This field provides the base address of the alternate channel control structures."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Alternate channel address pointer This field provides the base address of the alternate channel control structures."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
#[doc = "DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altbase](index.html) module"]
pub struct ALTBASE_SPEC;
impl crate::RegisterSpec for ALTBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altbase::R](R) reader structure"]
impl crate::Readable for ALTBASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ALTBASE to value 0"]
impl crate::Resettable for ALTBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
