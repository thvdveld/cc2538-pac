#[doc = "Register `USEBURSTCLR` writer"]
pub struct W(crate::W<USEBURSTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USEBURSTCLR_SPEC>;
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
impl From<crate::W<USEBURSTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USEBURSTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` writer - Channel \\[n\\]
useburst clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAUSEBURSTSET register meaning that uDMA channel \\[n\\]
responds to single and burst requests."]
pub type CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USEBURSTCLR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
useburst clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAUSEBURSTSET register meaning that uDMA channel \\[n\\]
responds to single and burst requests."]
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
#[doc = "DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the USEBURSTSET register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [useburstclr](index.html) module"]
pub struct USEBURSTCLR_SPEC;
impl crate::RegisterSpec for USEBURSTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [useburstclr::W](W) writer structure"]
impl crate::Writable for USEBURSTCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USEBURSTCLR to value 0"]
impl crate::Resettable for USEBURSTCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
