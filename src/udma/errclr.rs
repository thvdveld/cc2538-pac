#[doc = "Register `ERRCLR` reader"]
pub struct R(crate::R<ERRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRCLR` writer"]
pub struct W(crate::W<ERRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRCLR_SPEC>;
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
impl From<crate::W<ERRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERRCLR` reader - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
pub struct ERRCLR_R(crate::FieldReader<bool, bool>);
impl ERRCLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ERRCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRCLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRCLR` writer - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
pub struct ERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRCLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn errclr(&self) -> ERRCLR_R {
        ERRCLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uDMA bus error status 0: No bus error is pending 1: A bus error is pending This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn errclr(&mut self) -> ERRCLR_W {
        ERRCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA bus error clear The ERRCLR register is used to read and clear the uDMA bus error status. The error status is set if the uDMA controller encountered a bus error while performing a transfer. If a bus error occurs on a channel, that channel is automatically disabled by the uDMA controller. The other channels are unaffected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errclr](index.html) module"]
pub struct ERRCLR_SPEC;
impl crate::RegisterSpec for ERRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errclr::R](R) reader structure"]
impl crate::Readable for ERRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errclr::W](W) writer structure"]
impl crate::Writable for ERRCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ERRCLR to value 0"]
impl crate::Resettable for ERRCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
