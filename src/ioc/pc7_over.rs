#[doc = "Register `PC7_OVER` reader"]
pub type R = crate::R<Pc7OverSpec>;
#[doc = "Register `PC7_OVER` writer"]
pub type W = crate::W<Pc7OverSpec>;
#[doc = "Field `PC7_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pc7OverR = crate::FieldReader;
#[doc = "Field `PC7_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type Pc7OverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pc7_over(&self) -> Pc7OverR {
        Pc7OverR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pc7_over(&mut self) -> Pc7OverW<Pc7OverSpec> {
        Pc7OverW::new(self, 0)
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc7_over::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc7_over::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pc7OverSpec;
impl crate::RegisterSpec for Pc7OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc7_over::R`](R) reader structure"]
impl crate::Readable for Pc7OverSpec {}
#[doc = "`write(|w| ..)` method takes [`pc7_over::W`](W) writer structure"]
impl crate::Writable for Pc7OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PC7_OVER to value 0x04"]
impl crate::Resettable for Pc7OverSpec {
    const RESET_VALUE: u32 = 0x04;
}
