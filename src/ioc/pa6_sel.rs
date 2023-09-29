#[doc = "Register `PA6_SEL` reader"]
pub type R = crate::R<PA6_SEL_SPEC>;
#[doc = "Register `PA6_SEL` writer"]
pub type W = crate::W<PA6_SEL_SPEC>;
#[doc = "Field `PA6_sel` reader - Select one peripheral signal output for PA6."]
pub type PA6_SEL_R = crate::FieldReader;
#[doc = "Field `PA6_sel` writer - Select one peripheral signal output for PA6."]
pub type PA6_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA6."]
    #[inline(always)]
    pub fn pa6_sel(&self) -> PA6_SEL_R {
        PA6_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA6."]
    #[inline(always)]
    #[must_use]
    pub fn pa6_sel(&mut self) -> PA6_SEL_W<PA6_SEL_SPEC, 0> {
        PA6_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PA6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa6_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa6_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA6_SEL_SPEC;
impl crate::RegisterSpec for PA6_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa6_sel::R`](R) reader structure"]
impl crate::Readable for PA6_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pa6_sel::W`](W) writer structure"]
impl crate::Writable for PA6_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA6_SEL to value 0"]
impl crate::Resettable for PA6_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
