#[doc = "Register `TAPR` reader"]
pub type R = crate::R<TaprSpec>;
#[doc = "Register `TAPR` writer"]
pub type W = crate::W<TaprSpec>;
#[doc = "Field `TAPSR` reader - GPTM Timer A prescale"]
pub type TapsrR = crate::FieldReader;
#[doc = "Field `TAPSR` writer - GPTM Timer A prescale"]
pub type TapsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer A prescale"]
    #[inline(always)]
    pub fn tapsr(&self) -> TapsrR {
        TapsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer A prescale"]
    #[inline(always)]
    #[must_use]
    pub fn tapsr(&mut self) -> TapsrW<TaprSpec> {
        TapsrW::new(self, 0)
    }
}
#[doc = "GPTM Timer A prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TaprSpec;
impl crate::RegisterSpec for TaprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapr::R`](R) reader structure"]
impl crate::Readable for TaprSpec {}
#[doc = "`write(|w| ..)` method takes [`tapr::W`](W) writer structure"]
impl crate::Writable for TaprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAPR to value 0"]
impl crate::Resettable for TaprSpec {
    const RESET_VALUE: u32 = 0;
}
