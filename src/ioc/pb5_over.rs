#[doc = "Register `PB5_OVER` reader"]
pub type R = crate::R<Pb5OverSpec>;
#[doc = "Register `PB5_OVER` writer"]
pub type W = crate::W<Pb5OverSpec>;
#[doc = "Field `PB5_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb5OverR = crate::FieldReader;
#[doc = "Field `PB5_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pb5OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb5_over(&self) -> Pb5OverR {
        Pb5OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pb5_over(&mut self) -> Pb5OverW<Pb5OverSpec> {
        Pb5OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pb5_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb5_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pb5OverSpec;
impl crate::RegisterSpec for Pb5OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb5_over::R`](R) reader structure"]
impl crate::Readable for Pb5OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pb5_over::W`](W) writer structure"]
impl crate::Writable for Pb5OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB5_OVER to value 0x04"]
impl crate::Resettable for Pb5OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
