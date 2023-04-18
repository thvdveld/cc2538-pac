#[doc = "Register `TBPR` reader"]
pub struct R(crate::R<TBPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPR` writer"]
pub struct W(crate::W<TBPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TBPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBPSR` reader - GPTM Timer B prescale"]
pub type TBPSR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBPSR` writer - GPTM Timer B prescale"]
pub type TBPSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBPR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - GPTM Timer B prescale"]
    #[inline(always)]
    pub fn tbpsr(&self) -> TBPSR_R {
        TBPSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPTM Timer B prescale"]
    #[inline(always)]
    #[must_use]
    pub fn tbpsr(&mut self) -> TBPSR_W<0> {
        TBPSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B prescale This register allows software to extend the range of the 16-bit Timers in periodic and one-shot modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpr](index.html) module"]
pub struct TBPR_SPEC;
impl crate::RegisterSpec for TBPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpr::R](R) reader structure"]
impl crate::Readable for TBPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbpr::W](W) writer structure"]
impl crate::Writable for TBPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBPR to value 0"]
impl crate::Resettable for TBPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
