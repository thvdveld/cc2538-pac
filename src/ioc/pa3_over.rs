#[doc = "Register `PA3_OVER` reader"]
pub type R = crate::R<Pa3OverSpec>;
#[doc = "Register `PA3_OVER` writer"]
pub type W = crate::W<Pa3OverSpec>;
#[doc = "Field `PA3_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa3OverR = crate::FieldReader;
#[doc = "Field `PA3_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa3OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa3_over(&self) -> Pa3OverR {
        Pa3OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa3_over(&mut self) -> Pa3OverW<Pa3OverSpec> {
        Pa3OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa3_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa3_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa3OverSpec;
impl crate::RegisterSpec for Pa3OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa3_over::R`](R) reader structure"]
impl crate::Readable for Pa3OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pa3_over::W`](W) writer structure"]
impl crate::Writable for Pa3OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA3_OVER to value 0x04"]
impl crate::Resettable for Pa3OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
