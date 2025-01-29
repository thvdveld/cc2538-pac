#[doc = "Register `PA7_OVER` reader"]
pub type R = crate::R<Pa7OverSpec>;
#[doc = "Register `PA7_OVER` writer"]
pub type W = crate::W<Pa7OverSpec>;
#[doc = "Field `PA7_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa7OverR = crate::FieldReader;
#[doc = "Field `PA7_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa7OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa7_over(&self) -> Pa7OverR {
        Pa7OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa7_over(&mut self) -> Pa7OverW<Pa7OverSpec> {
        Pa7OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa7_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa7_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa7OverSpec;
impl crate::RegisterSpec for Pa7OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa7_over::R`](R) reader structure"]
impl crate::Readable for Pa7OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pa7_over::W`](W) writer structure"]
impl crate::Writable for Pa7OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA7_OVER to value 0x04"]
impl crate::Resettable for Pa7OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
