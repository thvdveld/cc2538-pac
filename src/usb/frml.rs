#[doc = "Register `FRML` reader"]
pub struct R(crate::R<FRML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRAMEL` reader - Bits 7:0 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
pub struct FRAMEL_R(crate::FieldReader<u8, u8>);
impl FRAMEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRAMEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAMEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Bits 7:0 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
    #[inline(always)]
    pub fn framel(&self) -> FRAMEL_R {
        FRAMEL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Frame number (low byte)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frml](index.html) module"]
pub struct FRML_SPEC;
impl crate::RegisterSpec for FRML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frml::R](R) reader structure"]
impl crate::Readable for FRML_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRML to value 0"]
impl crate::Resettable for FRML_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
