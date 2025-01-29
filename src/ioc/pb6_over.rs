#[doc = "Register `PB6_OVER` reader"]
pub type R = crate::R<Pb6OverSpec>;
#[doc = "Register `PB6_OVER` writer"]
pub type W = crate::W<Pb6OverSpec>;
#[doc = "Field `PB6_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb6OverR = crate::FieldReader;
#[doc = "Field `PB6_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb6OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb6_over(&self) -> Pb6OverR {
        Pb6OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb6_over(&mut self) -> Pb6OverW<Pb6OverSpec> {
        Pb6OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb6_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb6_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb6OverSpec;
impl crate::RegisterSpec for Pb6OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb6_over::R`](R) reader structure"]
impl crate::Readable for Pb6OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pb6_over::W`](W) writer structure"]
impl crate::Writable for Pb6OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB6_OVER to value 0x04"]
impl crate::Resettable for Pb6OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
