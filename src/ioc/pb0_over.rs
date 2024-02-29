#[doc = "Register `PB0_OVER` reader"]
pub type R = crate::R<Pb0OverSpec>;
#[doc = "Register `PB0_OVER` writer"]
pub type W = crate::W<Pb0OverSpec>;
#[doc = "Field `PB0_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb0OverR = crate::FieldReader;
#[doc = "Field `PB0_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb0OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb0_over(&self) -> Pb0OverR {
        Pb0OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb0_over(&mut self) -> Pb0OverW<Pb0OverSpec> {
        Pb0OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb0_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb0_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb0OverSpec;
impl crate::RegisterSpec for Pb0OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb0_over::R`](R) reader structure"]
impl crate::Readable for Pb0OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pb0_over::W`](W) writer structure"]
impl crate::Writable for Pb0OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB0_OVER to value 0x04"]
impl crate::Resettable for Pb0OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
