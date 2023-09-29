#[doc = "Register `SWREQ` writer"]
pub type W = crate::W<SWREQ_SPEC>;
#[doc = "Field `SWREQ` writer - Channel \\[n\\]
software request These bits generate software requests. Bit 0 corresponds to channel 0. 1: Generate a software request for the corresponding channel 0: No request generated These bits are automatically cleared when the software request has been completed."]
pub type SWREQ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
software request These bits generate software requests. Bit 0 corresponds to channel 0. 1: Generate a software request for the corresponding channel 0: No request generated These bits are automatically cleared when the software request has been completed."]
    #[inline(always)]
    #[must_use]
    pub fn swreq(&mut self) -> SWREQ_W<SWREQ_SPEC, 0> {
        SWREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWREQ_SPEC;
impl crate::RegisterSpec for SWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swreq::W`](W) writer structure"]
impl crate::Writable for SWREQ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWREQ to value 0"]
impl crate::Resettable for SWREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
