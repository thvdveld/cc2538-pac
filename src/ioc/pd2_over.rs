#[doc = "Register `PD2_OVER` reader"]
pub type R = crate::R<PD2_OVER_SPEC>;
#[doc = "Register `PD2_OVER` writer"]
pub type W = crate::W<PD2_OVER_SPEC>;
#[doc = "Field `PD2_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PD2_OVER_R = crate::FieldReader;
#[doc = "Field `PD2_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PD2_OVER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pd2_over(&self) -> PD2_OVER_R {
        PD2_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pd2_over(&mut self) -> PD2_OVER_W<PD2_OVER_SPEC> {
        PD2_OVER_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd2_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd2_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD2_OVER_SPEC;
impl crate::RegisterSpec for PD2_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd2_over::R`](R) reader structure"]
impl crate::Readable for PD2_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd2_over::W`](W) writer structure"]
impl crate::Writable for PD2_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD2_OVER to value 0x04"]
impl crate::Resettable for PD2_OVER_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
