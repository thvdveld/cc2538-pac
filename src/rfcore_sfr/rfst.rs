#[doc = "Register `RFST` reader"]
pub struct R(crate::R<RFST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFST` writer"]
pub struct W(crate::W<RFST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFST_SPEC>;
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
impl From<crate::W<RFST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR` reader - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
pub type INSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR` writer - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
pub type INSTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RFST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
    #[inline(always)]
    pub fn instr(&self) -> INSTR_R {
        INSTR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
    #[inline(always)]
    pub fn instr(&mut self) -> INSTR_W<0> {
        INSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF CSMA-CA/strobe processor\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfst](index.html) module"]
pub struct RFST_SPEC;
impl crate::RegisterSpec for RFST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfst::R](R) reader structure"]
impl crate::Readable for RFST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfst::W](W) writer structure"]
impl crate::Writable for RFST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFST to value 0"]
impl crate::Resettable for RFST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
