#[doc = "Register `TBPS` reader"]
pub struct R(crate::R<TBPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PSS` reader - GPTM Timer B prescaler"]
pub struct PSS_R(crate::FieldReader<u16, u16>);
impl PSS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B prescaler"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbps](index.html) module"]
pub struct TBPS_SPEC;
impl crate::RegisterSpec for TBPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbps::R](R) reader structure"]
impl crate::Readable for TBPS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBPS to value 0"]
impl crate::Resettable for TBPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
