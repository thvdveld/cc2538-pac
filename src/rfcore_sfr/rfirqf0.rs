#[doc = "Register `RFIRQF0` reader"]
pub type R = crate::R<Rfirqf0Spec>;
#[doc = "Register `RFIRQF0` writer"]
pub type W = crate::W<Rfirqf0Spec>;
#[doc = "Field `ACT_UNUSED` reader - Reserved 0: No interrupt pending 1: Interrupt pending"]
pub type ActUnusedR = crate::BitReader;
#[doc = "Field `ACT_UNUSED` writer - Reserved 0: No interrupt pending 1: Interrupt pending"]
pub type ActUnusedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFD` reader - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type SfdR = crate::BitReader;
#[doc = "Field `SFD` writer - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub type SfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOP` reader - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
pub type FifopR = crate::BitReader;
#[doc = "Field `FIFOP` writer - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
pub type FifopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MATCH_DONE` reader - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
pub type SrcMatchDoneR = crate::BitReader;
#[doc = "Field `SRC_MATCH_DONE` writer - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
pub type SrcMatchDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC_MATCH_FOUND` reader - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
pub type SrcMatchFoundR = crate::BitReader;
#[doc = "Field `SRC_MATCH_FOUND` writer - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
pub type SrcMatchFoundW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_ACCEPTED` reader - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
pub type FrameAcceptedR = crate::BitReader;
#[doc = "Field `FRAME_ACCEPTED` writer - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
pub type FrameAcceptedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPKTDONE` reader - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
pub type RxpktdoneR = crate::BitReader;
#[doc = "Field `RXPKTDONE` writer - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
pub type RxpktdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMASKZERO` reader - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
pub type RxmaskzeroR = crate::BitReader;
#[doc = "Field `RXMASKZERO` writer - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
pub type RxmaskzeroW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn act_unused(&self) -> ActUnusedR {
        ActUnusedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn sfd(&self) -> SfdR {
        SfdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn fifop(&self) -> FifopR {
        FifopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_done(&self) -> SrcMatchDoneR {
        SrcMatchDoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_found(&self) -> SrcMatchFoundR {
        SrcMatchFoundR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn frame_accepted(&self) -> FrameAcceptedR {
        FrameAcceptedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxpktdone(&self) -> RxpktdoneR {
        RxpktdoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxmaskzero(&self) -> RxmaskzeroR {
        RxmaskzeroR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn act_unused(&mut self) -> ActUnusedW<Rfirqf0Spec> {
        ActUnusedW::new(self, 0)
    }
    #[doc = "Bit 1 - SFD has been received or transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn sfd(&mut self) -> SfdW<Rfirqf0Spec> {
        SfdW::new(self, 1)
    }
    #[doc = "Bit 2 - The number of bytes in the RX FIFO is greater than the threshold. Also raised when a complete frame is received, and when a packet is read out completely and more complete packets are available. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn fifop(&mut self) -> FifopW<Rfirqf0Spec> {
        FifopW::new(self, 2)
    }
    #[doc = "Bit 3 - Source matching is complete. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_done(&mut self) -> SrcMatchDoneW<Rfirqf0Spec> {
        SrcMatchDoneW::new(self, 3)
    }
    #[doc = "Bit 4 - Source match is found. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn src_match_found(&mut self) -> SrcMatchFoundW<Rfirqf0Spec> {
        SrcMatchFoundW::new(self, 4)
    }
    #[doc = "Bit 5 - Frame has passed frame filtering. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn frame_accepted(&mut self) -> FrameAcceptedW<Rfirqf0Spec> {
        FrameAcceptedW::new(self, 5)
    }
    #[doc = "Bit 6 - A complete frame has been received. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxpktdone(&mut self) -> RxpktdoneW<Rfirqf0Spec> {
        RxpktdoneW::new(self, 6)
    }
    #[doc = "Bit 7 - The RXENABLE register has gone from a nonzero state to an all-zero state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxmaskzero(&mut self) -> RxmaskzeroW<Rfirqf0Spec> {
        RxmaskzeroW::new(self, 7)
    }
}
#[doc = "RF interrupt flags\n\nYou can [`read`](crate::Reg::read) this register and get [`rfirqf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfirqf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfirqf0Spec;
impl crate::RegisterSpec for Rfirqf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfirqf0::R`](R) reader structure"]
impl crate::Readable for Rfirqf0Spec {}
#[doc = "`write(|w| ..)` method takes [`rfirqf0::W`](W) writer structure"]
impl crate::Writable for Rfirqf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFIRQF0 to value 0"]
impl crate::Resettable for Rfirqf0Spec {
    const RESET_VALUE: u32 = 0;
}
