#[doc = "Register `IEV` reader"]
pub type R = crate::R<IEV_SPEC>;
#[doc = "Register `IEV` writer"]
pub type W = crate::W<IEV_SPEC>;
#[doc = "Field `IEV` reader - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
pub type IEV_R = crate::FieldReader;
#[doc = "Field `IEV` writer - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
pub type IEV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
    #[inline(always)]
    pub fn iev(&self) -> IEV_R {
        IEV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Rising edges or high levels on corresponding pin trigger interrupts Bits cleared: Falling edges or low levels on corresponding pin trigger interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn iev(&mut self) -> IEV_W<IEV_SPEC, 0> {
        IEV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The IEV register is the interrupt event register. Bits set to high in IEV configure the corresponding pin to detect rising edges or high levels, depending on the corresponding bit value in IS. Clearing a bit configures the pin to detect falling edges or low levels, depending on the corresponding bit value in IS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iev::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iev::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IEV_SPEC;
impl crate::RegisterSpec for IEV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iev::R`](R) reader structure"]
impl crate::Readable for IEV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iev::W`](W) writer structure"]
impl crate::Writable for IEV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEV to value 0"]
impl crate::Resettable for IEV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
