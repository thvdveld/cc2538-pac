#[doc = "Register `PD5_SEL` reader"]
pub type R = crate::R<PD5_SEL_SPEC>;
#[doc = "Register `PD5_SEL` writer"]
pub type W = crate::W<PD5_SEL_SPEC>;
#[doc = "Field `PD5_sel` reader - Select one peripheral signal output for PD5."]
pub type PD5_SEL_R = crate::FieldReader;
#[doc = "Field `PD5_sel` writer - Select one peripheral signal output for PD5."]
pub type PD5_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD5."]
    #[inline(always)]
    pub fn pd5_sel(&self) -> PD5_SEL_R {
        PD5_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PD5."]
    #[inline(always)]
    #[must_use]
    pub fn pd5_sel(&mut self) -> PD5_SEL_W<PD5_SEL_SPEC, 0> {
        PD5_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PD5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd5_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd5_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD5_SEL_SPEC;
impl crate::RegisterSpec for PD5_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd5_sel::R`](R) reader structure"]
impl crate::Readable for PD5_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd5_sel::W`](W) writer structure"]
impl crate::Writable for PD5_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PD5_SEL to value 0"]
impl crate::Resettable for PD5_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
