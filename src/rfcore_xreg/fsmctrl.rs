#[doc = "Register `FSMCTRL` reader"]
pub type R = crate::R<FSMCTRL_SPEC>;
#[doc = "Register `FSMCTRL` writer"]
pub type W = crate::W<FSMCTRL_SPEC>;
#[doc = "Field `RX2RX_TIME_OFF` reader - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
pub type RX2RX_TIME_OFF_R = crate::BitReader;
#[doc = "Field `RX2RX_TIME_OFF` writer - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
pub type RX2RX_TIME_OFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SLOTTED_ACK` reader - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
pub type SLOTTED_ACK_R = crate::BitReader;
#[doc = "Field `SLOTTED_ACK` writer - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
pub type SLOTTED_ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    #[must_use]
    pub fn rx2rx_time_off(&mut self) -> RX2RX_TIME_OFF_W<FSMCTRL_SPEC, 0> {
        RX2RX_TIME_OFF_W::new(self)
    }
    #[doc = "Bit 1 - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
    #[inline(always)]
    #[must_use]
    pub fn slotted_ack(&mut self) -> SLOTTED_ACK_W<FSMCTRL_SPEC, 1> {
        SLOTTED_ACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "FSM options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsmctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsmctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSMCTRL_SPEC;
impl crate::RegisterSpec for FSMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsmctrl::R`](R) reader structure"]
impl crate::Readable for FSMCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsmctrl::W`](W) writer structure"]
impl crate::Writable for FSMCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSMCTRL to value 0"]
impl crate::Resettable for FSMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
