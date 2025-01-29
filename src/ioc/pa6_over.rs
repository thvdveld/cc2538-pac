#[doc = "Register `PA6_OVER` reader"]
pub type R = crate::R<Pa6OverSpec>;
#[doc = "Register `PA6_OVER` writer"]
pub type W = crate::W<Pa6OverSpec>;
#[doc = "Field `PA6_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa6OverR = crate::FieldReader;
#[doc = "Field `PA6_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pa6OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa6_over(&self) -> Pa6OverR {
        Pa6OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa6_over(&mut self) -> Pa6OverW<Pa6OverSpec> {
        Pa6OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pa6_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa6_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pa6OverSpec;
impl crate::RegisterSpec for Pa6OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa6_over::R`](R) reader structure"]
impl crate::Readable for Pa6OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pa6_over::W`](W) writer structure"]
impl crate::Writable for Pa6OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA6_OVER to value 0x04"]
impl crate::Resettable for Pa6OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
