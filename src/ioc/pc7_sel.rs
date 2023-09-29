#[doc = "Register `PC7_SEL` reader"]
pub type R = crate::R<PC7_SEL_SPEC>;
#[doc = "Register `PC7_SEL` writer"]
pub type W = crate::W<PC7_SEL_SPEC>;
#[doc = "Field `PC7_sel` reader - Select one peripheral signal output for PC7."]
pub type PC7_SEL_R = crate::FieldReader;
#[doc = "Field `PC7_sel` writer - Select one peripheral signal output for PC7."]
pub type PC7_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC7."]
    #[inline(always)]
    pub fn pc7_sel(&self) -> PC7_SEL_R {
        PC7_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select one peripheral signal output for PC7."]
    #[inline(always)]
    #[must_use]
    pub fn pc7_sel(&mut self) -> PC7_SEL_W<PC7_SEL_SPEC, 0> {
        PC7_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral select control for PC7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pc7_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pc7_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PC7_SEL_SPEC;
impl crate::RegisterSpec for PC7_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pc7_sel::R`](R) reader structure"]
impl crate::Readable for PC7_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pc7_sel::W`](W) writer structure"]
impl crate::Writable for PC7_SEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PC7_SEL to value 0"]
impl crate::Resettable for PC7_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
