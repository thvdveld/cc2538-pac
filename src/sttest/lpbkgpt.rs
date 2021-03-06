#[doc = "Register `LPBKGPT` reader"]
pub struct R(crate::R<LPBKGPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPBKGPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPBKGPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPBKGPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPBKGPT` writer"]
pub struct W(crate::W<LPBKGPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPBKGPT_SPEC>;
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
impl From<crate::W<LPBKGPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPBKGPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPBK32GPT3` reader - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPBK32GPT3` writer - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPBKGPT_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPBK32GPT2` reader - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPBK32GPT2` writer - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPBKGPT_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPBK32GPT1` reader - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPBK32GPT1` writer - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type LPBK32GPT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPBKGPT_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPBK16GPT3` reader - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPBK16GPT3` writer - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPBKGPT_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPBK16GPT2` reader - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPBK16GPT2` writer - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPBKGPT_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPBK16GPT1` reader - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPBK16GPT1` writer - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPBKGPT_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPBK16GPT0` reader - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPBK16GPT0` writer - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type LPBK16GPT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPBKGPT_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 20:21 - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt3(&self) -> LPBK32GPT3_R {
        LPBK32GPT3_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt2(&self) -> LPBK32GPT2_R {
        LPBK32GPT2_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt1(&self) -> LPBK32GPT1_R {
        LPBK32GPT1_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt3(&self) -> LPBK16GPT3_R {
        LPBK16GPT3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt2(&self) -> LPBK16GPT2_R {
        LPBK16GPT2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt1(&self) -> LPBK16GPT1_R {
        LPBK16GPT1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt0(&self) -> LPBK16GPT0_R {
        LPBK16GPT0_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt3(&mut self) -> LPBK32GPT3_W<20> {
        LPBK32GPT3_W::new(self)
    }
    #[doc = "Bits 18:19 - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt2(&mut self) -> LPBK32GPT2_W<18> {
        LPBK32GPT2_W::new(self)
    }
    #[doc = "Bits 16:17 - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt1(&mut self) -> LPBK32GPT1_W<16> {
        LPBK32GPT1_W::new(self)
    }
    #[doc = "Bits 6:7 - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt3(&mut self) -> LPBK16GPT3_W<6> {
        LPBK16GPT3_W::new(self)
    }
    #[doc = "Bits 4:5 - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt2(&mut self) -> LPBK16GPT2_W<4> {
        LPBK16GPT2_W::new(self)
    }
    #[doc = "Bits 2:3 - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt1(&mut self) -> LPBK16GPT1_W<2> {
        LPBK16GPT1_W::new(self)
    }
    #[doc = "Bits 0:1 - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt0(&mut self) -> LPBK16GPT0_W<0> {
        LPBK16GPT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTIMER Internal loopback\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpbkgpt](index.html) module"]
pub struct LPBKGPT_SPEC;
impl crate::RegisterSpec for LPBKGPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpbkgpt::R](R) reader structure"]
impl crate::Readable for LPBKGPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpbkgpt::W](W) writer structure"]
impl crate::Writable for LPBKGPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPBKGPT to value 0"]
impl crate::Resettable for LPBKGPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
