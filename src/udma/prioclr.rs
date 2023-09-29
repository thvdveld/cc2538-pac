#[doc = "Register `PRIOCLR` writer"]
pub type W = crate::W<PRIOCLR_SPEC>;
#[doc = "Field `CLR` writer - Channel \\[n\\]
priority clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAPRIOSET register meaning that channel \\[n\\]
is using the default priority level."]
pub type CLR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
priority clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAPRIOSET register meaning that channel \\[n\\]
is using the default priority level."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> CLR_W<PRIOCLR_SPEC, 0> {
        CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the PRIOSET register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prioclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIOCLR_SPEC;
impl crate::RegisterSpec for PRIOCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prioclr::W`](W) writer structure"]
impl crate::Writable for PRIOCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIOCLR to value 0"]
impl crate::Resettable for PRIOCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
