#[doc = "Register `LPBKGPT` reader"]
pub type R = crate::R<LPBKGPT_SPEC>;
#[doc = "Register `LPBKGPT` writer"]
pub type W = crate::W<LPBKGPT_SPEC>;
#[doc = "Field `LPBK16GPT0` reader - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT0_R = crate::FieldReader;
#[doc = "Field `LPBK16GPT0` writer - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPBK16GPT1` reader - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT1_R = crate::FieldReader;
#[doc = "Field `LPBK16GPT1` writer - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPBK16GPT2` reader - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT2_R = crate::FieldReader;
#[doc = "Field `LPBK16GPT2` writer - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPBK16GPT3` reader - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT3_R = crate::FieldReader;
#[doc = "Field `LPBK16GPT3` writer - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPBK32GPT1` reader - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT1_R = crate::FieldReader;
#[doc = "Field `LPBK32GPT1` writer - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPBK32GPT2` reader - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT2_R = crate::FieldReader;
#[doc = "Field `LPBK32GPT2` writer - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `LPBK32GPT3` reader - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT3_R = crate::FieldReader;
#[doc = "Field `LPBK32GPT3` writer - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt0(&self) -> LPBK16GPT0_R {
        LPBK16GPT0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt1(&self) -> LPBK16GPT1_R {
        LPBK16GPT1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt2(&self) -> LPBK16GPT2_R {
        LPBK16GPT2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt3(&self) -> LPBK16GPT3_R {
        LPBK16GPT3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt1(&self) -> LPBK32GPT1_R {
        LPBK32GPT1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt2(&self) -> LPBK32GPT2_R {
        LPBK32GPT2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt3(&self) -> LPBK32GPT3_R {
        LPBK32GPT3_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk16gpt0(&mut self) -> LPBK16GPT0_W<LPBKGPT_SPEC, 0> {
        LPBK16GPT0_W::new(self)
    }
    #[doc = "Bits 2:3 - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk16gpt1(&mut self) -> LPBK16GPT1_W<LPBKGPT_SPEC, 2> {
        LPBK16GPT1_W::new(self)
    }
    #[doc = "Bits 4:5 - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk16gpt2(&mut self) -> LPBK16GPT2_W<LPBKGPT_SPEC, 4> {
        LPBK16GPT2_W::new(self)
    }
    #[doc = "Bits 6:7 - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk16gpt3(&mut self) -> LPBK16GPT3_W<LPBKGPT_SPEC, 6> {
        LPBK16GPT3_W::new(self)
    }
    #[doc = "Bits 16:17 - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk32gpt1(&mut self) -> LPBK32GPT1_W<LPBKGPT_SPEC, 16> {
        LPBK32GPT1_W::new(self)
    }
    #[doc = "Bits 18:19 - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk32gpt2(&mut self) -> LPBK32GPT2_W<LPBKGPT_SPEC, 18> {
        LPBK32GPT2_W::new(self)
    }
    #[doc = "Bits 20:21 - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk32gpt3(&mut self) -> LPBK32GPT3_W<LPBKGPT_SPEC, 20> {
        LPBK32GPT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTIMER Internal loopback\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbkgpt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbkgpt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPBKGPT_SPEC;
impl crate::RegisterSpec for LPBKGPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbkgpt::R`](R) reader structure"]
impl crate::Readable for LPBKGPT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpbkgpt::W`](W) writer structure"]
impl crate::Writable for LPBKGPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPBKGPT to value 0"]
impl crate::Resettable for LPBKGPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
