#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TFE` reader - SSI transmit FIFO empty (RO) Reset value: 0x1 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
pub type TfeR = crate::BitReader;
#[doc = "Field `TNF` reader - SSI transmit FIFO not full (RO) Reset value: 0x1 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
pub type TnfR = crate::BitReader;
#[doc = "Field `RNE` reader - SSI receive FIFO not empty (RO) Reset value: 0x0 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
pub type RneR = crate::BitReader;
#[doc = "Field `RFF` reader - SSI receive FIFO full (RO) Reset value: 0x0 0: Receive FIFO is not full. 1: Receive FIFO is full."]
pub type RffR = crate::BitReader;
#[doc = "Field `BSY` reader - SSI busy bit (RO) Reset value: 0x0 0: SSI is idle. 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
pub type BsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SSI transmit FIFO empty (RO) Reset value: 0x1 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI transmit FIFO not full (RO) Reset value: 0x1 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI receive FIFO not empty (RO) Reset value: 0x0 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI receive FIFO full (RO) Reset value: 0x0 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSI busy bit (RO) Reset value: 0x0 0: SSI is idle. 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "The SR register contains bits that indicate the FIFO fill status and the SSI busy status.\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0;
}
