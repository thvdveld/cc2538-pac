#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Field `IM` reader - Interrupt mask 1: The master interrupt is sent to the interrupt controller when the RIS bit in the I2CMRIS register is set. 0: The RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type IM_R = crate::BitReader;
#[doc = "Field `IM` writer - Interrupt mask 1: The master interrupt is sent to the interrupt controller when the RIS bit in the I2CMRIS register is set. 0: The RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type IM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interrupt mask 1: The master interrupt is sent to the interrupt controller when the RIS bit in the I2CMRIS register is set. 0: The RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt mask 1: The master interrupt is sent to the interrupt controller when the RIS bit in the I2CMRIS register is set. 0: The RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> IM_W<IMR_SPEC, 0> {
        IM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C master interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
