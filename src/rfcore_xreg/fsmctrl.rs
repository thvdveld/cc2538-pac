#[doc = "Register `FSMCTRL` reader"]
pub type R = crate::R<FsmctrlSpec>;
#[doc = "Register `FSMCTRL` writer"]
pub type W = crate::W<FsmctrlSpec>;
#[doc = "Field `RX2RX_TIME_OFF` reader - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
pub type Rx2rxTimeOffR = crate::BitReader;
#[doc = "Field `RX2RX_TIME_OFF` writer - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
pub type Rx2rxTimeOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLOTTED_ACK` reader - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
pub type SlottedAckR = crate::BitReader;
#[doc = "Field `SLOTTED_ACK` writer - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
pub type SlottedAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
    #[inline(always)]
    pub fn rx2rx_time_off(&self) -> Rx2rxTimeOffR {
        Rx2rxTimeOffR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
    #[inline(always)]
    pub fn slotted_ack(&self) -> SlottedAckR {
        SlottedAckR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines whether or not a 12-symbol time-out should be used after frame reception has ended. 0: No time-out 1: 12-symbol-period time-out"]
    #[inline(always)]
    pub fn rx2rx_time_off(&mut self) -> Rx2rxTimeOffW<FsmctrlSpec> {
        Rx2rxTimeOffW::new(self, 0)
    }
    #[doc = "Bit 1 - Controls timing of transmission of acknowledge frames 0: The acknowledge frame is sent 12 symbol periods after the end of the received frame which requests the aknowledge. 1: The acknowledge frame is sent at the first backoff-slot boundary more than 12 symbol periods after the end of the received frame which requests the aknowledge."]
    #[inline(always)]
    pub fn slotted_ack(&mut self) -> SlottedAckW<FsmctrlSpec> {
        SlottedAckW::new(self, 1)
    }
}
#[doc = "FSM options\n\nYou can [`read`](crate::Reg::read) this register and get [`fsmctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsmctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmctrlSpec;
impl crate::RegisterSpec for FsmctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsmctrl::R`](R) reader structure"]
impl crate::Readable for FsmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fsmctrl::W`](W) writer structure"]
impl crate::Writable for FsmctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSMCTRL to value 0"]
impl crate::Resettable for FsmctrlSpec {
    const RESET_VALUE: u32 = 0;
}
