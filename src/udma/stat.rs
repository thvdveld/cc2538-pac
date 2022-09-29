#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MASTEN` reader - Master enable status 0: The uDMA controller is disabled. 1: The uDMA controller is enabled."]
pub type MASTEN_R = crate::BitReader<bool>;
#[doc = "Field `STATE` reader - Control state machine status This field shows the current status of the control state-machine. Status can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source end pointer 0x3: Reading destination end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA-0xF: Undefined"]
pub type STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMACHANS` reader - Available uDMA channels minus 1 This field contains a value equal to the number of uDMA channels the uDMA controller is configured to use, minus one. The value of 0x1F corresponds to 32 uDMA channels."]
pub type DMACHANS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Master enable status 0: The uDMA controller is disabled. 1: The uDMA controller is enabled."]
    #[inline(always)]
    pub fn masten(&self) -> MASTEN_R {
        MASTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - Control state machine status This field shows the current status of the control state-machine. Status can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source end pointer 0x3: Reading destination end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA-0xF: Undefined"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Available uDMA channels minus 1 This field contains a value equal to the number of uDMA channels the uDMA controller is configured to use, minus one. The value of 0x1F corresponds to 32 uDMA channels."]
    #[inline(always)]
    pub fn dmachans(&self) -> DMACHANS_R {
        DMACHANS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
