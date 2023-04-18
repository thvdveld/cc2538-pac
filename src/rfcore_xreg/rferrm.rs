#[doc = "Register `RFERRM` reader"]
pub struct R(crate::R<RFERRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFERRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFERRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFERRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFERRM` writer"]
pub struct W(crate::W<RFERRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFERRM_SPEC>;
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
impl From<crate::W<RFERRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFERRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFERRM` reader - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
pub type RFERRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFERRM` writer - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
pub type RFERRM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFERRM_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
    #[inline(always)]
    pub fn rferrm(&self) -> RFERRM_R {
        RFERRM_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Bit mask is masking out interrupt sources. Bit position: 6: STROBE_ERR 5: TXUNDERF 4: TXOVERF 3: RXUNDERF 2: RXOVERF 1: RXABO 0: NLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn rferrm(&mut self) -> RFERRM_W<0> {
        RFERRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF error interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rferrm](index.html) module"]
pub struct RFERRM_SPEC;
impl crate::RegisterSpec for RFERRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rferrm::R](R) reader structure"]
impl crate::Readable for RFERRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rferrm::W](W) writer structure"]
impl crate::Writable for RFERRM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFERRM to value 0"]
impl crate::Resettable for RFERRM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
