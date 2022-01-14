#[doc = "Register `FSMSTAT0` reader"]
pub struct R(crate::R<FSMSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSMSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSMSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSMSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAL_DONE` reader - Frequency synthesis calibration has been performed since the last time the FS was turned on."]
pub struct CAL_DONE_R(crate::FieldReader<bool, bool>);
impl CAL_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAL_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL_RUNNING` reader - Frequency synthesis calibration status 0: Calibration is complete or not started. 1: Calibration is in progress."]
pub struct CAL_RUNNING_R(crate::FieldReader<bool, bool>);
impl CAL_RUNNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CAL_RUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_RUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSM_FFCTRL_STATE` reader - Gives the current state of the FIFO and frame control (FFCTRL) finite state-machine."]
pub struct FSM_FFCTRL_STATE_R(crate::FieldReader<u8, u8>);
impl FSM_FFCTRL_STATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FSM_FFCTRL_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSM_FFCTRL_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Frequency synthesis calibration has been performed since the last time the FS was turned on."]
    #[inline(always)]
    pub fn cal_done(&self) -> CAL_DONE_R {
        CAL_DONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Frequency synthesis calibration status 0: Calibration is complete or not started. 1: Calibration is in progress."]
    #[inline(always)]
    pub fn cal_running(&self) -> CAL_RUNNING_R {
        CAL_RUNNING_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - Gives the current state of the FIFO and frame control (FFCTRL) finite state-machine."]
    #[inline(always)]
    pub fn fsm_ffctrl_state(&self) -> FSM_FFCTRL_STATE_R {
        FSM_FFCTRL_STATE_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Radio status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsmstat0](index.html) module"]
pub struct FSMSTAT0_SPEC;
impl crate::RegisterSpec for FSMSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsmstat0::R](R) reader structure"]
impl crate::Readable for FSMSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSMSTAT0 to value 0"]
impl crate::Resettable for FSMSTAT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
