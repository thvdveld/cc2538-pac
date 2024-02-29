#[doc = "Register `IEV` reader"]
pub type R = crate::R<IevSpec>;
#[doc = "Register `IEV` writer"]
pub type W = crate::W<IevSpec>;
#[doc = "Field `IEV` reader - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
pub type IevR = crate::FieldReader;
#[doc = "Field `IEV` writer - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
pub type IevW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
    #[inline(always)]
    pub fn iev(&self) -> IevR {
        IevR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn iev(&mut self) -> IevW<IevSpec> {
        IevW::new(self, 0)
    }
}
#[doc = "The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IevSpec;
impl crate::RegisterSpec for IevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iev::R`](R) reader structure"]
impl crate::Readable for IevSpec {}
#[doc = "`write(|w| ..)` method takes [`iev::W`](W) writer structure"]
impl crate::Writable for IevSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEV to value 0"]
impl crate::Resettable for IevSpec {
    const RESET_VALUE: u32 = 0;
}
