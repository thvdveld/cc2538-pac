#[doc = "Register `LTIM` reader"]
pub type R = crate::R<LtimSpec>;
#[doc = "Field `TIMER` reader - Timer value This field contains the value of the free-running timer."]
pub type TimerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Timer value This field contains the value of the free-running timer."]
    #[inline(always)]
    pub fn timer(&self) -> TimerR {
        TimerR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "UART LIN timer The LTIM register contains the current timer value for the free-running timer that is used to calculate the baud rate when in LIN slave mode. The value in this register is used along with the value in the UART LIN snap shot (LSS) register to adjust the baud rate to match that of the master.\n\nYou can [`read`](crate::Reg::read) this register and get [`ltim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LtimSpec;
impl crate::RegisterSpec for LtimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltim::R`](R) reader structure"]
impl crate::Readable for LtimSpec {}
#[doc = "`reset()` method sets LTIM to value 0"]
impl crate::Resettable for LtimSpec {
    const RESET_VALUE: u32 = 0;
}
