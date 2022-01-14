#[doc = "Register `CLOCK_CTRL` reader"]
pub struct R(crate::R<CLOCK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCK_CTRL` writer"]
pub struct W(crate::W<CLOCK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCK_CTRL_SPEC>;
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
impl From<crate::W<CLOCK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC32K_CALDIS` reader - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
pub struct OSC32K_CALDIS_R(crate::FieldReader<bool, bool>);
impl OSC32K_CALDIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC32K_CALDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32K_CALDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32K_CALDIS` writer - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
pub struct OSC32K_CALDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32K_CALDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `OSC32K` reader - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
pub struct OSC32K_R(crate::FieldReader<bool, bool>);
impl OSC32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32K` writer - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
pub struct OSC32K_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `AMP_DET` reader - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
pub struct AMP_DET_R(crate::FieldReader<bool, bool>);
impl AMP_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AMP_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMP_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMP_DET` writer - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
pub struct AMP_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> AMP_DET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `OSC_PD` reader - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
pub struct OSC_PD_R(crate::FieldReader<bool, bool>);
impl OSC_PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC_PD` writer - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
pub struct OSC_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `OSC` reader - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
pub struct OSC_R(crate::FieldReader<bool, bool>);
impl OSC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC` writer - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
pub struct OSC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `IO_DIV` reader - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub struct IO_DIV_R(crate::FieldReader<u8, u8>);
impl IO_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_DIV` writer - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub struct IO_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `SYS_DIV` reader - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub struct SYS_DIV_R(crate::FieldReader<u8, u8>);
impl SYS_DIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYS_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYS_DIV` writer - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub struct SYS_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 25 - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
    #[inline(always)]
    pub fn osc32k_caldis(&self) -> OSC32K_CALDIS_R {
        OSC32K_CALDIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&self) -> OSC32K_R {
        OSC32K_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
    #[inline(always)]
    pub fn amp_det(&self) -> AMP_DET_R {
        AMP_DET_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
    #[inline(always)]
    pub fn osc_pd(&self) -> OSC_PD_R {
        OSC_PD_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&self) -> IO_DIV_R {
        IO_DIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&self) -> SYS_DIV_R {
        SYS_DIV_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 25 - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
    #[inline(always)]
    pub fn osc32k_caldis(&mut self) -> OSC32K_CALDIS_W {
        OSC32K_CALDIS_W { w: self }
    }
    #[doc = "Bit 24 - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&mut self) -> OSC32K_W {
        OSC32K_W { w: self }
    }
    #[doc = "Bit 21 - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
    #[inline(always)]
    pub fn amp_det(&mut self) -> AMP_DET_W {
        AMP_DET_W { w: self }
    }
    #[doc = "Bit 17 - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
    #[inline(always)]
    pub fn osc_pd(&mut self) -> OSC_PD_W {
        OSC_PD_W { w: self }
    }
    #[doc = "Bit 16 - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&mut self) -> OSC_W {
        OSC_W { w: self }
    }
    #[doc = "Bits 8:10 - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&mut self) -> IO_DIV_W {
        IO_DIV_W { w: self }
    }
    #[doc = "Bits 0:2 - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&mut self) -> SYS_DIV_W {
        SYS_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_ctrl](index.html) module"]
pub struct CLOCK_CTRL_SPEC;
impl crate::RegisterSpec for CLOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_ctrl::R](R) reader structure"]
impl crate::Readable for CLOCK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clock_ctrl::W](W) writer structure"]
impl crate::Writable for CLOCK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0"]
impl crate::Resettable for CLOCK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
