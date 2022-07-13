#[doc = "Register `ALTSET` reader"]
pub struct R(crate::R<ALTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTSET` writer"]
pub struct W(crate::W<ALTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTSET_SPEC>;
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
impl From<crate::W<ALTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET` reader - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
pub type SET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
pub type SET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALTSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
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
#[doc = "DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altset](index.html) module"]
pub struct ALTSET_SPEC;
impl crate::RegisterSpec for ALTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altset::R](R) reader structure"]
impl crate::Readable for ALTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altset::W](W) writer structure"]
impl crate::Writable for ALTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTSET to value 0"]
impl crate::Resettable for ALTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
