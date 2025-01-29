#[doc = "Register `FSMSTAT1` reader"]
pub type R = crate::R<Fsmstat1Spec>;
#[doc = "Field `RX_ACTIVE` reader - Status signal Active when FFC is in one of the receive states"]
pub type RxActiveR = crate::BitReader;
#[doc = "Field `TX_ACTIVE` reader - Status signal Active when FFC is in one of the transmit states"]
pub type TxActiveR = crate::BitReader;
#[doc = "Field `LOCK_STATUS` reader - 1 when PLL is in lock; otherwise 0"]
pub type LockStatusR = crate::BitReader;
#[doc = "Field `SAMPLED_CCA` reader - Contains a sampled value of the CCA The value is updated when a SSAMPLECCA or STXONCCA strobe is issued."]
pub type SampledCcaR = crate::BitReader;
#[doc = "Field `CCA` reader - Clear channel assessment Dependent on CCA_MODE settings. See CCACTRL1 for details."]
pub type CcaR = crate::BitReader;
#[doc = "Field `SFD` reader - In TX 0: When a complete frame with SFD was sent or no SFD was sent 1: SFD was sent. In RX 0: When a complete frame was received or no SFD was received 1: SFD was received."]
pub type SfdR = crate::BitReader;
#[doc = "Field `FIFOP` reader - FIFOP is set high when there are at more than FIFOP_THR bytes of data in the RX FIFO that has passed frame filtering. FIFOP is set high when there is at least one complete frame in the RX FIFO. FIFOP is high during RX FIFO overflow."]
pub type FifopR = crate::BitReader;
#[doc = "Field `FIFO` reader - FIFO is high when there is data in the RX FIFO. FIFO is low during RX FIFO overflow."]
pub type FifoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status signal Active when FFC is in one of the receive states"]
    #[inline(always)]
    pub fn rx_active(&self) -> RxActiveR {
        RxActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status signal Active when FFC is in one of the transmit states"]
    #[inline(always)]
    pub fn tx_active(&self) -> TxActiveR {
        TxActiveR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 when PLL is in lock; otherwise 0"]
    #[inline(always)]
    pub fn lock_status(&self) -> LockStatusR {
        LockStatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Contains a sampled value of the CCA The value is updated when a SSAMPLECCA or STXONCCA strobe is issued."]
    #[inline(always)]
    pub fn sampled_cca(&self) -> SampledCcaR {
        SampledCcaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear channel assessment Dependent on CCA_MODE settings. See CCACTRL1 for details."]
    #[inline(always)]
    pub fn cca(&self) -> CcaR {
        CcaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - In TX 0: When a complete frame with SFD was sent or no SFD was sent 1: SFD was sent. In RX 0: When a complete frame was received or no SFD was received 1: SFD was received."]
    #[inline(always)]
    pub fn sfd(&self) -> SfdR {
        SfdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFOP is set high when there are at more than FIFOP_THR bytes of data in the RX FIFO that has passed frame filtering. FIFOP is set high when there is at least one complete frame in the RX FIFO. FIFOP is high during RX FIFO overflow."]
    #[inline(always)]
    pub fn fifop(&self) -> FifopR {
        FifopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO is high when there is data in the RX FIFO. FIFO is low during RX FIFO overflow."]
    #[inline(always)]
    pub fn fifo(&self) -> FifoR {
        FifoR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Radio status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmstat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsmstat1Spec;
impl crate::RegisterSpec for Fsmstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsmstat1::R`](R) reader structure"]
impl crate::Readable for Fsmstat1Spec {}
#[doc = "`reset()` method sets FSMSTAT1 to value 0"]
impl crate::Resettable for Fsmstat1Spec {
    const RESET_VALUE: u32 = 0;
}
