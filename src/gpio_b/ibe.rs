#[doc = "Register `IBE` reader"]
pub type R = crate::R<IBE_SPEC>;
#[doc = "Register `IBE` writer"]
pub type W = crate::W<IBE_SPEC>;
#[doc = "Field `IBE` reader - Bits set: Both edges on corresponding pin trigger an interrupt Bits cleared: Interrupt generation event is controlled by GPIOIEV Single edge: Determined by corresponding bit in GPIOIEV register"]
pub type IBE_R = crate::FieldReader;
#[doc = "Field `IBE` writer - Bits set: Both edges on corresponding pin trigger an interrupt Bits cleared: Interrupt generation event is controlled by GPIOIEV Single edge: Determined by corresponding bit in GPIOIEV register"]
pub type IBE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Both edges on corresponding pin trigger an interrupt Bits cleared: Interrupt generation event is controlled by GPIOIEV Single edge: Determined by corresponding bit in GPIOIEV register"]
    #[inline(always)]
    pub fn ibe(&self) -> IBE_R {
        IBE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Both edges on corresponding pin trigger an interrupt Bits cleared: Interrupt generation event is controlled by GPIOIEV Single edge: Determined by corresponding bit in GPIOIEV register"]
    #[inline(always)]
    #[must_use]
    pub fn ibe(&mut self) -> IBE_W<IBE_SPEC, 0> {
        IBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The IBE register is the interrupt both-edges register. When the corresponding bit in IS is set to detect edges, bits set to high in IBE configure the corresponding pin to detect both rising and falling edges, regardless of the corresponding bit in the IEV (interrupt event register). Clearing a bit configures the pin to be controlled by IEV.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBE_SPEC;
impl crate::RegisterSpec for IBE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibe::R`](R) reader structure"]
impl crate::Readable for IBE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibe::W`](W) writer structure"]
impl crate::Writable for IBE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBE to value 0"]
impl crate::Resettable for IBE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
