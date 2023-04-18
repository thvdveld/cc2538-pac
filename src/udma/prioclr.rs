#[doc = "Register `PRIOCLR` writer"]
pub struct W(crate::W<PRIOCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIOCLR_SPEC>;
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
impl From<crate::W<PRIOCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIOCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR` writer - Channel \\[n\\]
priority clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAPRIOSET register meaning that channel \\[n\\]
is using the default priority level."]
pub type CLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRIOCLR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
priority clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAPRIOSET register meaning that channel \\[n\\]
is using the default priority level."]
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
#[doc = "DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the PRIOSET register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prioclr](index.html) module"]
pub struct PRIOCLR_SPEC;
impl crate::RegisterSpec for PRIOCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prioclr::W](W) writer structure"]
impl crate::Writable for PRIOCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIOCLR to value 0"]
impl crate::Resettable for PRIOCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
