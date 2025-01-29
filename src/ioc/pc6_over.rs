#[doc = "Register `PC6_OVER` reader"]
pub type R = crate::R<Pc6OverSpec>;
#[doc = "Register `PC6_OVER` writer"]
pub type W = crate::W<Pc6OverSpec>;
#[doc = "Field `PC6_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pc6OverR = crate::FieldReader;
#[doc = "Field `PC6_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pc6OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pc6_over(&self) -> Pc6OverR {
        Pc6OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pc6_over(&mut self) -> Pc6OverW<Pc6OverSpec> {
        Pc6OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc6_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc6_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc6OverSpec;
impl crate::RegisterSpec for Pc6OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc6_over::R`](R) reader structure"]
impl crate::Readable for Pc6OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pc6_over::W`](W) writer structure"]
impl crate::Writable for Pc6OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC6_OVER to value 0x04"]
impl crate::Resettable for Pc6OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
