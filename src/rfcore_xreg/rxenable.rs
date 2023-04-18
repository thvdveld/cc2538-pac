#[doc = "Register `RXENABLE` reader"]
pub struct R(crate::R<RXENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXENMASK` reader - RXENABLE enables the receiver. A nonzero value in this register causes FFCTRL to enable the receiver when in idle, after transmission and after acknowledgement transmission. The following strobes can modify RXENMASK: SRXON: Set bit 7 in RXENMASK. STXON: Set bit 6 in RXENMASK if SET_RXENMASK_ON_TX = 1. SRFOFF: Clears all bits in RXENMASK. SRXMASKBITSET: Set bit 5 in RXENMASK. SRXMASKBITCLR: Clear bit 5 in RXENMASK. There could be conflicts between the CSP and xreg_bus write operations if both operations try to modify RXENMASK simultaneously. To handle the case of simultaneous access to RXENMASK the following rules apply: - If the two sources agree (they modify different parts of the register) both of their requests to modify RXENMASK are processed. - If both operations try to modify the mask simultaneously, bus write operations to RXMASKSET and RXMASKCLR have priority over the CSP. This situation must be avoided."]
pub type RXENMASK_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RXENABLE enables the receiver. A nonzero value in this register causes FFCTRL to enable the receiver when in idle, after transmission and after acknowledgement transmission. The following strobes can modify RXENMASK: SRXON: Set bit 7 in RXENMASK. STXON: Set bit 6 in RXENMASK if SET_RXENMASK_ON_TX = 1. SRFOFF: Clears all bits in RXENMASK. SRXMASKBITSET: Set bit 5 in RXENMASK. SRXMASKBITCLR: Clear bit 5 in RXENMASK. There could be conflicts between the CSP and xreg_bus write operations if both operations try to modify RXENMASK simultaneously. To handle the case of simultaneous access to RXENMASK the following rules apply: - If the two sources agree (they modify different parts of the register) both of their requests to modify RXENMASK are processed. - If both operations try to modify the mask simultaneously, bus write operations to RXMASKSET and RXMASKCLR have priority over the CSP. This situation must be avoided."]
    #[inline(always)]
    pub fn rxenmask(&self) -> RXENMASK_R {
        RXENMASK_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX enabling\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxenable](index.html) module"]
pub struct RXENABLE_SPEC;
impl crate::RegisterSpec for RXENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxenable::R](R) reader structure"]
impl crate::Readable for RXENABLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXENABLE to value 0"]
impl crate::Resettable for RXENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
