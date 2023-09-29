#[doc = "Register `TAMR` reader"]
pub type R = crate::R<TAMR_SPEC>;
#[doc = "Register `TAMR` writer"]
pub type W = crate::W<TAMR_SPEC>;
#[doc = "Field `TAMR` reader - GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
pub type TAMR_R = crate::FieldReader;
#[doc = "Field `TAMR` writer - GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
pub type TAMR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `TACMR` reader - GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
pub type TACMR_R = crate::BitReader;
#[doc = "Field `TACMR` writer - GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
pub type TACMR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAAMS` reader - GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
pub type TAAMS_R = crate::BitReader;
#[doc = "Field `TAAMS` writer - GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
pub type TAAMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TACDIR` reader - GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
pub type TACDIR_R = crate::BitReader;
#[doc = "Field `TACDIR` writer - GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
pub type TACDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAMIE` reader - GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
pub type TAMIE_R = crate::BitReader;
#[doc = "Field `TAMIE` writer - GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
pub type TAMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAWOT` reader - GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
pub type TAWOT_R = crate::BitReader;
#[doc = "Field `TAWOT` writer - GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
pub type TAWOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TASNAPS` reader - GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
pub type TASNAPS_R = crate::BitReader;
#[doc = "Field `TASNAPS` writer - GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
pub type TASNAPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAILD` reader - GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
pub type TAILD_R = crate::BitReader;
#[doc = "Field `TAILD` writer - GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
pub type TAILD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAPWMIE` reader - GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
pub type TAPWMIE_R = crate::BitReader;
#[doc = "Field `TAPWMIE` writer - GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
pub type TAPWMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAMRSU` reader - Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
pub type TAMRSU_R = crate::BitReader;
#[doc = "Field `TAMRSU` writer - Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
pub type TAMRSU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TAPLO` reader - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
pub type TAPLO_R = crate::BitReader;
#[doc = "Field `TAPLO` writer - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
pub type TAPLO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
    #[inline(always)]
    pub fn tamr(&self) -> TAMR_R {
        TAMR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tacmr(&self) -> TACMR_R {
        TACMR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn taams(&self) -> TAAMS_R {
        TAAMS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tacdir(&self) -> TACDIR_R {
        TACDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tamie(&self) -> TAMIE_R {
        TAMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
    #[inline(always)]
    pub fn tawot(&self) -> TAWOT_R {
        TAWOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
    #[inline(always)]
    pub fn tasnaps(&self) -> TASNAPS_R {
        TASNAPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
    #[inline(always)]
    pub fn taild(&self) -> TAILD_R {
        TAILD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tapwmie(&self) -> TAPWMIE_R {
        TAPWMIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tamrsu(&self) -> TAMRSU_R {
        TAMRSU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    pub fn taplo(&self) -> TAPLO_R {
        TAPLO_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer A mode 0x0: Reserved 0x1: One-shot mode 0x2: Periodic mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
    #[inline(always)]
    #[must_use]
    pub fn tamr(&mut self) -> TAMR_W<TAMR_SPEC, 0> {
        TAMR_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    #[must_use]
    pub fn tacmr(&mut self) -> TACMR_W<TAMR_SPEC, 2> {
        TACMR_W::new(self)
    }
    #[doc = "Bit 3 - GPTM Timer A alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TACM bit must be cleared and the TAMR field must be configured to 0x2."]
    #[inline(always)]
    #[must_use]
    pub fn taams(&mut self) -> TAAMS_W<TAMR_SPEC, 3> {
        TAAMS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn tacdir(&mut self) -> TACDIR_W<TAMR_SPEC, 4> {
        TACDIR_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTAMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    #[must_use]
    pub fn tamie(&mut self) -> TAMIE_W<TAMR_SPEC, 5> {
        TAMIE_W::new(self)
    }
    #[doc = "Bit 6 - GPTM Timer A wait-on-trigger 0: Timer A begins counting as soon as it is enabled. 1: If Timer A is enabled (TAEN is set in the GPTMCTL register), Timer A does not begin counting until it receives a trigger from the Timer in the previous position in the daisy-chain. This bit must be clear for GP Timer module 0, Timer A."]
    #[inline(always)]
    #[must_use]
    pub fn tawot(&mut self) -> TAWOT_W<TAMR_SPEC, 6> {
        TAWOT_W::new(self)
    }
    #[doc = "Bit 7 - GPTM Timer A snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer A is configured in periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPTM Timer A (GPTMTAR) register."]
    #[inline(always)]
    #[must_use]
    pub fn tasnaps(&mut self) -> TASNAPS_W<TAMR_SPEC, 7> {
        TASNAPS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer A PWM interval load write 0: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next cycle. 1: Update the GPTMTAR register with the value in the GPTMTAILR register on the next cycle. If the prescaler is used, update the GPTMTAPS register with the value in the GPTMTAPR register on the next time-out."]
    #[inline(always)]
    #[must_use]
    pub fn taild(&mut self) -> TAILD_W<TAMR_SPEC, 8> {
        TAILD_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn tapwmie(&mut self) -> TAPWMIE_W<TAMR_SPEC, 9> {
        TAPWMIE_W::new(self)
    }
    #[doc = "Bit 10 - Timer A match register update mode 0: Update GPTMAMATCHR and GPTMAPR if used on the next cycle. 1: Update GPTMAMATCHR and GPTMAPR if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTAMATCHR and GPTMTAPR are updated when the timer is enabled. If the timer is stalled (TASTALL is set), GPTMTAMATCHR and GPTMTAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn tamrsu(&mut self) -> TAMRSU_W<TAMR_SPEC, 10> {
        TAMRSU_W::new(self)
    }
    #[doc = "Bit 11 - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    #[must_use]
    pub fn taplo(&mut self) -> TAPLO_W<TAMR_SPEC, 11> {
        TAPLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTM Timer A mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer A when it is used individually. When Timer A and Timer B are concatenated, this register controls the modes for both Timer A and Timer B, and the contents of TBMR are ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMR_SPEC;
impl crate::RegisterSpec for TAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamr::R`](R) reader structure"]
impl crate::Readable for TAMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamr::W`](W) writer structure"]
impl crate::Writable for TAMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMR to value 0"]
impl crate::Resettable for TAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
