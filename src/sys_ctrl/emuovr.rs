#[doc = "Register `EMUOVR` reader"]
pub struct R(crate::R<EMUOVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMUOVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMUOVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMUOVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMUOVR` writer"]
pub struct W(crate::W<EMUOVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMUOVR_SPEC>;
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
impl From<crate::W<EMUOVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMUOVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICEPICK_FORCE_CLOCK_CG` reader - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub struct ICEPICK_FORCE_CLOCK_CG_R(crate::FieldReader<bool, bool>);
impl ICEPICK_FORCE_CLOCK_CG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEPICK_FORCE_CLOCK_CG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEPICK_FORCE_CLOCK_CG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEPICK_FORCE_CLOCK_CG` writer - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub struct ICEPICK_FORCE_CLOCK_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_FORCE_CLOCK_CG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ICEPICK_FORCE_POWER_CG` reader - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub struct ICEPICK_FORCE_POWER_CG_R(crate::FieldReader<bool, bool>);
impl ICEPICK_FORCE_POWER_CG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEPICK_FORCE_POWER_CG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEPICK_FORCE_POWER_CG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEPICK_FORCE_POWER_CG` writer - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub struct ICEPICK_FORCE_POWER_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_FORCE_POWER_CG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_CG` reader - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub struct ICEPICK_INHIBIT_SLEEP_CG_R(crate::FieldReader<bool, bool>);
impl ICEPICK_INHIBIT_SLEEP_CG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEPICK_INHIBIT_SLEEP_CG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEPICK_INHIBIT_SLEEP_CG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_CG` writer - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
pub struct ICEPICK_INHIBIT_SLEEP_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_INHIBIT_SLEEP_CG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ICEMELTER_WKUP_CG` reader - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
pub struct ICEMELTER_WKUP_CG_R(crate::FieldReader<bool, bool>);
impl ICEMELTER_WKUP_CG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEMELTER_WKUP_CG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEMELTER_WKUP_CG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEMELTER_WKUP_CG` writer - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
pub struct ICEMELTER_WKUP_CG_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEMELTER_WKUP_CG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ICEPICK_FORCE_CLOCK_PM` reader - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub struct ICEPICK_FORCE_CLOCK_PM_R(crate::FieldReader<bool, bool>);
impl ICEPICK_FORCE_CLOCK_PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEPICK_FORCE_CLOCK_PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEPICK_FORCE_CLOCK_PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEPICK_FORCE_CLOCK_PM` writer - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub struct ICEPICK_FORCE_CLOCK_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_FORCE_CLOCK_PM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ICEPICK_FORCE_POWER_PM` reader - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub struct ICEPICK_FORCE_POWER_PM_R(crate::FieldReader<bool, bool>);
impl ICEPICK_FORCE_POWER_PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEPICK_FORCE_POWER_PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEPICK_FORCE_POWER_PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEPICK_FORCE_POWER_PM` writer - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub struct ICEPICK_FORCE_POWER_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_FORCE_POWER_PM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_PM` reader - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub struct ICEPICK_INHIBIT_SLEEP_PM_R(crate::FieldReader<bool, bool>);
impl ICEPICK_INHIBIT_SLEEP_PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEPICK_INHIBIT_SLEEP_PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEPICK_INHIBIT_SLEEP_PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEPICK_INHIBIT_SLEEP_PM` writer - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub struct ICEPICK_INHIBIT_SLEEP_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEPICK_INHIBIT_SLEEP_PM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ICEMELTER_WKUP_PM` reader - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub struct ICEMELTER_WKUP_PM_R(crate::FieldReader<bool, bool>);
impl ICEMELTER_WKUP_PM_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICEMELTER_WKUP_PM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICEMELTER_WKUP_PM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICEMELTER_WKUP_PM` writer - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
pub struct ICEMELTER_WKUP_PM_W<'a> {
    w: &'a mut W,
}
impl<'a> ICEMELTER_WKUP_PM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_clock_cg(&self) -> ICEPICK_FORCE_CLOCK_CG_R {
        ICEPICK_FORCE_CLOCK_CG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_power_cg(&self) -> ICEPICK_FORCE_POWER_CG_R {
        ICEPICK_FORCE_POWER_CG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_cg(&self) -> ICEPICK_INHIBIT_SLEEP_CG_R {
        ICEPICK_INHIBIT_SLEEP_CG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
    #[inline(always)]
    pub fn icemelter_wkup_cg(&self) -> ICEMELTER_WKUP_CG_R {
        ICEMELTER_WKUP_CG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_clock_pm(&self) -> ICEPICK_FORCE_CLOCK_PM_R {
        ICEPICK_FORCE_CLOCK_PM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_power_pm(&self) -> ICEPICK_FORCE_POWER_PM_R {
        ICEPICK_FORCE_POWER_PM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_pm(&self) -> ICEPICK_INHIBIT_SLEEP_PM_R {
        ICEPICK_INHIBIT_SLEEP_PM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icemelter_wkup_pm(&self) -> ICEMELTER_WKUP_PM_R {
        ICEMELTER_WKUP_PM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - ICEPick 'Force Active' clock gate override bit. 'Force Active' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_clock_cg(&mut self) -> ICEPICK_FORCE_CLOCK_CG_W {
        ICEPICK_FORCE_CLOCK_CG_W { w: self }
    }
    #[doc = "Bit 6 - ICEPick 'Force Power' clock gate override bit. 'Force Power' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_force_power_cg(&mut self) -> ICEPICK_FORCE_POWER_CG_W {
        ICEPICK_FORCE_POWER_CG_W { w: self }
    }
    #[doc = "Bit 5 - ICEPick 'Inhibit Sleep' clock gate override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_cg(&mut self) -> ICEPICK_INHIBIT_SLEEP_CG_W {
        ICEPICK_INHIBIT_SLEEP_CG_W { w: self }
    }
    #[doc = "Bit 4 - ICEMelter 'WAKEUPEMU' clock gate override bit. 1 --> In non-sleep power mode, peripherals clocks are forced to follow RCG* register settings. It forces CM3 clocks on. 0 --> Does not affect the peripheral clock settings"]
    #[inline(always)]
    pub fn icemelter_wkup_cg(&mut self) -> ICEMELTER_WKUP_CG_W {
        ICEMELTER_WKUP_CG_W { w: self }
    }
    #[doc = "Bit 3 - ICEPick 'Force Active' power mode override bit. 'Force Active' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_clock_pm(&mut self) -> ICEPICK_FORCE_CLOCK_PM_W {
        ICEPICK_FORCE_CLOCK_PM_W { w: self }
    }
    #[doc = "Bit 2 - ICEPick 'Force Power' power mode override bit. 'Force Power' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_force_power_pm(&mut self) -> ICEPICK_FORCE_POWER_PM_W {
        ICEPICK_FORCE_POWER_PM_W { w: self }
    }
    #[doc = "Bit 1 - ICEPick 'Inhibit Sleep' power mode override bit. 'Inhibit Sleep' is an ICEPick command. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icepick_inhibit_sleep_pm(&mut self) -> ICEPICK_INHIBIT_SLEEP_PM_W {
        ICEPICK_INHIBIT_SLEEP_PM_W { w: self }
    }
    #[doc = "Bit 0 - ICEMelter 'WAKEUPEMU' power mode override bit. 1 --> Prohibit the system to go into any power down modes. Keeps the emulator attached. 0 --> Does not override any power mode settings from SYSREGS and does not prohibit system to go into any power down modes."]
    #[inline(always)]
    pub fn icemelter_wkup_pm(&mut self) -> ICEMELTER_WKUP_PM_W {
        ICEMELTER_WKUP_PM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the emulator override controls for power mode and peripheral clock gate.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emuovr](index.html) module"]
pub struct EMUOVR_SPEC;
impl crate::RegisterSpec for EMUOVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emuovr::R](R) reader structure"]
impl crate::Readable for EMUOVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emuovr::W](W) writer structure"]
impl crate::Writable for EMUOVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EMUOVR to value 0"]
impl crate::Resettable for EMUOVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
