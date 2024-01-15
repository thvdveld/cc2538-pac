#[doc = "Register `EMUOVR` reader"]
pub type R = crate::R<EMUOVR_SPEC>;
#[doc = "Register `EMUOVR` writer"]
pub type W = crate::W<EMUOVR_SPEC>;
#[doc = "Field `ICEMELTER_WKUP_PM` reader - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type ICEMELTER_WKUP_PM_R = crate::BitReader;
#[doc = "Field `ICEMELTER_WKUP_PM` writer - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type ICEMELTER_WKUP_PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_PM` reader - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type ICEPICK_INHIBIT_SLEEP_PM_R = crate::BitReader;
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_PM` writer - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type ICEPICK_INHIBIT_SLEEP_PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_FORCE_POWER_PM` reader - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type ICEPICK_FORCE_POWER_PM_R = crate::BitReader;
#[doc = "Field `ICEPICK_FORCE_POWER_PM` writer - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type ICEPICK_FORCE_POWER_PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_FORCE_CLOCK_PM` reader - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type ICEPICK_FORCE_CLOCK_PM_R = crate::BitReader;
#[doc = "Field `ICEPICK_FORCE_CLOCK_PM` writer - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type ICEPICK_FORCE_CLOCK_PM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEMELTER_WKUP_CG` reader - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
pub type ICEMELTER_WKUP_CG_R = crate::BitReader;
#[doc = "Field `ICEMELTER_WKUP_CG` writer - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
pub type ICEMELTER_WKUP_CG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_CG` reader - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type ICEPICK_INHIBIT_SLEEP_CG_R = crate::BitReader;
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_CG` writer - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type ICEPICK_INHIBIT_SLEEP_CG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_FORCE_POWER_CG` reader - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type ICEPICK_FORCE_POWER_CG_R = crate::BitReader;
#[doc = "Field `ICEPICK_FORCE_POWER_CG` writer - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type ICEPICK_FORCE_POWER_CG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_FORCE_CLOCK_CG` reader - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type ICEPICK_FORCE_CLOCK_CG_R = crate::BitReader;
#[doc = "Field `ICEPICK_FORCE_CLOCK_CG` writer - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type ICEPICK_FORCE_CLOCK_CG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icemelter_wkup_pm(&self) -> ICEMELTER_WKUP_PM_R {
        ICEMELTER_WKUP_PM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_pm(&self) -> ICEPICK_INHIBIT_SLEEP_PM_R {
        ICEPICK_INHIBIT_SLEEP_PM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_power_pm(&self) -> ICEPICK_FORCE_POWER_PM_R {
        ICEPICK_FORCE_POWER_PM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_clock_pm(&self) -> ICEPICK_FORCE_CLOCK_PM_R {
        ICEPICK_FORCE_CLOCK_PM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
    #[inline(always)]
    pub fn icemelter_wkup_cg(&self) -> ICEMELTER_WKUP_CG_R {
        ICEMELTER_WKUP_CG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_cg(&self) -> ICEPICK_INHIBIT_SLEEP_CG_R {
        ICEPICK_INHIBIT_SLEEP_CG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_power_cg(&self) -> ICEPICK_FORCE_POWER_CG_R {
        ICEPICK_FORCE_POWER_CG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_clock_cg(&self) -> ICEPICK_FORCE_CLOCK_CG_R {
        ICEPICK_FORCE_CLOCK_CG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    #[must_use]
    pub fn icemelter_wkup_pm(&mut self) -> ICEMELTER_WKUP_PM_W<EMUOVR_SPEC> {
        ICEMELTER_WKUP_PM_W::new(self, 0)
    }
    #[doc = "Bit 1 - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    #[must_use]
    pub fn icepick_inhibit_sleep_pm(&mut self) -> ICEPICK_INHIBIT_SLEEP_PM_W<EMUOVR_SPEC> {
        ICEPICK_INHIBIT_SLEEP_PM_W::new(self, 1)
    }
    #[doc = "Bit 2 - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    #[must_use]
    pub fn icepick_force_power_pm(&mut self) -> ICEPICK_FORCE_POWER_PM_W<EMUOVR_SPEC> {
        ICEPICK_FORCE_POWER_PM_W::new(self, 2)
    }
    #[doc = "Bit 3 - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    #[must_use]
    pub fn icepick_force_clock_pm(&mut self) -> ICEPICK_FORCE_CLOCK_PM_W<EMUOVR_SPEC> {
        ICEPICK_FORCE_CLOCK_PM_W::new(self, 3)
    }
    #[doc = "Bit 4 - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
    #[inline(always)]
    #[must_use]
    pub fn icemelter_wkup_cg(&mut self) -> ICEMELTER_WKUP_CG_W<EMUOVR_SPEC> {
        ICEMELTER_WKUP_CG_W::new(self, 4)
    }
    #[doc = "Bit 5 - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    #[must_use]
    pub fn icepick_inhibit_sleep_cg(&mut self) -> ICEPICK_INHIBIT_SLEEP_CG_W<EMUOVR_SPEC> {
        ICEPICK_INHIBIT_SLEEP_CG_W::new(self, 5)
    }
    #[doc = "Bit 6 - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    #[must_use]
    pub fn icepick_force_power_cg(&mut self) -> ICEPICK_FORCE_POWER_CG_W<EMUOVR_SPEC> {
        ICEPICK_FORCE_POWER_CG_W::new(self, 6)
    }
    #[doc = "Bit 7 - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    #[must_use]
    pub fn icepick_force_clock_cg(&mut self) -> ICEPICK_FORCE_CLOCK_CG_W<EMUOVR_SPEC> {
        ICEPICK_FORCE_CLOCK_CG_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emuovr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emuovr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMUOVR_SPEC;
impl crate::RegisterSpec for EMUOVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emuovr::R`](R) reader structure"]
impl crate::Readable for EMUOVR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`emuovr::W`](W) writer structure"]
impl crate::Writable for EMUOVR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMUOVR to value 0"]
impl crate::Resettable for EMUOVR_SPEC {
    const RESET_VALUE: u32 = 0;
}
