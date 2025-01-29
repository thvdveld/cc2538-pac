#[doc = "Register `FSMSTAT0` reader"]
pub type R = crate::R<Fsmstat0Spec>;
#[doc = "Field `FSM_FFCTRL_STATE` reader - Gives the current state of the FIFO and frame control (FFCTRL) finite state-machine."]
pub type FsmFfctrlStateR = crate::FieldReader;
#[doc = "Field `CAL_RUNNING` reader - Frequency synthesis calibration status 0: Calibration is complete or not started. 1: Calibration is in progress."]
pub type CalRunningR = crate::BitReader;
#[doc = "Field `CAL_DONE` reader - Frequency synthesis calibration has been performed since the last time the FS was turned on."]
pub type CalDoneR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Gives the current state of the FIFO and frame control (FFCTRL) finite state-machine."]
    #[inline(always)]
    pub fn fsm_ffctrl_state(&self) -> FsmFfctrlStateR {
        FsmFfctrlStateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Frequency synthesis calibration status 0: Calibration is complete or not started. 1: Calibration is in progress."]
    #[inline(always)]
    pub fn cal_running(&self) -> CalRunningR {
        CalRunningR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frequency synthesis calibration has been performed since the last time the FS was turned on."]
    #[inline(always)]
    pub fn cal_done(&self) -> CalDoneR {
        CalDoneR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Radio status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmstat0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsmstat0Spec;
impl crate::RegisterSpec for Fsmstat0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsmstat0::R`](R) reader structure"]
impl crate::Readable for Fsmstat0Spec {}
#[doc = "`reset()` method sets FSMSTAT0 to value 0"]
impl crate::Resettable for Fsmstat0Spec {
    const RESET_VALUE: u32 = 0;
}
