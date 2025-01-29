#[doc = "Register `PB7_OVER` reader"]
pub type R = crate::R<Pb7OverSpec>;
#[doc = "Register `PB7_OVER` writer"]
pub type W = crate::W<Pb7OverSpec>;
#[doc = "Field `PB7_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb7OverR = crate::FieldReader;
#[doc = "Field `PB7_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb7OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb7_over(&self) -> Pb7OverR {
        Pb7OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb7_over(&mut self) -> Pb7OverW<Pb7OverSpec> {
        Pb7OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb7_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb7_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb7OverSpec;
impl crate::RegisterSpec for Pb7OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb7_over::R`](R) reader structure"]
impl crate::Readable for Pb7OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pb7_over::W`](W) writer structure"]
impl crate::Writable for Pb7OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB7_OVER to value 0x04"]
impl crate::Resettable for Pb7OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
