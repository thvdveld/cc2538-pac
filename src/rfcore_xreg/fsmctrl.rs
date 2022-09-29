#[doc = "Register `FSMCTRL` reader"]
pub struct R(crate::R<FSMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSMCTRL` writer"]
pub struct W(crate::W<FSMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FSMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX2RX_TIME_OFF` reader - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
pub type RX2RX_TIME_OFF_R = crate::BitReader<bool>;
#[doc = "Field `RX2RX_TIME_OFF` writer - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
pub type RX2RX_TIME_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMCTRL_SPEC, bool, O>;
#[doc = "Field `SLOTTED_ACK` reader - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
pub type SLOTTED_ACK_R = crate::BitReader<bool>;
#[doc = "Field `SLOTTED_ACK` writer - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
pub type SLOTTED_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSMCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
    #[inline(always)]
    pub fn rx2rx_time_off(&self) -> RX2RX_TIME_OFF_R {
        RX2RX_TIME_OFF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
    #[inline(always)]
    pub fn slotted_ack(&self) -> SLOTTED_ACK_R {
        SLOTTED_ACK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
    #[inline(always)]
    pub fn rx2rx_time_off(&mut self) -> RX2RX_TIME_OFF_W<0> {
        RX2RX_TIME_OFF_W::new(self)
    }
    #[doc = "Bit 1 - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
    #[inline(always)]
    pub fn slotted_ack(&mut self) -> SLOTTED_ACK_W<1> {
        SLOTTED_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FSM options\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsmctrl](index.html) module"]
pub struct FSMCTRL_SPEC;
impl crate::RegisterSpec for FSMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsmctrl::R](R) reader structure"]
impl crate::Readable for FSMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsmctrl::W](W) writer structure"]
impl crate::Writable for FSMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSMCTRL to value 0"]
impl crate::Resettable for FSMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
