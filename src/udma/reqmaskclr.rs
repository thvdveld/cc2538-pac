#[doc = "Register `REQMASKCLR` writer"]
pub struct W(crate::W<REQMASKCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REQMASKCLR_SPEC>;
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
impl From<crate::W<REQMASKCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REQMASKCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` writer - Channel \\[n\\]
request mask clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAREQMASKSET register meaning that the peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers."]
pub type CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REQMASKCLR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
request mask clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAREQMASKSET register meaning that the peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers."]
    #[inline(always)]
    #[must_use]
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
#[doc = "DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the REQMASKSET register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqmaskclr](index.html) module"]
pub struct REQMASKCLR_SPEC;
impl crate::RegisterSpec for REQMASKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [reqmaskclr::W](W) writer structure"]
impl crate::Writable for REQMASKCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQMASKCLR to value 0"]
impl crate::Resettable for REQMASKCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
