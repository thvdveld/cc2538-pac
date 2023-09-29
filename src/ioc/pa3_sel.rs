#[doc = "Register `PA3_SEL` reader"]
pub type R = crate::R<PA3_SEL_SPEC>;
#[doc = "Register `PA3_SEL` writer"]
pub type W = crate::W<PA3_SEL_SPEC>;
#[doc = "Field `PA3_sel` reader - Select one peripheral signal output for PA3."]
pub type PA3_SEL_R = crate::FieldReader;
#[doc = "Field `PA3_sel` writer - Select one peripheral signal output for PA3."]
pub type PA3_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA3."]
    #[inline(always)]
    pub fn pa3_sel(&self) -> PA3_SEL_R {
        PA3_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA3."]
    #[inline(always)]
    #[must_use]
    pub fn pa3_sel(&mut self) -> PA3_SEL_W<PA3_SEL_SPEC, 0> {
        PA3_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PA3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa3_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa3_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA3_SEL_SPEC;
impl crate::RegisterSpec for PA3_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa3_sel::R`](R) reader structure"]
impl crate::Readable for PA3_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pa3_sel::W`](W) writer structure"]
impl crate::Writable for PA3_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA3_SEL to value 0"]
impl crate::Resettable for PA3_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
