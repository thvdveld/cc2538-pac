#[doc = "Register `TBPV` reader"]
pub struct R(crate::R<TBPV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PSV` reader - GPTM Timer B prescaler value"]
pub struct PSV_R(crate::FieldReader<u16, u16>);
impl PSV_R {
    pub(crate) fn new(bits: u16) -> Self {
        PSV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B prescaler value"]
    #[inline(always)]
    pub fn psv(&self) -> PSV_R {
        PSV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPTM Timer B prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer B prescaler in the 32-bit modes. Software can use this value in conjunction with the TBV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpv](index.html) module"]
pub struct TBPV_SPEC;
impl crate::RegisterSpec for TBPV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpv::R](R) reader structure"]
impl crate::Readable for TBPV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TBPV to value 0"]
impl crate::Resettable for TBPV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
