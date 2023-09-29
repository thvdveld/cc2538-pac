#[doc = "Register `REQMASKCLR` writer"]
pub type W = crate::W<REQMASKCLR_SPEC>;
#[doc = "Field `CLR` writer - Channel \\[n\\]
request mask clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAREQMASKSET register meaning that the peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers."]
pub type CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
request mask clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAREQMASKSET register meaning that the peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<REQMASKCLR_SPEC, 0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the REQMASKSET register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqmaskclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REQMASKCLR_SPEC;
impl crate::RegisterSpec for REQMASKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reqmaskclr::W`](W) writer structure"]
impl crate::Writable for REQMASKCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQMASKCLR to value 0"]
impl crate::Resettable for REQMASKCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
