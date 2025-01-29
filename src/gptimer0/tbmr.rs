#[doc = "Register `TBMR` reader"]
pub type R = crate::R<TbmrSpec>;
#[doc = "Register `TBMR` writer"]
pub type W = crate::W<TbmrSpec>;
#[doc = "Field `TBMR` reader - GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
pub type TbmrR = crate::FieldReader;
#[doc = "Field `TBMR` writer - GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
pub type TbmrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TBCMR` reader - GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
pub type TbcmrR = crate::BitReader;
#[doc = "Field `TBCMR` writer - GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
pub type TbcmrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBAMS` reader - GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
pub type TbamsR = crate::BitReader;
#[doc = "Field `TBAMS` writer - GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
pub type TbamsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBCDIR` reader - GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
pub type TbcdirR = crate::BitReader;
#[doc = "Field `TBCDIR` writer - GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
pub type TbcdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMIE` reader - GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
pub type TbmieR = crate::BitReader;
#[doc = "Field `TBMIE` writer - GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
pub type TbmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBWOT` reader - GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
pub type TbwotR = crate::BitReader;
#[doc = "Field `TBWOT` writer - GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
pub type TbwotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBSNAPS` reader - GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
pub type TbsnapsR = crate::BitReader;
#[doc = "Field `TBSNAPS` writer - GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
pub type TbsnapsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBILD` reader - GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
pub type TbildR = crate::BitReader;
#[doc = "Field `TBILD` writer - GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
pub type TbildW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBPWMIE` reader - GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
pub type TbpwmieR = crate::BitReader;
#[doc = "Field `TBPWMIE` writer - GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
pub type TbpwmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMRSU` reader - Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
pub type TbmrsuR = crate::BitReader;
#[doc = "Field `TBMRSU` writer - Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
pub type TbmrsuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBPLO` reader - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
pub type TbploR = crate::BitReader;
#[doc = "Field `TBPLO` writer - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
pub type TbploW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
    #[inline(always)]
    pub fn tbmr(&self) -> TbmrR {
        TbmrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tbcmr(&self) -> TbcmrR {
        TbcmrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn tbams(&self) -> TbamsR {
        TbamsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tbcdir(&self) -> TbcdirR {
        TbcdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tbmie(&self) -> TbmieR {
        TbmieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
    #[inline(always)]
    pub fn tbwot(&self) -> TbwotR {
        TbwotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
    #[inline(always)]
    pub fn tbsnaps(&self) -> TbsnapsR {
        TbsnapsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
    #[inline(always)]
    pub fn tbild(&self) -> TbildR {
        TbildR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tbpwmie(&self) -> TbpwmieR {
        TbpwmieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tbmrsu(&self) -> TbmrsuR {
        TbmrsuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    pub fn tbplo(&self) -> TbploR {
        TbploR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer B mode 0x0: Reserved 0x1: One-shot timer mode 0x2: Periodic timer mode 0x3: Capture mode The timer mode is based on the timer configuration defined by bits \\[2:0\\]
in the GPTMCFG register."]
    #[inline(always)]
    pub fn tbmr(&mut self) -> TbmrW<TbmrSpec> {
        TbmrW::new(self, 0)
    }
    #[doc = "Bit 2 - GPTM Timer B capture mode 0: Edge-count mode 1: Edge-time mode"]
    #[inline(always)]
    pub fn tbcmr(&mut self) -> TbcmrW<TbmrSpec> {
        TbcmrW::new(self, 2)
    }
    #[doc = "Bit 3 - GPTM Timer B alternate mode 0: Capture mode is enabled. 1: PWM mode is enabled. Note: To enable PWM mode, the TBCM bit must be cleared and the TBMR field must be configured to 0x2."]
    #[inline(always)]
    pub fn tbams(&mut self) -> TbamsW<TbmrSpec> {
        TbamsW::new(self, 3)
    }
    #[doc = "Bit 4 - GPTM Timer B count direction 0: The timer counts down. 1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn tbcdir(&mut self) -> TbcdirW<TbmrSpec> {
        TbcdirW::new(self, 4)
    }
    #[doc = "Bit 5 - GPTM Timer B match interrupt enable 0: The match interrupt is disabled. 1: An interrupt is generated when the match value in the GPTMTBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn tbmie(&mut self) -> TbmieW<TbmrSpec> {
        TbmieW::new(self, 5)
    }
    #[doc = "Bit 6 - GPTM Timer B wait-on-trigger 0: Timer B begins counting as soon as it is enabled. 1: If Timer B is enabled (TBEN is set in the GPTMCTL register), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy-chain."]
    #[inline(always)]
    pub fn tbwot(&mut self) -> TbwotW<TbmrSpec> {
        TbwotW::new(self, 6)
    }
    #[doc = "Bit 7 - GPTM Timer B snap-shot mode 0: Snap-shot mode is disabled. 1: If Timer B is configured in the periodic mode, the actual free-running value of Timer A is loaded into the GPTM Timer B (GPTMTBR) register at the time-out event."]
    #[inline(always)]
    pub fn tbsnaps(&mut self) -> TbsnapsW<TbmrSpec> {
        TbsnapsW::new(self, 7)
    }
    #[doc = "Bit 8 - GPTM Timer B PWM interval load write 0: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next cycle. 1: Update the GPTMTBR register with the value in the GPTMTBILR register on the next cycle. If the prescaler is used, update the GPTMTBPS register with the value in the GPTMTBPR register on the next time-out."]
    #[inline(always)]
    pub fn tbild(&mut self) -> TbildW<TbmrSpec> {
        TbildW::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM interrupt enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output. 0: Interrupt is disabled. 1: Interrupt is enabled. This bit is valid only in PWM mode."]
    #[inline(always)]
    pub fn tbpwmie(&mut self) -> TbpwmieW<TbmrSpec> {
        TbpwmieW::new(self, 9)
    }
    #[doc = "Bit 10 - Timer B match register update mode 0: Update the GPTMBMATCHR and the GPTMBPR, if used on the next cycle. 1: Update the GPTMBMATCHR and the GPTMBPR, if used on the next time-out. If the timer is disabled (TAEN is clear) when this bit is set, GPTMTBMATCHR and GPTMTBPR are updated when the timer is enabled. If the timer is stalled (TBSTALL is set), GPTMTBMATCHR and GPTMTBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tbmrsu(&mut self) -> TbmrsuW<TbmrSpec> {
        TbmrsuW::new(self, 10)
    }
    #[doc = "Bit 11 - Legacy PWM operation 0: Legacy operation 1: CCP is set to 1 on time-out."]
    #[inline(always)]
    pub fn tbplo(&mut self) -> TbploW<TbmrSpec> {
        TbploW::new(self, 11)
    }
}
#[doc = "GPTM Timer B mode This register configures the GPTM based on the configuration selected in the CFG register. This register controls the modes for Timer B when it is used individually. When Timer A and Timer B are concatenated, this register is ignored and TBMR controls the modes for both Timer A and Timer B.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbmrSpec;
impl crate::RegisterSpec for TbmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbmr::R`](R) reader structure"]
impl crate::Readable for TbmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbmr::W`](W) writer structure"]
impl crate::Writable for TbmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBMR to value 0"]
impl crate::Resettable for TbmrSpec {
    const RESET_VALUE: u32 = 0;
}
