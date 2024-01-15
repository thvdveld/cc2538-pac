#[doc = "Register `DMAC_SWRES` writer"]
pub type W = crate::W<DMAC_SWRES_SPEC>;
#[doc = "Field `SWRES` writer - Software reset enable 0 = Disabled 1 = Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMAC_STATUS register."]
pub type SWRES_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Software reset enable 0 = Disabled 1 = Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMAC_STATUS register."]
    #[inline(always)]
    #[must_use]
    pub fn swres(&mut self) -> SWRES_W<DMAC_SWRES_SPEC> {
        SWRES_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_swres::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_SWRES_SPEC;
impl crate::RegisterSpec for DMAC_SWRES_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmac_swres::W`](W) writer structure"]
impl crate::Writable for DMAC_SWRES_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_SWRES to value 0"]
impl crate::Resettable for DMAC_SWRES_SPEC {
    const RESET_VALUE: u32 = 0;
}
