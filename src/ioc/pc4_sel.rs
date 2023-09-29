#[doc = "Register `PC4_SEL` reader"]
pub type R = crate::R<PC4_SEL_SPEC>;
#[doc = "Register `PC4_SEL` writer"]
pub type W = crate::W<PC4_SEL_SPEC>;
#[doc = "Field `PC4_sel` reader - Select one peripheral signal output for PC4."]
pub type PC4_SEL_R = crate::FieldReader;
#[doc = "Field `PC4_sel` writer - Select one peripheral signal output for PC4."]
pub type PC4_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC4."]
    #[inline(always)]
    pub fn pc4_sel(&self) -> PC4_SEL_R {
        PC4_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC4."]
    #[inline(always)]
    #[must_use]
    pub fn pc4_sel(&mut self) -> PC4_SEL_W<PC4_SEL_SPEC, 0> {
        PC4_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PC4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc4_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc4_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC4_SEL_SPEC;
impl crate::RegisterSpec for PC4_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc4_sel::R`](R) reader structure"]
impl crate::Readable for PC4_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc4_sel::W`](W) writer structure"]
impl crate::Writable for PC4_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC4_SEL to value 0"]
impl crate::Resettable for PC4_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
