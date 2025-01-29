#[doc = "Register `EMUOVR` reader"]
pub type R = crate::R<EmuovrSpec>;
#[doc = "Register `EMUOVR` writer"]
pub type W = crate::W<EmuovrSpec>;
#[doc = "Field `ICEMELTER_WKUP_PM` reader - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type IcemelterWkupPmR = crate::BitReader;
#[doc = "Field `ICEMELTER_WKUP_PM` writer - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type IcemelterWkupPmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_PM` reader - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type IcepickInhibitSleepPmR = crate::BitReader;
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_PM` writer - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type IcepickInhibitSleepPmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_FORCE_POWER_PM` reader - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type IcepickForcePowerPmR = crate::BitReader;
#[doc = "Field `ICEPICK_FORCE_POWER_PM` writer - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type IcepickForcePowerPmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_FORCE_CLOCK_PM` reader - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type IcepickForceClockPmR = crate::BitReader;
#[doc = "Field `ICEPICK_FORCE_CLOCK_PM` writer - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub type IcepickForceClockPmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEMELTER_WKUP_CG` reader - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
pub type IcemelterWkupCgR = crate::BitReader;
#[doc = "Field `ICEMELTER_WKUP_CG` writer - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
pub type IcemelterWkupCgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_CG` reader - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type IcepickInhibitSleepCgR = crate::BitReader;
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_CG` writer - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type IcepickInhibitSleepCgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_FORCE_POWER_CG` reader - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type IcepickForcePowerCgR = crate::BitReader;
#[doc = "Field `ICEPICK_FORCE_POWER_CG` writer - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type IcepickForcePowerCgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICEPICK_FORCE_CLOCK_CG` reader - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type IcepickForceClockCgR = crate::BitReader;
#[doc = "Field `ICEPICK_FORCE_CLOCK_CG` writer - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub type IcepickForceClockCgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icemelter_wkup_pm(&self) -> IcemelterWkupPmR {
        IcemelterWkupPmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_pm(&self) -> IcepickInhibitSleepPmR {
        IcepickInhibitSleepPmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_power_pm(&self) -> IcepickForcePowerPmR {
        IcepickForcePowerPmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_clock_pm(&self) -> IcepickForceClockPmR {
        IcepickForceClockPmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
    #[inline(always)]
    pub fn icemelter_wkup_cg(&self) -> IcemelterWkupCgR {
        IcemelterWkupCgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_cg(&self) -> IcepickInhibitSleepCgR {
        IcepickInhibitSleepCgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_power_cg(&self) -> IcepickForcePowerCgR {
        IcepickForcePowerCgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_clock_cg(&self) -> IcepickForceClockCgR {
        IcepickForceClockCgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icemelter_wkup_pm(&mut self) -> IcemelterWkupPmW<EmuovrSpec> {
        IcemelterWkupPmW::new(self, 0)
    }
    #[doc = "Bit 1 - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_pm(&mut self) -> IcepickInhibitSleepPmW<EmuovrSpec> {
        IcepickInhibitSleepPmW::new(self, 1)
    }
    #[doc = "Bit 2 - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_power_pm(&mut self) -> IcepickForcePowerPmW<EmuovrSpec> {
        IcepickForcePowerPmW::new(self, 2)
    }
    #[doc = "Bit 3 - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_clock_pm(&mut self) -> IcepickForceClockPmW<EmuovrSpec> {
        IcepickForceClockPmW::new(self, 3)
    }
    #[doc = "Bit 4 - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
    #[inline(always)]
    pub fn icemelter_wkup_cg(&mut self) -> IcemelterWkupCgW<EmuovrSpec> {
        IcemelterWkupCgW::new(self, 4)
    }
    #[doc = "Bit 5 - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_cg(&mut self) -> IcepickInhibitSleepCgW<EmuovrSpec> {
        IcepickInhibitSleepCgW::new(self, 5)
    }
    #[doc = "Bit 6 - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_power_cg(&mut self) -> IcepickForcePowerCgW<EmuovrSpec> {
        IcepickForcePowerCgW::new(self, 6)
    }
    #[doc = "Bit 7 - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_clock_cg(&mut self) -> IcepickForceClockCgW<EmuovrSpec> {
        IcepickForceClockCgW::new(self, 7)
    }
}
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate.\n\nYou can [`read`](crate::Reg::read) this register and get [`emuovr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emuovr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmuovrSpec;
impl crate::RegisterSpec for EmuovrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emuovr::R`](R) reader structure"]
impl crate::Readable for EmuovrSpec {}
#[doc = "`write(|w| ..)` method takes [`emuovr::W`](W) writer structure"]
impl crate::Writable for EmuovrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMUOVR to value 0"]
impl crate::Resettable for EmuovrSpec {
    const RESET_VALUE: u32 = 0;
}
