#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `IM` reader - Interrupt mask 1: The master interrupt is sent to the interrupt controller when the RIS bit in the I2CMRIS register is set. 0: The RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type ImR = crate::BitReader;
#[doc = "Field `IM` writer - Interrupt mask 1: The master interrupt is sent to the interrupt controller when the RIS bit in the I2CMRIS register is set. 0: The RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type ImW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt mask 1: The master interrupt is sent to the interrupt controller when the RIS bit in the I2CMRIS register is set. 0: The RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn im(&self) -> ImR {
        ImR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt mask 1: The master interrupt is sent to the interrupt controller when the RIS bit in the I2CMRIS register is set. 0: The RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> ImW<ImrSpec> {
        ImW::new(self, 0)
    }
}
#[doc = "I2C master interrupt mask This register controls whether a raw interrupt is promoted to a controller interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
