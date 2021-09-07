#[doc = "Register `SWREQ` writer"]
pub struct W(crate::W<SWREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWREQ_SPEC>;
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
impl From<crate::W<SWREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWREQ` writer - Channel \\[n\\]
software request These bits generate software requests. Bit 0 corresponds to channel 0. 1: Generate a software request for the corresponding channel 0: No request generated These bits are automatically cleared when the software request has been completed."]
pub struct SWREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SWREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits |= value as u32;
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
software request These bits generate software requests. Bit 0 corresponds to channel 0. 1: Generate a software request for the corresponding channel 0: No request generated These bits are automatically cleared when the software request has been completed."]
    #[inline(always)]
    pub fn swreq(&mut self) -> SWREQ_W {
        SWREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreq](index.html) module"]
pub struct SWREQ_SPEC;
impl crate::RegisterSpec for SWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [swreq::W](W) writer structure"]
impl crate::Writable for SWREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWREQ to value 0"]
impl crate::Resettable for SWREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
