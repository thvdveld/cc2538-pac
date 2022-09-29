#[doc = "Register `MTCTRL` reader"]
pub struct R(crate::R<MTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTCTRL` writer"]
pub struct W(crate::W<MTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTCTRL_SPEC>;
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
impl From<crate::W<MTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUN` reader - Write 1 to start timer, write 0 to stop timer. When read, it returns the last written value."]
pub type RUN_R = crate::BitReader<bool>;
#[doc = "Field `RUN` writer - Write 1 to start timer, write 0 to stop timer. When read, it returns the last written value."]
pub type RUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTCTRL_SPEC, bool, O>;
#[doc = "Field `SYNC` reader - 0: Starting and stopping of timer is immediate; that is, synchronous with clk_rf_32m. 1: Starting and stopping of timer occurs at the first positive edge of the 32-kHz clock. For more details regarding timer start and stop, see Section 22.4."]
pub type SYNC_R = crate::BitReader<bool>;
#[doc = "Field `SYNC` writer - 0: Starting and stopping of timer is immediate; that is, synchronous with clk_rf_32m. 1: Starting and stopping of timer occurs at the first positive edge of the 32-kHz clock. For more details regarding timer start and stop, see Section 22.4."]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTCTRL_SPEC, bool, O>;
#[doc = "Field `STATE` reader - State of MAC Timer 0: Timer idle 1: Timer running"]
pub type STATE_R = crate::BitReader<bool>;
#[doc = "Field `LATCH_MODE` reader - 0: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer, making it ready to be read from MTM1. Reading MTMOVF0 with MTMSEL.MTMOVFSEL = 000 latches the two most-significant bytes of the overflow counter, making it possible to read these from MTMOVF1 and MTMOVF2. 1: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer and the entire overflow counter at once, making it possible to read the values from MTM1, MTMOVF0, MTMOVF1, and MTMOVF2."]
pub type LATCH_MODE_R = crate::BitReader<bool>;
#[doc = "Field `LATCH_MODE` writer - 0: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer, making it ready to be read from MTM1. Reading MTMOVF0 with MTMSEL.MTMOVFSEL = 000 latches the two most-significant bytes of the overflow counter, making it possible to read these from MTMOVF1 and MTMOVF2. 1: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer and the entire overflow counter at once, making it possible to read the values from MTM1, MTMOVF0, MTMOVF1, and MTMOVF2."]
pub type LATCH_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write 1 to start timer, write 0 to stop timer. When read, it returns the last written value."]
    #[inline(always)]
    pub fn run(&self) -> RUN_R {
        RUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Starting and stopping of timer is immediate; that is, synchronous with clk_rf_32m. 1: Starting and stopping of timer occurs at the first positive edge of the 32-kHz clock. For more details regarding timer start and stop, see Section 22.4."]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - State of MAC Timer 0: Timer idle 1: Timer running"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer, making it ready to be read from MTM1. Reading MTMOVF0 with MTMSEL.MTMOVFSEL = 000 latches the two most-significant bytes of the overflow counter, making it possible to read these from MTMOVF1 and MTMOVF2. 1: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer and the entire overflow counter at once, making it possible to read the values from MTM1, MTMOVF0, MTMOVF1, and MTMOVF2."]
    #[inline(always)]
    pub fn latch_mode(&self) -> LATCH_MODE_R {
        LATCH_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to start timer, write 0 to stop timer. When read, it returns the last written value."]
    #[inline(always)]
    pub fn run(&mut self) -> RUN_W<0> {
        RUN_W::new(self)
    }
    #[doc = "Bit 1 - 0: Starting and stopping of timer is immediate; that is, synchronous with clk_rf_32m. 1: Starting and stopping of timer occurs at the first positive edge of the 32-kHz clock. For more details regarding timer start and stop, see Section 22.4."]
    #[inline(always)]
    pub fn sync(&mut self) -> SYNC_W<1> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 3 - 0: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer, making it ready to be read from MTM1. Reading MTMOVF0 with MTMSEL.MTMOVFSEL = 000 latches the two most-significant bytes of the overflow counter, making it possible to read these from MTMOVF1 and MTMOVF2. 1: Reading MTM0 with MTMSEL.MTMSEL = 000 latches the high byte of the timer and the entire overflow counter at once, making it possible to read the values from MTM1, MTMOVF0, MTMOVF1, and MTMOVF2."]
    #[inline(always)]
    pub fn latch_mode(&mut self) -> LATCH_MODE_W<3> {
        LATCH_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtctrl](index.html) module"]
pub struct MTCTRL_SPEC;
impl crate::RegisterSpec for MTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtctrl::R](R) reader structure"]
impl crate::Readable for MTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtctrl::W](W) writer structure"]
impl crate::Writable for MTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTCTRL to value 0"]
impl crate::Resettable for MTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
