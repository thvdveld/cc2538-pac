#[doc = "Register `TAPMR` reader"]
pub type R = crate::R<TapmrSpec>;
#[doc = "Register `TAPMR` writer"]
pub type W = crate::W<TapmrSpec>;
#[doc = "Field `TAPSR` reader - GPTM Timer A prescale match"]
pub type TapsrR = crate::FieldReader;
#[doc = "Field `TAPSR` writer - GPTM Timer A prescale match"]
pub type TapsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer A prescale match"]
    #[inline(always)]
    pub fn tapsr(&self) -> TapsrR {
        TapsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer A prescale match"]
    #[inline(always)]
    #[must_use]
    pub fn tapsr(&mut self) -> TapsrW<TapmrSpec> {
        TapsrW::new(self, 0)
    }
}
#[doc = "GPTM Timer A prescale match This register effectively extends the range of TAMATCHR to 24 bits when operating in 16-bit, one-shot or periodic mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tapmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tapmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TapmrSpec;
impl crate::RegisterSpec for TapmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tapmr::R`](R) reader structure"]
impl crate::Readable for TapmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tapmr::W`](W) writer structure"]
impl crate::Writable for TapmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAPMR to value 0"]
impl crate::Resettable for TapmrSpec {
    const RESET_VALUE: u32 = 0;
}
