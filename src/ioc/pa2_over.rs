#[doc = "Register `PA2_OVER` reader"]
pub type R = crate::R<Pa2OverSpec>;
#[doc = "Register `PA2_OVER` writer"]
pub type W = crate::W<Pa2OverSpec>;
#[doc = "Field `PA2_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa2OverR = crate::FieldReader;
#[doc = "Field `PA2_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa2OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa2_over(&self) -> Pa2OverR {
        Pa2OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pa2_over(&mut self) -> Pa2OverW<Pa2OverSpec> {
        Pa2OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa2_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa2_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa2OverSpec;
impl crate::RegisterSpec for Pa2OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa2_over::R`](R) reader structure"]
impl crate::Readable for Pa2OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pa2_over::W`](W) writer structure"]
impl crate::Writable for Pa2OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA2_OVER to value 0x04"]
impl crate::Resettable for Pa2OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
