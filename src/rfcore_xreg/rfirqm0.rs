#[doc = "Register `RFIRQM0` reader"]
pub struct R(crate::R<RFIRQM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIRQM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIRQM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIRQM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIRQM0` writer"]
pub struct W(crate::W<RFIRQM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIRQM0_SPEC>;
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
impl From<crate::W<RFIRQM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIRQM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFIRQM` reader - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
pub struct RFIRQM_R(crate::FieldReader<u8, u8>);
impl RFIRQM_R {
    pub(crate) fn new(bits: u8) -> Self {
        RFIRQM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIRQM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFIRQM` writer - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
pub struct RFIRQM_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIRQM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
    #[inline(always)]
    pub fn rfirqm(&self) -> RFIRQM_R {
        RFIRQM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit mask is masking out interrupt sources. Bit position: 7: RXMASKZERO 6: RXPKTDONE 5: FRAME_ACCEPTED 4: SRC_MATCH_FOUND 3: SRC_MATCH_DONE 2: FIFOP 1: SFD 0: ACT_UNUSED"]
    #[inline(always)]
    pub fn rfirqm(&mut self) -> RFIRQM_W {
        RFIRQM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF interrupt masks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfirqm0](index.html) module"]
pub struct RFIRQM0_SPEC;
impl crate::RegisterSpec for RFIRQM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfirqm0::R](R) reader structure"]
impl crate::Readable for RFIRQM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfirqm0::W](W) writer structure"]
impl crate::Writable for RFIRQM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIRQM0 to value 0"]
impl crate::Resettable for RFIRQM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
