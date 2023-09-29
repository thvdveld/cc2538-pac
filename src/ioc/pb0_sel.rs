#[doc = "Register `PB0_SEL` reader"]
pub type R = crate::R<PB0_SEL_SPEC>;
#[doc = "Register `PB0_SEL` writer"]
pub type W = crate::W<PB0_SEL_SPEC>;
#[doc = "Field `PB0_sel` reader - Select one peripheral signal output for PB0."]
pub type PB0_SEL_R = crate::FieldReader;
#[doc = "Field `PB0_sel` writer - Select one peripheral signal output for PB0."]
pub type PB0_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB0."]
    #[inline(always)]
    pub fn pb0_sel(&self) -> PB0_SEL_R {
        PB0_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB0."]
    #[inline(always)]
    #[must_use]
    pub fn pb0_sel(&mut self) -> PB0_SEL_W<PB0_SEL_SPEC, 0> {
        PB0_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PB0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb0_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb0_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB0_SEL_SPEC;
impl crate::RegisterSpec for PB0_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb0_sel::R`](R) reader structure"]
impl crate::Readable for PB0_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb0_sel::W`](W) writer structure"]
impl crate::Writable for PB0_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PB0_SEL to value 0"]
impl crate::Resettable for PB0_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
