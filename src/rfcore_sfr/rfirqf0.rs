#[doc = "Register `RFIRQF0` reader"]
pub type R = crate::R<RFIRQF0_SPEC>;
#[doc = "Register `RFIRQF0` writer"]
pub type W = crate::W<RFIRQF0_SPEC>;
#[doc = "Field `ACT_UNUSED` reader - Reserved 0: No interrupt pending 1: Interrupt pending"]
pub type ACT_UNUSED_R = crate::BitReader;
#[doc = "Field `ACT_UNUSED` writer - Reserved 0: No interrupt pending 1: Interrupt pending"]
pub type ACT_UNUSED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFD` reader - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type SFD_R = crate::BitReader;
#[doc = "Field `SFD` writer - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type SFD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOP` reader - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
pub type FIFOP_R = crate::BitReader;
#[doc = "Field `FIFOP` writer - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
pub type FIFOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MATCH_DONE` reader - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
pub type SRC_MATCH_DONE_R = crate::BitReader;
#[doc = "Field `SRC_MATCH_DONE` writer - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
pub type SRC_MATCH_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MATCH_FOUND` reader - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
pub type SRC_MATCH_FOUND_R = crate::BitReader;
#[doc = "Field `SRC_MATCH_FOUND` writer - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
pub type SRC_MATCH_FOUND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_ACCEPTED` reader - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
pub type FRAME_ACCEPTED_R = crate::BitReader;
#[doc = "Field `FRAME_ACCEPTED` writer - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
pub type FRAME_ACCEPTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPKTDONE` reader - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
pub type RXPKTDONE_R = crate::BitReader;
#[doc = "Field `RXPKTDONE` writer - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
pub type RXPKTDONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMASKZERO` reader - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
pub type RXMASKZERO_R = crate::BitReader;
#[doc = "Field `RXMASKZERO` writer - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
pub type RXMASKZERO_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[must_use]
    pub fn act_unused(&mut self) -> ACT_UNUSED_W<RFIRQF0_SPEC> {
        ACT_UNUSED_W::new(self, 0)
    }
    #[doc = "Bit 1 - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn sfd(&mut self) -> SFD_W<RFIRQF0_SPEC> {
        SFD_W::new(self, 1)
    }
    #[doc = "Bit 2 - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn fifop(&mut self) -> FIFOP_W<RFIRQF0_SPEC> {
        FIFOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn src_match_done(&mut self) -> SRC_MATCH_DONE_W<RFIRQF0_SPEC> {
        SRC_MATCH_DONE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn src_match_found(&mut self) -> SRC_MATCH_FOUND_W<RFIRQF0_SPEC> {
        SRC_MATCH_FOUND_W::new(self, 4)
    }
    #[doc = "Bit 5 - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn frame_accepted(&mut self) -> FRAME_ACCEPTED_W<RFIRQF0_SPEC> {
        FRAME_ACCEPTED_W::new(self, 5)
    }
    #[doc = "Bit 6 - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxpktdone(&mut self) -> RXPKTDONE_W<RFIRQF0_SPEC> {
        RXPKTDONE_W::new(self, 6)
    }
    #[doc = "Bit 7 - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    #[must_use]
    pub fn rxmaskzero(&mut self) -> RXMASKZERO_W<RFIRQF0_SPEC> {
        RXMASKZERO_W::new(self, 7)
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
#[doc = "RF interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfirqf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfirqf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFIRQF0_SPEC;
impl crate::RegisterSpec for RFIRQF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfirqf0::R`](R) reader structure"]
impl crate::Readable for RFIRQF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfirqf0::W`](W) writer structure"]
impl crate::Writable for RFIRQF0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFIRQF0 to value 0"]
impl crate::Resettable for RFIRQF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
