#[doc = "Register `PB3_SEL` reader"]
pub type R = crate::R<PB3_SEL_SPEC>;
#[doc = "Register `PB3_SEL` writer"]
pub type W = crate::W<PB3_SEL_SPEC>;
#[doc = "Field `PB3_sel` reader - Select one peripheral signal output for PB3."]
pub type PB3_SEL_R = crate::FieldReader;
#[doc = "Field `PB3_sel` writer - Select one peripheral signal output for PB3."]
pub type PB3_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB3."]
    #[inline(always)]
    pub fn pb3_sel(&self) -> PB3_SEL_R {
        PB3_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB3."]
    #[inline(always)]
    #[must_use]
    pub fn pb3_sel(&mut self) -> PB3_SEL_W<PB3_SEL_SPEC, 0> {
        PB3_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PB3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb3_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb3_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB3_SEL_SPEC;
impl crate::RegisterSpec for PB3_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb3_sel::R`](R) reader structure"]
impl crate::Readable for PB3_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb3_sel::W`](W) writer structure"]
impl crate::Writable for PB3_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PB3_SEL to value 0"]
impl crate::Resettable for PB3_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
