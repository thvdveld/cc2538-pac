#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXMIS` reader - SSI SSITXINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSITXINTR"]
pub type TXMIS_R = crate::BitReader<bool>;
#[doc = "Field `RXMIS` reader - SSI SSIRXINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRXINTR"]
pub type RXMIS_R = crate::BitReader<bool>;
#[doc = "Field `RTMIS` reader - SSI SSIRTINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRTINTR"]
pub type RTMIS_R = crate::BitReader<bool>;
#[doc = "Field `RORMIS` reader - SSI SSIRORINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRORINTR"]
pub type RORMIS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 3 - SSI SSITXINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSITXINTR"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - SSI SSIRXINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRXINTR"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - SSI SSIRTINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRTINTR"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - SSI SSIRORINTR masked state (RO) Reset value: 0x0 Gives the interrupt state (after masking) of SSIRORINTR"]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "The MIS register is the masked interrupt status register. On a read, this register gives the current masked status value of the corresponding interrupt. A write has no effect.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
