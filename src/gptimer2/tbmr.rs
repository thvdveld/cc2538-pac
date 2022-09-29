#[doc = "Register `TBMR` reader"]
pub struct R(crate::R<TBMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBMR` writer"]
pub struct W(crate::W<TBMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBMR_SPEC>;
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
impl From<crate::W<TBMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBMR` reader - GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
pub type TBMR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBMR` writer - GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
pub type TBMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBMR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TBCMR` reader - GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
pub type TBCMR_R = crate::BitReader<bool>;
#[doc = "Field `TBCMR` writer - GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
pub type TBCMR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBAMS` reader - GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
pub type TBAMS_R = crate::BitReader<bool>;
#[doc = "Field `TBAMS` writer - GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
pub type TBAMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBCDIR` reader - GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
pub type TBCDIR_R = crate::BitReader<bool>;
#[doc = "Field `TBCDIR` writer - GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
pub type TBCDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBMIE` reader - GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
pub type TBMIE_R = crate::BitReader<bool>;
#[doc = "Field `TBMIE` writer - GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
pub type TBMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBWOT` reader - GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
pub type TBWOT_R = crate::BitReader<bool>;
#[doc = "Field `TBWOT` writer - GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
pub type TBWOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBSNAPS` reader - GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
pub type TBSNAPS_R = crate::BitReader<bool>;
#[doc = "Field `TBSNAPS` writer - GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
pub type TBSNAPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBILD` reader - GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
pub type TBILD_R = crate::BitReader<bool>;
#[doc = "Field `TBILD` writer - GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
pub type TBILD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBPWMIE` reader - GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
pub type TBPWMIE_R = crate::BitReader<bool>;
#[doc = "Field `TBPWMIE` writer - GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
pub type TBPWMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBMRSU` reader - Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
pub type TBMRSU_R = crate::BitReader<bool>;
#[doc = "Field `TBMRSU` writer - Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
pub type TBMRSU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
#[doc = "Field `TBPLO` reader - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
pub type TBPLO_R = crate::BitReader<bool>;
#[doc = "Field `TBPLO` writer - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
pub type TBPLO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tbcmr(&self) -> TBCMR_R {
        TBCMR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn tbams(&self) -> TBAMS_R {
        TBAMS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tbcdir(&self) -> TBCDIR_R {
        TBCDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tbmie(&self) -> TBMIE_R {
        TBMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
    #[inline(always)]
    pub fn tbwot(&self) -> TBWOT_R {
        TBWOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
    #[inline(always)]
    pub fn tbsnaps(&self) -> TBSNAPS_R {
        TBSNAPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
    #[inline(always)]
    pub fn tbild(&self) -> TBILD_R {
        TBILD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tbpwmie(&self) -> TBPWMIE_R {
        TBPWMIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tbmrsu(&self) -> TBMRSU_R {
        TBMRSU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    pub fn tbplo(&self) -> TBPLO_R {
        TBPLO_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
    #[inline(always)]
    pub fn tbmr(&mut self) -> TBMR_W<0> {
        TBMR_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tbcmr(&mut self) -> TBCMR_W<2> {
        TBCMR_W::new(self)
    }
    #[doc = "Bit 3 - GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn tbams(&mut self) -> TBAMS_W<3> {
        TBAMS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tbcdir(&mut self) -> TBCDIR_W<4> {
        TBCDIR_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tbmie(&mut self) -> TBMIE_W<5> {
        TBMIE_W::new(self)
    }
    #[doc = "Bit 6 - GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
    #[inline(always)]
    pub fn tbwot(&mut self) -> TBWOT_W<6> {
        TBWOT_W::new(self)
    }
    #[doc = "Bit 7 - GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
    #[inline(always)]
    pub fn tbsnaps(&mut self) -> TBSNAPS_W<7> {
        TBSNAPS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
    #[inline(always)]
    pub fn tbild(&mut self) -> TBILD_W<8> {
        TBILD_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tbpwmie(&mut self) -> TBPWMIE_W<9> {
        TBPWMIE_W::new(self)
    }
    #[doc = "Bit 10 - Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tbmrsu(&mut self) -> TBMRSU_W<10> {
        TBMRSU_W::new(self)
    }
    #[doc = "Bit 11 - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    pub fn tbplo(&mut self) -> TBPLO_W<11> {
        TBPLO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbmr](index.html) module"]
pub struct TBMR_SPEC;
impl crate::RegisterSpec for TBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbmr::R](R) reader structure"]
impl crate::Readable for TBMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbmr::W](W) writer structure"]
impl crate::Writable for TBMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TBMR to value 0"]
impl crate::Resettable for TBMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
