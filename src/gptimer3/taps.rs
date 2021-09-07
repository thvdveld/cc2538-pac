#[doc = "Register `TAPS` reader"]
pub struct R(crate::R<TAPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PSS` reader - GPTM Timer A prescaler"]
pub struct PSS_R(crate::FieldReader<u16, u16>);
impl PSS_R {
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
    #[doc = "Bits 0:15 - GPTM Timer A prescaler"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [taps](index.html) module"]
pub struct TAPS_SPEC;
impl crate::RegisterSpec for TAPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [taps::R](R) reader structure"]
impl crate::Readable for TAPS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TAPS to value 0"]
impl crate::Resettable for TAPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
