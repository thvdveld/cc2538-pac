#[doc = "Register `PB3_OVER` reader"]
pub type R = crate::R<Pb3OverSpec>;
#[doc = "Register `PB3_OVER` writer"]
pub type W = crate::W<Pb3OverSpec>;
#[doc = "Field `PB3_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb3OverR = crate::FieldReader;
#[doc = "Field `PB3_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb3OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb3_over(&self) -> Pb3OverR {
        Pb3OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pb3_over(&mut self) -> Pb3OverW<Pb3OverSpec> {
        Pb3OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb3_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb3_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb3OverSpec;
impl crate::RegisterSpec for Pb3OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb3_over::R`](R) reader structure"]
impl crate::Readable for Pb3OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pb3_over::W`](W) writer structure"]
impl crate::Writable for Pb3OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB3_OVER to value 0x04"]
impl crate::Resettable for Pb3OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
