#[doc = "Register `TAPV` reader"]
pub type R = crate::R<TapvSpec>;
#[doc = "Field `PSV` reader - GPTM Timer A prescaler value"]
pub type PsvR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer A prescaler value"]
    #[inline(always)]
    pub fn psv(&self) -> PsvR {
        PsvR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPTM Timer A prescale value For the 32-bit wide GPTM, this register shows the current free-running value of the Timer A prescaler in the 32-bit modes. Software can use this value in conjunction with the TAV register to determine the time elapsed between an interrupt and the ISR entry. This register is ununsed in 16- or 32-bit GPTM mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`tapv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TapvSpec;
impl crate::RegisterSpec for TapvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapv::R`](R) reader structure"]
impl crate::Readable for TapvSpec {}
#[doc = "`reset()` method sets TAPV to value 0"]
impl crate::Resettable for TapvSpec {
    const RESET_VALUE: u32 = 0;
}
