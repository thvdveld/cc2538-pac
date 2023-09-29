#[doc = "Register `PB7_SEL` reader"]
pub type R = crate::R<PB7_SEL_SPEC>;
#[doc = "Register `PB7_SEL` writer"]
pub type W = crate::W<PB7_SEL_SPEC>;
#[doc = "Field `PB7_sel` reader - Select one peripheral signal output for PB7."]
pub type PB7_SEL_R = crate::FieldReader;
#[doc = "Field `PB7_sel` writer - Select one peripheral signal output for PB7."]
pub type PB7_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB7."]
    #[inline(always)]
    pub fn pb7_sel(&self) -> PB7_SEL_R {
        PB7_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PB7."]
    #[inline(always)]
    #[must_use]
    pub fn pb7_sel(&mut self) -> PB7_SEL_W<PB7_SEL_SPEC, 0> {
        PB7_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PB7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb7_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb7_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB7_SEL_SPEC;
impl crate::RegisterSpec for PB7_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb7_sel::R`](R) reader structure"]
impl crate::Readable for PB7_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb7_sel::W`](W) writer structure"]
impl crate::Writable for PB7_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PB7_SEL to value 0"]
impl crate::Resettable for PB7_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
