#[doc = "Register `FIFOPCTRL` reader"]
pub type R = crate::R<FifopctrlSpec>;
#[doc = "Register `FIFOPCTRL` writer"]
pub type W = crate::W<FifopctrlSpec>;
#[doc = "Field `FIFOP_THR` reader - Threshold used when generating FIFOP signal"]
pub type FifopThrR = crate::FieldReader;
#[doc = "Field `FIFOP_THR` writer - Threshold used when generating FIFOP signal"]
pub type FifopThrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Threshold used when generating FIFOP signal"]
    #[inline(always)]
    pub fn fifop_thr(&self) -> FifopThrR {
        FifopThrR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Threshold used when generating FIFOP signal"]
    #[inline(always)]
    #[must_use]
    pub fn fifop_thr(&mut self) -> FifopThrW<FifopctrlSpec> {
        FifopThrW::new(self, 0)
    }
}
#[doc = "FIFOP threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifopctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifopctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifopctrlSpec;
impl crate::RegisterSpec for FifopctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifopctrl::R`](R) reader structure"]
impl crate::Readable for FifopctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fifopctrl::W`](W) writer structure"]
impl crate::Writable for FifopctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOPCTRL to value 0"]
impl crate::Resettable for FifopctrlSpec {
    const RESET_VALUE: u32 = 0;
}
