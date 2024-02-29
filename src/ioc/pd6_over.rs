#[doc = "Register `PD6_OVER` reader"]
pub type R = crate::R<Pd6OverSpec>;
#[doc = "Register `PD6_OVER` writer"]
pub type W = crate::W<Pd6OverSpec>;
#[doc = "Field `PD6_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pd6OverR = crate::FieldReader;
#[doc = "Field `PD6_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pd6OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd6_over(&self) -> Pd6OverR {
        Pd6OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd6_over(&mut self) -> Pd6OverW<Pd6OverSpec> {
        Pd6OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd6_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd6_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pd6OverSpec;
impl crate::RegisterSpec for Pd6OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd6_over::R`](R) reader structure"]
impl crate::Readable for Pd6OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pd6_over::W`](W) writer structure"]
impl crate::Writable for Pd6OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD6_OVER to value 0x04"]
impl crate::Resettable for Pd6OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
