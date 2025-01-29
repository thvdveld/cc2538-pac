#[doc = "Register `PB1_OVER` reader"]
pub type R = crate::R<Pb1OverSpec>;
#[doc = "Register `PB1_OVER` writer"]
pub type W = crate::W<Pb1OverSpec>;
#[doc = "Field `PB1_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb1OverR = crate::FieldReader;
#[doc = "Field `PB1_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb1OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb1_over(&self) -> Pb1OverR {
        Pb1OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb1_over(&mut self) -> Pb1OverW<Pb1OverSpec> {
        Pb1OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb1_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb1_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb1OverSpec;
impl crate::RegisterSpec for Pb1OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb1_over::R`](R) reader structure"]
impl crate::Readable for Pb1OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pb1_over::W`](W) writer structure"]
impl crate::Writable for Pb1OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB1_OVER to value 0x04"]
impl crate::Resettable for Pb1OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
