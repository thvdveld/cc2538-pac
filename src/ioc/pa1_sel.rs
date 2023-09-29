#[doc = "Register `PA1_SEL` reader"]
pub type R = crate::R<PA1_SEL_SPEC>;
#[doc = "Register `PA1_SEL` writer"]
pub type W = crate::W<PA1_SEL_SPEC>;
#[doc = "Field `PA1_sel` reader - Select one peripheral signal output for PA1."]
pub type PA1_SEL_R = crate::FieldReader;
#[doc = "Field `PA1_sel` writer - Select one peripheral signal output for PA1."]
pub type PA1_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA1."]
    #[inline(always)]
    pub fn pa1_sel(&self) -> PA1_SEL_R {
        PA1_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PA1."]
    #[inline(always)]
    #[must_use]
    pub fn pa1_sel(&mut self) -> PA1_SEL_W<PA1_SEL_SPEC, 0> {
        PA1_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PA1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pa1_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pa1_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PA1_SEL_SPEC;
impl crate::RegisterSpec for PA1_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pa1_sel::R`](R) reader structure"]
impl crate::Readable for PA1_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pa1_sel::W`](W) writer structure"]
impl crate::Writable for PA1_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PA1_SEL to value 0"]
impl crate::Resettable for PA1_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
