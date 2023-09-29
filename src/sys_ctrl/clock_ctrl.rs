#[doc = "Register `CLOCK_CTRL` reader"]
pub type R = crate::R<CLOCK_CTRL_SPEC>;
#[doc = "Register `CLOCK_CTRL` writer"]
pub type W = crate::W<CLOCK_CTRL_SPEC>;
#[doc = "Field `SYS_DIV` reader - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type SYS_DIV_R = crate::FieldReader;
#[doc = "Field `SYS_DIV` writer - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type SYS_DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `IO_DIV` reader - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type IO_DIV_R = crate::FieldReader;
#[doc = "Field `IO_DIV` writer - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type IO_DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `OSC` reader - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
pub type OSC_R = crate::BitReader;
#[doc = "Field `OSC` writer - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
pub type OSC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSC_PD` reader - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
pub type OSC_PD_R = crate::BitReader;
#[doc = "Field `OSC_PD` writer - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
pub type OSC_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `AMP_DET` reader - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
pub type AMP_DET_R = crate::BitReader;
#[doc = "Field `AMP_DET` writer - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
pub type AMP_DET_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSC32K` reader - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
pub type OSC32K_R = crate::BitReader;
#[doc = "Field `OSC32K` writer - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
pub type OSC32K_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OSC32K_CALDIS` reader - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
pub type OSC32K_CALDIS_R = crate::BitReader;
#[doc = "Field `OSC32K_CALDIS` writer - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
pub type OSC32K_CALDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:2 - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&self) -> SYS_DIV_R {
        SYS_DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&self) -> IO_DIV_R {
        IO_DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
    #[inline(always)]
    pub fn osc_pd(&self) -> OSC_PD_R {
        OSC_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
    #[inline(always)]
    pub fn amp_det(&self) -> AMP_DET_R {
        AMP_DET_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&self) -> OSC32K_R {
        OSC32K_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
    #[inline(always)]
    pub fn osc32k_caldis(&self) -> OSC32K_CALDIS_R {
        OSC32K_CALDIS_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sys_div(&mut self) -> SYS_DIV_W<CLOCK_CTRL_SPEC, 0> {
        SYS_DIV_W::new(self)
    }
    #[doc = "Bits 8:10 - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn io_div(&mut self) -> IO_DIV_W<CLOCK_CTRL_SPEC, 8> {
        IO_DIV_W::new(self)
    }
    #[doc = "Bit 16 - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn osc(&mut self) -> OSC_W<CLOCK_CTRL_SPEC, 16> {
        OSC_W::new(self)
    }
    #[doc = "Bit 17 - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
    #[inline(always)]
    #[must_use]
    pub fn osc_pd(&mut self) -> OSC_PD_W<CLOCK_CTRL_SPEC, 17> {
        OSC_PD_W::new(self)
    }
    #[doc = "Bit 21 - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
    #[inline(always)]
    #[must_use]
    pub fn amp_det(&mut self) -> AMP_DET_W<CLOCK_CTRL_SPEC, 21> {
        AMP_DET_W::new(self)
    }
    #[doc = "Bit 24 - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn osc32k(&mut self) -> OSC32K_W<CLOCK_CTRL_SPEC, 24> {
        OSC32K_W::new(self)
    }
    #[doc = "Bit 25 - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
    #[inline(always)]
    #[must_use]
    pub fn osc32k_caldis(&mut self) -> OSC32K_CALDIS_W<CLOCK_CTRL_SPEC, 25> {
        OSC32K_CALDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clock_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLOCK_CTRL_SPEC;
impl crate::RegisterSpec for CLOCK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_ctrl::R`](R) reader structure"]
impl crate::Readable for CLOCK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clock_ctrl::W`](W) writer structure"]
impl crate::Writable for CLOCK_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0"]
impl crate::Resettable for CLOCK_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
