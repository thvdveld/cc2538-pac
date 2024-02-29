#[doc = "Register `TAMR` reader"]
pub type R = crate::R<TamrSpec>;
#[doc = "Register `TAMR` writer"]
pub type W = crate::W<TamrSpec>;
#[doc = "Field `TAMR` reader - GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
pub type TamrR = crate::FieldReader;
#[doc = "Field `TAMR` writer - GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
pub type TamrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TACMR` reader - GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
pub type TacmrR = crate::BitReader;
#[doc = "Field `TACMR` writer - GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
pub type TacmrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAAMS` reader - GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
pub type TaamsR = crate::BitReader;
#[doc = "Field `TAAMS` writer - GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
pub type TaamsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TACDIR` reader - GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
pub type TacdirR = crate::BitReader;
#[doc = "Field `TACDIR` writer - GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
pub type TacdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMIE` reader - GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
pub type TamieR = crate::BitReader;
#[doc = "Field `TAMIE` writer - GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
pub type TamieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAWOT` reader - GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
pub type TawotR = crate::BitReader;
#[doc = "Field `TAWOT` writer - GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
pub type TawotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASNAPS` reader - GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
pub type TasnapsR = crate::BitReader;
#[doc = "Field `TASNAPS` writer - GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
pub type TasnapsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAILD` reader - GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
pub type TaildR = crate::BitReader;
#[doc = "Field `TAILD` writer - GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
pub type TaildW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAPWMIE` reader - GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
pub type TapwmieR = crate::BitReader;
#[doc = "Field `TAPWMIE` writer - GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
pub type TapwmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMRSU` reader - Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
pub type TamrsuR = crate::BitReader;
#[doc = "Field `TAMRSU` writer - Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
pub type TamrsuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAPLO` reader - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
pub type TaploR = crate::BitReader;
#[doc = "Field `TAPLO` writer - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
pub type TaploW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
    #[inline(always)]
    pub fn tamr(&self) -> TamrR {
        TamrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tacmr(&self) -> TacmrR {
        TacmrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn taams(&self) -> TaamsR {
        TaamsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tacdir(&self) -> TacdirR {
        TacdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tamie(&self) -> TamieR {
        TamieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
    #[inline(always)]
    pub fn tawot(&self) -> TawotR {
        TawotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
    #[inline(always)]
    pub fn tasnaps(&self) -> TasnapsR {
        TasnapsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
    #[inline(always)]
    pub fn taild(&self) -> TaildR {
        TaildR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tapwmie(&self) -> TapwmieR {
        TapwmieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tamrsu(&self) -> TamrsuR {
        TamrsuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    pub fn taplo(&self) -> TaploR {
        TaploR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
    #[inline(always)]
    #[must_use]
    pub fn tamr(&mut self) -> TamrW<TamrSpec> {
        TamrW::new(self, 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    #[must_use]
    pub fn tacmr(&mut self) -> TacmrW<TamrSpec> {
        TacmrW::new(self, 2)
    }
    #[doc = "Bit 3 - GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
    #[inline(always)]
    #[must_use]
    pub fn taams(&mut self) -> TaamsW<TamrSpec> {
        TaamsW::new(self, 3)
    }
    #[doc = "Bit 4 - GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn tacdir(&mut self) -> TacdirW<TamrSpec> {
        TacdirW::new(self, 4)
    }
    #[doc = "Bit 5 - GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    #[must_use]
    pub fn tamie(&mut self) -> TamieW<TamrSpec> {
        TamieW::new(self, 5)
    }
    #[doc = "Bit 6 - GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
    #[inline(always)]
    #[must_use]
    pub fn tawot(&mut self) -> TawotW<TamrSpec> {
        TawotW::new(self, 6)
    }
    #[doc = "Bit 7 - GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
    #[inline(always)]
    #[must_use]
    pub fn tasnaps(&mut self) -> TasnapsW<TamrSpec> {
        TasnapsW::new(self, 7)
    }
    #[doc = "Bit 8 - GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
    #[inline(always)]
    #[must_use]
    pub fn taild(&mut self) -> TaildW<TamrSpec> {
        TaildW::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn tapwmie(&mut self) -> TapwmieW<TamrSpec> {
        TapwmieW::new(self, 9)
    }
    #[doc = "Bit 10 - Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn tamrsu(&mut self) -> TamrsuW<TamrSpec> {
        TamrsuW::new(self, 10)
    }
    #[doc = "Bit 11 - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    #[must_use]
    pub fn taplo(&mut self) -> TaploW<TamrSpec> {
        TaploW::new(self, 11)
    }
}
#[doc = "GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TamrSpec;
impl crate::RegisterSpec for TamrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamr::R`](R) reader structure"]
impl crate::Readable for TamrSpec {}
#[doc = "`write(|w| ..)` method takes [`tamr::W`](W) writer structure"]
impl crate::Writable for TamrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMR to value 0"]
impl crate::Resettable for TamrSpec {
    const RESET_VALUE: u32 = 0;
}
