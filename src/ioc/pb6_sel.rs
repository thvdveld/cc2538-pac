#[doc = "Register `PB6_SEL` reader"]
pub type R = crate::R<PB6_SEL_SPEC>;
#[doc = "Register `PB6_SEL` writer"]
pub type W = crate::W<PB6_SEL_SPEC>;
#[doc = "Field `PB6_sel` reader - Select one peripheral signal output for PB6."]
pub type PB6_SEL_R = crate::FieldReader;
#[doc = "Field `PB6_sel` writer - Select one peripheral signal output for PB6."]
pub type PB6_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB6."]
    #[inline(always)]
    pub fn pb6_sel(&self) -> PB6_SEL_R {
        PB6_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB6."]
    #[inline(always)]
    #[must_use]
    pub fn pb6_sel(&mut self) -> PB6_SEL_W<PB6_SEL_SPEC, 0> {
        PB6_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PB6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb6_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb6_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB6_SEL_SPEC;
impl crate::RegisterSpec for PB6_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb6_sel::R`](R) reader structure"]
impl crate::Readable for PB6_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb6_sel::W`](W) writer structure"]
impl crate::Writable for PB6_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PB6_SEL to value 0"]
impl crate::Resettable for PB6_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
