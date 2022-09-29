#[doc = "Register `RFIRQF0` reader"]
pub struct R(crate::R<RFIRQF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIRQF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIRQF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIRQF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIRQF0` writer"]
pub struct W(crate::W<RFIRQF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIRQF0_SPEC>;
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
impl From<crate::W<RFIRQF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIRQF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT_UNUSED` reader - Reserved 0: No interrupt pending 1: Interrupt pending"]
pub type ACT_UNUSED_R = crate::BitReader<bool>;
#[doc = "Field `ACT_UNUSED` writer - Reserved 0: No interrupt pending 1: Interrupt pending"]
pub type ACT_UNUSED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIRQF0_SPEC, bool, O>;
#[doc = "Field `SFD` reader - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type SFD_R = crate::BitReader<bool>;
#[doc = "Field `SFD` writer - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type SFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIRQF0_SPEC, bool, O>;
#[doc = "Field `FIFOP` reader - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
pub type FIFOP_R = crate::BitReader<bool>;
#[doc = "Field `FIFOP` writer - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
pub type FIFOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIRQF0_SPEC, bool, O>;
#[doc = "Field `SRC_MATCH_DONE` reader - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
pub type SRC_MATCH_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SRC_MATCH_DONE` writer - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
pub type SRC_MATCH_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIRQF0_SPEC, bool, O>;
#[doc = "Field `SRC_MATCH_FOUND` reader - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
pub type SRC_MATCH_FOUND_R = crate::BitReader<bool>;
#[doc = "Field `SRC_MATCH_FOUND` writer - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
pub type SRC_MATCH_FOUND_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIRQF0_SPEC, bool, O>;
#[doc = "Field `FRAME_ACCEPTED` reader - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
pub type FRAME_ACCEPTED_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_ACCEPTED` writer - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
pub type FRAME_ACCEPTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIRQF0_SPEC, bool, O>;
#[doc = "Field `RXPKTDONE` reader - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
pub type RXPKTDONE_R = crate::BitReader<bool>;
#[doc = "Field `RXPKTDONE` writer - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
pub type RXPKTDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIRQF0_SPEC, bool, O>;
#[doc = "Field `RXMASKZERO` reader - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
pub type RXMASKZERO_R = crate::BitReader<bool>;
#[doc = "Field `RXMASKZERO` writer - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
pub type RXMASKZERO_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFIRQF0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reserved 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn act_unused(&self) -> ACT_UNUSED_R {
        ACT_UNUSED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn sfd(&self) -> SFD_R {
        SFD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn fifop(&self) -> FIFOP_R {
        FIFOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_done(&self) -> SRC_MATCH_DONE_R {
        SRC_MATCH_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_found(&self) -> SRC_MATCH_FOUND_R {
        SRC_MATCH_FOUND_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn frame_accepted(&self) -> FRAME_ACCEPTED_R {
        FRAME_ACCEPTED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxpktdone(&self) -> RXPKTDONE_R {
        RXPKTDONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxmaskzero(&self) -> RXMASKZERO_R {
        RXMASKZERO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn act_unused(&mut self) -> ACT_UNUSED_W<0> {
        ACT_UNUSED_W::new(self)
    }
    #[doc = "Bit 1 - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SFD_W<1> {
        SFD_W::new(self)
    }
    #[doc = "Bit 2 - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn fifop(&mut self) -> FIFOP_W<2> {
        FIFOP_W::new(self)
    }
    #[doc = "Bit 3 - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_done(&mut self) -> SRC_MATCH_DONE_W<3> {
        SRC_MATCH_DONE_W::new(self)
    }
    #[doc = "Bit 4 - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_found(&mut self) -> SRC_MATCH_FOUND_W<4> {
        SRC_MATCH_FOUND_W::new(self)
    }
    #[doc = "Bit 5 - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn frame_accepted(&mut self) -> FRAME_ACCEPTED_W<5> {
        FRAME_ACCEPTED_W::new(self)
    }
    #[doc = "Bit 6 - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxpktdone(&mut self) -> RXPKTDONE_W<6> {
        RXPKTDONE_W::new(self)
    }
    #[doc = "Bit 7 - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxmaskzero(&mut self) -> RXMASKZERO_W<7> {
        RXMASKZERO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfirqf0](index.html) module"]
pub struct RFIRQF0_SPEC;
impl crate::RegisterSpec for RFIRQF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfirqf0::R](R) reader structure"]
impl crate::Readable for RFIRQF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfirqf0::W](W) writer structure"]
impl crate::Writable for RFIRQF0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIRQF0 to value 0"]
impl crate::Resettable for RFIRQF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
