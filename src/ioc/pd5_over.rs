#[doc = "Register `PD5_OVER` reader"]
pub type R = crate::R<Pd5OverSpec>;
#[doc = "Register `PD5_OVER` writer"]
pub type W = crate::W<Pd5OverSpec>;
#[doc = "Field `PD5_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pd5OverR = crate::FieldReader;
#[doc = "Field `PD5_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pd5OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd5_over(&self) -> Pd5OverR {
        Pd5OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd5_over(&mut self) -> Pd5OverW<Pd5OverSpec> {
        Pd5OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pd5_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd5_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd5OverSpec;
impl crate::RegisterSpec for Pd5OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd5_over::R`](R) reader structure"]
impl crate::Readable for Pd5OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pd5_over::W`](W) writer structure"]
impl crate::Writable for Pd5OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD5_OVER to value 0x04"]
impl crate::Resettable for Pd5OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
