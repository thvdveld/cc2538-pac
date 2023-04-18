#[doc = "Register `FRMH` reader"]
pub struct R(crate::R<FRMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAMEH` reader - Bits 10:8 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
pub type FRAMEH_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Bits 10:8 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
    #[inline(always)]
    pub fn frameh(&self) -> FRAMEH_R {
        FRAMEH_R::new((self.bits & 7) as u8)
    }
}
#[doc = "Frame number (high byte)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmh](index.html) module"]
pub struct FRMH_SPEC;
impl crate::RegisterSpec for FRMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frmh::R](R) reader structure"]
impl crate::Readable for FRMH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRMH to value 0"]
impl crate::Resettable for FRMH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
