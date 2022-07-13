#[doc = "Register `LTIM` reader"]
pub struct R(crate::R<LTIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER` reader - Timer value This field contains the value of the free-running timer."]
pub type TIMER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timer value This field contains the value of the free-running timer."]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "UART LIN timer The LTIM register contains the current timer value for the free-running timer that is used to calculate the baud rate when in LIN slave mode. The value in this register is used along with the value in the UART LIN snap shot (LSS) register to adjust the baud rate to match that of the master.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltim](index.html) module"]
pub struct LTIM_SPEC;
impl crate::RegisterSpec for LTIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltim::R](R) reader structure"]
impl crate::Readable for LTIM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LTIM to value 0"]
impl crate::Resettable for LTIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
