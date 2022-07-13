#[doc = "Register `ALTCLR` writer"]
pub struct W(crate::W<ALTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTCLR_SPEC>;
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
impl From<crate::W<ALTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` writer - Channel \\[n\\]
alternate clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAALTSET register meaning that channel \\[n\\]
is using the primary control structure. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
pub type CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALTCLR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
alternate clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAALTSET register meaning that channel \\[n\\]
is using the primary control structure. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ALTSET register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altclr](index.html) module"]
pub struct ALTCLR_SPEC;
impl crate::RegisterSpec for ALTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [altclr::W](W) writer structure"]
impl crate::Writable for ALTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALTCLR to value 0"]
impl crate::Resettable for ALTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
