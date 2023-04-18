#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIS` reader - Masked value of interrupt due to corresponding pin Bits clear: GPIO line interrupt not active Bits set: GPIO line asserting interrupt"]
pub type MIS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Masked value of interrupt due to corresponding pin Bits clear: GPIO line interrupt not active Bits set: GPIO line asserting interrupt"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "The MIS register is the masked interrupt status register. Bits read high in MIS reflect the status of input lines triggering an interrupt. Bits read as low indicate that either no interrupt has been generated, or the interrupt is masked. MIS is the state of the interrupt after masking.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
