#[doc = "Register `AGCCTRL1` reader"]
pub type R = crate::R<Agcctrl1Spec>;
#[doc = "Register `AGCCTRL1` writer"]
pub type W = crate::W<Agcctrl1Spec>;
#[doc = "Field `AGC_REF` reader - Target value for the AGC control loop, given in 1-dB steps"]
pub type AgcRefR = crate::FieldReader;
#[doc = "Field `AGC_REF` writer - Target value for the AGC control loop, given in 1-dB steps"]
pub type AgcRefW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Target value for the AGC control loop, given in 1-dB steps"]
    #[inline(always)]
    pub fn agc_ref(&self) -> AgcRefR {
        AgcRefR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Target value for the AGC control loop, given in 1-dB steps"]
    #[inline(always)]
    #[must_use]
    pub fn agc_ref(&mut self) -> AgcRefW<Agcctrl1Spec> {
        AgcRefW::new(self, 0)
    }
}
#[doc = "AGC reference level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`agcctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`agcctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Agcctrl1Spec;
impl crate::RegisterSpec for Agcctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`agcctrl1::R`](R) reader structure"]
impl crate::Readable for Agcctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`agcctrl1::W`](W) writer structure"]
impl crate::Writable for Agcctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGCCTRL1 to value 0"]
impl crate::Resettable for Agcctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
