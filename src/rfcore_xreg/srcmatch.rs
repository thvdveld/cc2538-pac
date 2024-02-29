#[doc = "Register `SRCMATCH` reader"]
pub type R = crate::R<SrcmatchSpec>;
#[doc = "Register `SRCMATCH` writer"]
pub type W = crate::W<SrcmatchSpec>;
#[doc = "Field `SRC_MATCH_EN` reader - Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
pub type SrcMatchEnR = crate::BitReader;
#[doc = "Field `SRC_MATCH_EN` writer - Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
pub type SrcMatchEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOPEND` reader - Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
pub type AutopendR = crate::BitReader;
#[doc = "Field `AUTOPEND` writer - Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
pub type AutopendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEND_DATAREQ_ONLY` reader - When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
pub type PendDatareqOnlyR = crate::BitReader;
#[doc = "Field `PEND_DATAREQ_ONLY` writer - When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
pub type PendDatareqOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
    #[inline(always)]
    pub fn src_match_en(&self) -> SrcMatchEnR {
        SrcMatchEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
    #[inline(always)]
    pub fn autopend(&self) -> AutopendR {
        AutopendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
    #[inline(always)]
    pub fn pend_datareq_only(&self) -> PendDatareqOnlyR {
        PendDatareqOnlyR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn src_match_en(&mut self) -> SrcMatchEnW<SrcmatchSpec> {
        SrcMatchEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
    #[inline(always)]
    #[must_use]
    pub fn autopend(&mut self) -> AutopendW<SrcmatchSpec> {
        AutopendW::new(self, 1)
    }
    #[doc = "Bit 2 - When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
    #[inline(always)]
    #[must_use]
    pub fn pend_datareq_only(&mut self) -> PendDatareqOnlyW<SrcmatchSpec> {
        PendDatareqOnlyW::new(self, 2)
    }
}
#[doc = "Source address matching and pending bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcmatch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcmatch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcmatchSpec;
impl crate::RegisterSpec for SrcmatchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcmatch::R`](R) reader structure"]
impl crate::Readable for SrcmatchSpec {}
#[doc = "`write(|w| ..)` method takes [`srcmatch::W`](W) writer structure"]
impl crate::Writable for SrcmatchSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCMATCH to value 0"]
impl crate::Resettable for SrcmatchSpec {
    const RESET_VALUE: u32 = 0;
}
