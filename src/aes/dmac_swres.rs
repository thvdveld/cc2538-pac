#[doc = "Register `DMAC_SWRES` writer"]
pub struct W(crate::W<DMAC_SWRES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_SWRES_SPEC>;
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
impl From<crate::W<DMAC_SWRES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_SWRES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRES` writer - Software reset enable 0 = Disabled 1 = Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMAC_STATUS register."]
pub type SWRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAC_SWRES_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Software reset enable 0 = Disabled 1 = Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMAC_STATUS register."]
    #[inline(always)]
    #[must_use]
    pub fn swres(&mut self) -> SWRES_W<0> {
        SWRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_swres](index.html) module"]
pub struct DMAC_SWRES_SPEC;
impl crate::RegisterSpec for DMAC_SWRES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dmac_swres::W](W) writer structure"]
impl crate::Writable for DMAC_SWRES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC_SWRES to value 0"]
impl crate::Resettable for DMAC_SWRES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
