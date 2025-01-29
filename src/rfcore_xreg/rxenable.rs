#[doc = "Register `RXENABLE` reader"]
pub type R = crate::R<RxenableSpec>;
#[doc = "Field `RXENMASK` reader - RXENABLE enables the receiver. A nonzero value in this register causes FFCTRL to enable the receiver when in idle, after transmission and after acknowledgement transmission. The following strobes can modify RXENMASK: SRXON: Set bit 7 in RXENMASK. STXON: Set bit 6 in RXENMASK if SET_RXENMASK_ON_TX = 1. SRFOFF: Clears all bits in RXENMASK. SRXMASKBITSET: Set bit 5 in RXENMASK. SRXMASKBITCLR: Clear bit 5 in RXENMASK. There could be conflicts between the CSP and xreg_bus write operations if both operations try to modify RXENMASK simultaneously. To handle the case of simultaneous access to RXENMASK the following rules apply: - If the two sources agree (they modify different parts of the register) both of their requests to modify RXENMASK are processed. - If both operations try to modify the mask simultaneously, bus write operations to RXMASKSET and RXMASKCLR have priority over the CSP. This situation must be avoided."]
pub type RxenmaskR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RXENABLE enables the receiver. A nonzero value in this register causes FFCTRL to enable the receiver when in idle, after transmission and after acknowledgement transmission. The following strobes can modify RXENMASK: SRXON: Set bit 7 in RXENMASK. STXON: Set bit 6 in RXENMASK if SET_RXENMASK_ON_TX = 1. SRFOFF: Clears all bits in RXENMASK. SRXMASKBITSET: Set bit 5 in RXENMASK. SRXMASKBITCLR: Clear bit 5 in RXENMASK. There could be conflicts between the CSP and xreg_bus write operations if both operations try to modify RXENMASK simultaneously. To handle the case of simultaneous access to RXENMASK the following rules apply: - If the two sources agree (they modify different parts of the register) both of their requests to modify RXENMASK are processed. - If both operations try to modify the mask simultaneously, bus write operations to RXMASKSET and RXMASKCLR have priority over the CSP. This situation must be avoided."]
    #[inline(always)]
    pub fn rxenmask(&self) -> RxenmaskR {
        RxenmaskR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RX enabling\n\nYou can [`read`](crate::Reg::read) this register and get [`rxenable::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxenableSpec;
impl crate::RegisterSpec for RxenableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxenable::R`](R) reader structure"]
impl crate::Readable for RxenableSpec {}
#[doc = "`reset()` method sets RXENABLE to value 0"]
impl crate::Resettable for RxenableSpec {
    const RESET_VALUE: u32 = 0;
}
