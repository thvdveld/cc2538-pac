#[doc = "Register `TAPS` reader"]
pub type R = crate::R<TAPS_SPEC>;
#[doc = "Field `PSS` reader - GPTM Timer A prescaler"]
pub type PSS_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A prescaler"]
    #[inline(always)]
    pub fn pss(&self) -> PSS_R {
        PSS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPTM Timer A prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer A prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`taps::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAPS_SPEC;
impl crate::RegisterSpec for TAPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`taps::R`](R) reader structure"]
impl crate::Readable for TAPS_SPEC {}
#[doc = "`reset()` method sets TAPS to value 0"]
impl crate::Resettable for TAPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
