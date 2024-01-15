#[doc = "Register `PA5_OVER` reader"]
pub type R = crate::R<PA5_OVER_SPEC>;
#[doc = "Register `PA5_OVER` writer"]
pub type W = crate::W<PA5_OVER_SPEC>;
#[doc = "Field `PA5_over` reader - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PA5_OVER_R = crate::FieldReader;
#[doc = "Field `PA5_over` writer - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
pub type PA5_OVER_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    pub fn pa5_over(&self) -> PA5_OVER_R {
        PA5_OVER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 0x8: oe - output enable 0x4: pue - pullup enable 0x2: pde - pulldown enable 0x1: ana - analog enable"]
    #[inline(always)]
    #[must_use]
    pub fn pa5_over(&mut self) -> PA5_OVER_W<PA5_OVER_SPEC> {
        PA5_OVER_W::new(self, 0)
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
#[doc = "This is the overide configuration register for each pad.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa5_over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa5_over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA5_OVER_SPEC;
impl crate::RegisterSpec for PA5_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa5_over::R`](R) reader structure"]
impl crate::Readable for PA5_OVER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pa5_over::W`](W) writer structure"]
impl crate::Writable for PA5_OVER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PA5_OVER to value 0x04"]
impl crate::Resettable for PA5_OVER_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
