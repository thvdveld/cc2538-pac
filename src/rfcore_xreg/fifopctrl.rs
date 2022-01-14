#[doc = "Register `FIFOPCTRL` reader"]
pub struct R(crate::R<FIFOPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOPCTRL` writer"]
pub struct W(crate::W<FIFOPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOPCTRL_SPEC>;
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
impl From<crate::W<FIFOPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOP_THR` reader - Threshold used when generating FIFOP signal"]
pub struct FIFOP_THR_R(crate::FieldReader<u8, u8>);
impl FIFOP_THR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFOP_THR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOP_THR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOP_THR` writer - Threshold used when generating FIFOP signal"]
pub struct FIFOP_THR_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOP_THR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Threshold used when generating FIFOP signal"]
    #[inline(always)]
    pub fn fifop_thr(&self) -> FIFOP_THR_R {
        FIFOP_THR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Threshold used when generating FIFOP signal"]
    #[inline(always)]
    pub fn fifop_thr(&mut self) -> FIFOP_THR_W {
        FIFOP_THR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFOP threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifopctrl](index.html) module"]
pub struct FIFOPCTRL_SPEC;
impl crate::RegisterSpec for FIFOPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifopctrl::R](R) reader structure"]
impl crate::Readable for FIFOPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifopctrl::W](W) writer structure"]
impl crate::Writable for FIFOPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOPCTRL to value 0"]
impl crate::Resettable for FIFOPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
