#[doc = "Register `PC4_OVER` reader"]
pub type R = crate::R<Pc4OverSpec>;
#[doc = "Register `PC4_OVER` writer"]
pub type W = crate::W<Pc4OverSpec>;
#[doc = "Field `PC4_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pc4OverR = crate::FieldReader;
#[doc = "Field `PC4_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pc4OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pc4_over(&self) -> Pc4OverR {
        Pc4OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pc4_over(&mut self) -> Pc4OverW<Pc4OverSpec> {
        Pc4OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc4_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc4_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc4OverSpec;
impl crate::RegisterSpec for Pc4OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc4_over::R`](R) reader structure"]
impl crate::Readable for Pc4OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pc4_over::W`](W) writer structure"]
impl crate::Writable for Pc4OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC4_OVER to value 0x04"]
impl crate::Resettable for Pc4OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
