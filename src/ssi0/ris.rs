#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RORRIS` reader - SSI SSIRORINTR raw state (RO) Reset value: 0x0 Gives the raw interrupt state (before masking) of SSIRORINTR"]
pub type RorrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - SSI SSIRTINTR raw state (RO) Reset value: 0x0 Gives the raw interrupt state (before masking) of SSIRTINTR"]
pub type RtrisR = crate::BitReader;
#[doc = "Field `RXRIS` reader - SSI SSIRXINTR raw state (RO) Reset value: 0x0 Gives the raw interrupt state (before masking) of SSIRXINTR"]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - SSI SSITXINTR raw state (RO) Reset value: 0x1 Gives the raw interrupt state (before masking) of SSITXINTR"]
pub type TxrisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SSI SSIRORINTR raw state (RO) Reset value: 0x0 Gives the raw interrupt state (before masking) of SSIRORINTR"]
    #[inline(always)]
    pub fn rorris(&self) -> RorrisR {
        RorrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI SSIRTINTR raw state (RO) Reset value: 0x0 Gives the raw interrupt state (before masking) of SSIRTINTR"]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI SSIRXINTR raw state (RO) Reset value: 0x0 Gives the raw interrupt state (before masking) of SSIRXINTR"]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SSI SSITXINTR raw state (RO) Reset value: 0x1 Gives the raw interrupt state (before masking) of SSITXINTR"]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "The RIS register is the raw interrupt status register. On a read, this register gives the current raw status value of the corresponding interrupt before masking. A write has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
