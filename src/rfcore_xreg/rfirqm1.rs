#[doc = "Register `RFIRQM1` reader"]
pub struct R(crate::R<RFIRQM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIRQM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIRQM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIRQM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIRQM1` writer"]
pub struct W(crate::W<RFIRQM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIRQM1_SPEC>;
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
impl From<crate::W<RFIRQM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIRQM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFIRQM` reader - Bit mask is masking out interrupt sources. Bit position: 5: CSP_WAIT 4: CSP_STOP 3: CSP_MANINT 2: RF_IDLE 1: TXDONE 0: TXACKDONE"]
pub type RFIRQM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFIRQM` writer - Bit mask is masking out interrupt sources. Bit position: 5: CSP_WAIT 4: CSP_STOP 3: CSP_MANINT 2: RF_IDLE 1: TXDONE 0: TXACKDONE"]
pub type RFIRQM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFIRQM1_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Bit mask is masking out interrupt sources. Bit position: 5: CSP_WAIT 4: CSP_STOP 3: CSP_MANINT 2: RF_IDLE 1: TXDONE 0: TXACKDONE"]
    #[inline(always)]
    pub fn rfirqm(&self) -> RFIRQM_R {
        RFIRQM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Bit mask is masking out interrupt sources. Bit position: 5: CSP_WAIT 4: CSP_STOP 3: CSP_MANINT 2: RF_IDLE 1: TXDONE 0: TXACKDONE"]
    #[inline(always)]
    #[must_use]
    pub fn rfirqm(&mut self) -> RFIRQM_W<0> {
        RFIRQM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF interrupt masks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfirqm1](index.html) module"]
pub struct RFIRQM1_SPEC;
impl crate::RegisterSpec for RFIRQM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfirqm1::R](R) reader structure"]
impl crate::Readable for RFIRQM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfirqm1::W](W) writer structure"]
impl crate::Writable for RFIRQM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFIRQM1 to value 0"]
impl crate::Resettable for RFIRQM1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
