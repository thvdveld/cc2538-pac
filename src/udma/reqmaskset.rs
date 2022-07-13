#[doc = "Register `REQMASKSET` reader"]
pub struct R(crate::R<REQMASKSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REQMASKSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REQMASKSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REQMASKSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REQMASKSET` writer"]
pub struct W(crate::W<REQMASKSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REQMASKSET_SPEC>;
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
impl From<crate::W<REQMASKSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REQMASKSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET` reader - Channel \\[n\\]
request mask set 0: The peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers 1: The peripheral associated with channel \\[n\\]
is not able to request uDMA transfers. Channel \\[n\\]
may be used for software-initiated transfers. Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAREQMASKCLR register."]
pub type SET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
request mask set 0: The peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers 1: The peripheral associated with channel \\[n\\]
is not able to request uDMA transfers. Channel \\[n\\]
may be used for software-initiated transfers. Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAREQMASKCLR register."]
pub type SET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REQMASKSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
request mask set 0: The peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers 1: The peripheral associated with channel \\[n\\]
is not able to request uDMA transfers. Channel \\[n\\]
may be used for software-initiated transfers. Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAREQMASKCLR register."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
request mask set 0: The peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers 1: The peripheral associated with channel \\[n\\]
is not able to request uDMA transfers. Channel \\[n\\]
may be used for software-initiated transfers. Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAREQMASKCLR register."]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W<0> {
        SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqmaskset](index.html) module"]
pub struct REQMASKSET_SPEC;
impl crate::RegisterSpec for REQMASKSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reqmaskset::R](R) reader structure"]
impl crate::Readable for REQMASKSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reqmaskset::W](W) writer structure"]
impl crate::Writable for REQMASKSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REQMASKSET to value 0"]
impl crate::Resettable for REQMASKSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
