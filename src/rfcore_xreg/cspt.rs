#[doc = "Register `CSPT` reader"]
pub struct R(crate::R<CSPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CSPT` reader - Content is decremented each time the MAC Timer overflows while the CSP program is running. The SCP program stops when decremented to 0. Setting CSPT = 0xFF prevents the register from being decremented."]
pub struct CSPT_R(crate::FieldReader<u8, u8>);
impl CSPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CSPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSPT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Content is decremented each time the MAC Timer overflows while the CSP program is running. The SCP program stops when decremented to 0. Setting CSPT = 0xFF prevents the register from being decremented."]
    #[inline(always)]
    pub fn cspt(&self) -> CSPT_R {
        CSPT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP T data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cspt](index.html) module"]
pub struct CSPT_SPEC;
impl crate::RegisterSpec for CSPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cspt::R](R) reader structure"]
impl crate::Readable for CSPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSPT to value 0"]
impl crate::Resettable for CSPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
