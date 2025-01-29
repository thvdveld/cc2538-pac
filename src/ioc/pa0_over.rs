#[doc = "Register `PA0_OVER` reader"]
pub type R = crate::R<Pa0OverSpec>;
#[doc = "Register `PA0_OVER` writer"]
pub type W = crate::W<Pa0OverSpec>;
#[doc = "Field `PA0_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa0OverR = crate::FieldReader;
#[doc = "Field `PA0_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa0OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa0_over(&self) -> Pa0OverR {
        Pa0OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa0_over(&mut self) -> Pa0OverW<Pa0OverSpec> {
        Pa0OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa0_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa0_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa0OverSpec;
impl crate::RegisterSpec for Pa0OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa0_over::R`](R) reader structure"]
impl crate::Readable for Pa0OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pa0_over::W`](W) writer structure"]
impl crate::Writable for Pa0OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA0_OVER to value 0x04"]
impl crate::Resettable for Pa0OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
