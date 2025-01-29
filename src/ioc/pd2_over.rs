#[doc = "Register `PD2_OVER` reader"]
pub type R = crate::R<Pd2OverSpec>;
#[doc = "Register `PD2_OVER` writer"]
pub type W = crate::W<Pd2OverSpec>;
#[doc = "Field `PD2_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pd2OverR = crate::FieldReader;
#[doc = "Field `PD2_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pd2OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd2_over(&self) -> Pd2OverR {
        Pd2OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd2_over(&mut self) -> Pd2OverW<Pd2OverSpec> {
        Pd2OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd2_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd2_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd2OverSpec;
impl crate::RegisterSpec for Pd2OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd2_over::R`](R) reader structure"]
impl crate::Readable for Pd2OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pd2_over::W`](W) writer structure"]
impl crate::Writable for Pd2OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD2_OVER to value 0x04"]
impl crate::Resettable for Pd2OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
