#[doc = "Register `SRCMATCH` reader"]
pub struct R(crate::R<SRCMATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCMATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCMATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCMATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCMATCH` writer"]
pub struct W(crate::W<SRCMATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCMATCH_SPEC>;
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
impl From<crate::W<SRCMATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCMATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC_MATCH_EN` reader - Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
pub type SRC_MATCH_EN_R = crate::BitReader<bool>;
#[doc = "Field `SRC_MATCH_EN` writer - Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
pub type SRC_MATCH_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCMATCH_SPEC, bool, O>;
#[doc = "Field `AUTOPEND` reader - Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
pub type AUTOPEND_R = crate::BitReader<bool>;
#[doc = "Field `AUTOPEND` writer - Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
pub type AUTOPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCMATCH_SPEC, bool, O>;
#[doc = "Field `PEND_DATAREQ_ONLY` reader - When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
pub type PEND_DATAREQ_ONLY_R = crate::BitReader<bool>;
#[doc = "Field `PEND_DATAREQ_ONLY` writer - When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
pub type PEND_DATAREQ_ONLY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRCMATCH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
    #[inline(always)]
    pub fn src_match_en(&self) -> SRC_MATCH_EN_R {
        SRC_MATCH_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
    #[inline(always)]
    pub fn autopend(&self) -> AUTOPEND_R {
        AUTOPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
    #[inline(always)]
    pub fn pend_datareq_only(&self) -> PEND_DATAREQ_ONLY_R {
        PEND_DATAREQ_ONLY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source address matching enable (requires that FRMFILT.FRAME_FILTER_EN = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn src_match_en(&mut self) -> SRC_MATCH_EN_W<0> {
        SRC_MATCH_EN_W::new(self)
    }
    #[doc = "Bit 1 - Automatic acknowledgment pending flag enable When a frame is received, the pending bit in the (possibly) returned acknowledgment is set automatically when the following conditions are met: - FRMFILT.FRAME_FILTER_EN is set. - SRCMATCH.SRC_MATCH_EN is set. - SRCMATCH.AUTOPEND is set. - The received frame matches the current SRCMATCH.PEND_DATAREQ_ONLY setting. - The received source address matches at least one source match table entry, which is enabled in SHORT_ADDR_EN and SHORT_PEND_EN or in EXT_ADDR_EN and EXT_PEND_EN."]
    #[inline(always)]
    #[must_use]
    pub fn autopend(&mut self) -> AUTOPEND_W<1> {
        AUTOPEND_W::new(self)
    }
    #[doc = "Bit 2 - When this bit is set, the AUTOPEND function also requires that the received frame is a DATA REQUEST MAC command frame."]
    #[inline(always)]
    #[must_use]
    pub fn pend_datareq_only(&mut self) -> PEND_DATAREQ_ONLY_W<2> {
        PEND_DATAREQ_ONLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching and pending bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcmatch](index.html) module"]
pub struct SRCMATCH_SPEC;
impl crate::RegisterSpec for SRCMATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcmatch::R](R) reader structure"]
impl crate::Readable for SRCMATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcmatch::W](W) writer structure"]
impl crate::Writable for SRCMATCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCMATCH to value 0"]
impl crate::Resettable for SRCMATCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
