#[doc = "Register `PD3_OVER` reader"]
pub type R = crate::R<Pd3OverSpec>;
#[doc = "Register `PD3_OVER` writer"]
pub type W = crate::W<Pd3OverSpec>;
#[doc = "Field `PD3_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pd3OverR = crate::FieldReader;
#[doc = "Field `PD3_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pd3OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd3_over(&self) -> Pd3OverR {
        Pd3OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd3_over(&mut self) -> Pd3OverW<Pd3OverSpec> {
        Pd3OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd3_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd3_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd3OverSpec;
impl crate::RegisterSpec for Pd3OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd3_over::R`](R) reader structure"]
impl crate::Readable for Pd3OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pd3_over::W`](W) writer structure"]
impl crate::Writable for Pd3OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD3_OVER to value 0x04"]
impl crate::Resettable for Pd3OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
