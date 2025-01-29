#[doc = "Register `LPBKGPT` reader"]
pub type R = crate::R<LpbkgptSpec>;
#[doc = "Register `LPBKGPT` writer"]
pub type W = crate::W<LpbkgptSpec>;
#[doc = "Field `LPBK16GPT0` reader - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type Lpbk16gpt0R = crate::FieldReader;
#[doc = "Field `LPBK16GPT0` writer - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type Lpbk16gpt0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPBK16GPT1` reader - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type Lpbk16gpt1R = crate::FieldReader;
#[doc = "Field `LPBK16GPT1` writer - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type Lpbk16gpt1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPBK16GPT2` reader - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type Lpbk16gpt2R = crate::FieldReader;
#[doc = "Field `LPBK16GPT2` writer - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type Lpbk16gpt2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPBK16GPT3` reader - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type Lpbk16gpt3R = crate::FieldReader;
#[doc = "Field `LPBK16GPT3` writer - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
pub type Lpbk16gpt3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPBK32GPT1` reader - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type Lpbk32gpt1R = crate::FieldReader;
#[doc = "Field `LPBK32GPT1` writer - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type Lpbk32gpt1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPBK32GPT2` reader - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type Lpbk32gpt2R = crate::FieldReader;
#[doc = "Field `LPBK32GPT2` writer - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
pub type Lpbk32gpt2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPBK32GPT3` reader - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
pub type Lpbk32gpt3R = crate::FieldReader;
#[doc = "Field `LPBK32GPT3` writer - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
pub type Lpbk32gpt3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt0(&self) -> Lpbk16gpt0R {
        Lpbk16gpt0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt1(&self) -> Lpbk16gpt1R {
        Lpbk16gpt1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt2(&self) -> Lpbk16gpt2R {
        Lpbk16gpt2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt3(&self) -> Lpbk16gpt3R {
        Lpbk16gpt3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt1(&self) -> Lpbk32gpt1R {
        Lpbk32gpt1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt2(&self) -> Lpbk32gpt2R {
        Lpbk32gpt2R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt3(&self) -> Lpbk32gpt3R {
        Lpbk32gpt3R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTimer0 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt0(&mut self) -> Lpbk16gpt0W<LpbkgptSpec> {
        Lpbk16gpt0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPTimer1 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt1(&mut self) -> Lpbk16gpt1W<LpbkgptSpec> {
        Lpbk16gpt1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPTimer2 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt2(&mut self) -> Lpbk16gpt2W<LpbkgptSpec> {
        Lpbk16gpt2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPTimer3 16-bit loopback modes 00: Normal operation 01: Timer A PWM connected to Timer B capture 10: Timer A capture connected to Timer B PWM 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk16gpt3(&mut self) -> Lpbk16gpt3W<LpbkgptSpec> {
        Lpbk16gpt3W::new(self, 6)
    }
    #[doc = "Bits 16:17 - GPTimer1 32-bit RTC loopback modes 00: Normal operation 01: GPT0 timerA PWM connected to GPT1 capture 10: GPT0 capture connected to GPT1 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt1(&mut self) -> Lpbk32gpt1W<LpbkgptSpec> {
        Lpbk32gpt1W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPTimer2 32-bit RTC loopback modes 00: Normal operation 01: GPT0 Timer A PWM connected to GPT2 capture 10: GPT0 capture connected to GPT2 PWM Timer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt2(&mut self) -> Lpbk32gpt2W<LpbkgptSpec> {
        Lpbk32gpt2W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPTimer3 32-bit RTC loopback modes 00: Normal operation 01: GPT0 GPTimerA PWM connected to GPT3 capture 10: GPT0 capture connected to GPT3 PWM GPTimer A 11: Reserved, defaults to normal operation"]
    #[inline(always)]
    pub fn lpbk32gpt3(&mut self) -> Lpbk32gpt3W<LpbkgptSpec> {
        Lpbk32gpt3W::new(self, 20)
    }
}
#[doc = "GPTIMER Internal loopback\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbkgpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbkgpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpbkgptSpec;
impl crate::RegisterSpec for LpbkgptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbkgpt::R`](R) reader structure"]
impl crate::Readable for LpbkgptSpec {}
#[doc = "`write(|w| ..)` method takes [`lpbkgpt::W`](W) writer structure"]
impl crate::Writable for LpbkgptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPBKGPT to value 0"]
impl crate::Resettable for LpbkgptSpec {
    const RESET_VALUE: u32 = 0;
}
