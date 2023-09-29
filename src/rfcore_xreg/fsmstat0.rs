#[doc = "Register `FSMSTAT0` reader"]
pub type R = crate::R<FSMSTAT0_SPEC>;
#[doc = "Field `FSM_FFCTRL_STATE` reader - Gives the current state of the FIFO and frame control (FFCTRL) finite state-machine."]
pub type FSM_FFCTRL_STATE_R = crate::FieldReader;
#[doc = "Field `CAL_RUNNING` reader - Frequency synthesis calibration status 0: Calibration is complete or not started. 1: Calibration is in progress."]
pub type CAL_RUNNING_R = crate::BitReader;
#[doc = "Field `CAL_DONE` reader - Frequency synthesis calibration has been performed since the last time the FS was turned on."]
pub type CAL_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Gives the current state of the FIFO and frame control (FFCTRL) finite state-machine."]
    #[inline(always)]
    pub fn fsm_ffctrl_state(&self) -> FSM_FFCTRL_STATE_R {
        FSM_FFCTRL_STATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Frequency synthesis calibration status 0: Calibration is complete or not started. 1: Calibration is in progress."]
    #[inline(always)]
    pub fn cal_running(&self) -> CAL_RUNNING_R {
        CAL_RUNNING_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frequency synthesis calibration has been performed since the last time the FS was turned on."]
    #[inline(always)]
    pub fn cal_done(&self) -> CAL_DONE_R {
        CAL_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Radio status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsmstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSMSTAT0_SPEC;
impl crate::RegisterSpec for FSMSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsmstat0::R`](R) reader structure"]
impl crate::Readable for FSMSTAT0_SPEC {}
#[doc = "`reset()` method sets FSMSTAT0 to value 0"]
impl crate::Resettable for FSMSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
