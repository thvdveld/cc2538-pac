#[doc = "Register `STAT` reader"]
pub type R = crate::R<STAT_SPEC>;
#[doc = "Field `MASTEN` reader - Master enable status 0: The uDMA controller is disabled. 1: The uDMA controller is enabled."]
pub type MASTEN_R = crate::BitReader;
#[doc = "Field `STATE` reader - Control state machine status This field shows the current status of the control state-machine. Status can be one of the following: 0x0: Idle 0x1: Reading channel controller data 0x2: Reading source end pointer 0x3: Reading destination end pointer 0x4: Reading source data 0x5: Writing destination data 0x6: Waiting for uDMA request to clear 0x7: Writing channel controller data 0x8: Stalled 0x9: Done 0xA-0xF: Undefined"]
pub type STATE_R = crate::FieldReader;
#[doc = "Field `DMACHANS` reader - Available uDMA channels minus 1 This field contains a value equal to the number of uDMA channels the uDMA controller is configured to use, minus one. The value of 0x1F corresponds to 32 uDMA channels."]
pub type DMACHANS_R = crate::FieldReader;
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
#[doc = "DMA status The STAT register returns the status of the uDMA controller. This register cannot be read when the uDMA controller is in the reset state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for STAT_SPEC {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: u32 = 0;
}
