#[doc = "Register `CLOCK_CTRL` reader"]
pub type R = crate::R<ClockCtrlSpec>;
#[doc = "Register `CLOCK_CTRL` writer"]
pub type W = crate::W<ClockCtrlSpec>;
#[doc = "Field `SYS_DIV` reader - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type SysDivR = crate::FieldReader;
#[doc = "Field `SYS_DIV` writer - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type SysDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IO_DIV` reader - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type IoDivR = crate::FieldReader;
#[doc = "Field `IO_DIV` writer - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type IoDivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OSC` reader - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
pub type OscR = crate::BitReader;
#[doc = "Field `OSC` writer - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
pub type OscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC_PD` reader - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
pub type OscPdR = crate::BitReader;
#[doc = "Field `OSC_PD` writer - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
pub type OscPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMP_DET` reader - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
pub type AmpDetR = crate::BitReader;
#[doc = "Field `AMP_DET` writer - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
pub type AmpDetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC32K` reader - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
pub type Osc32kR = crate::BitReader;
#[doc = "Field `OSC32K` writer - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
pub type Osc32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC32K_CALDIS` reader - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
pub type Osc32kCaldisR = crate::BitReader;
#[doc = "Field `OSC32K_CALDIS` writer - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
pub type Osc32kCaldisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&self) -> SysDivR {
        SysDivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&self) -> IoDivR {
        IoDivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OscR {
        OscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
    #[inline(always)]
    pub fn osc_pd(&self) -> OscPdR {
        OscPdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
    #[inline(always)]
    pub fn amp_det(&self) -> AmpDetR {
        AmpDetR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&self) -> Osc32kR {
        Osc32kR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
    #[inline(always)]
    pub fn osc32k_caldis(&self) -> Osc32kCaldisR {
        Osc32kCaldisR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&mut self) -> SysDivW<ClockCtrlSpec> {
        SysDivW::new(self, 0)
    }
    #[doc = "Bits 8:10 - I/O clock rate setting Cannot be higher than OSC setting 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&mut self) -> IoDivW<ClockCtrlSpec> {
        IoDivW::new(self, 8)
    }
    #[doc = "Bit 16 - System clock oscillator selection 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&mut self) -> OscW<ClockCtrlSpec> {
        OscW::new(self, 16)
    }
    #[doc = "Bit 17 - 0: Power up both oscillators 1: Power down oscillator not selected by OSC bit (hardware-controlled when selected)."]
    #[inline(always)]
    pub fn osc_pd(&mut self) -> OscPdW<ClockCtrlSpec> {
        OscPdW::new(self, 17)
    }
    #[doc = "Bit 21 - Amplitude detector of XOSC during power up 0: No action 1: Delay qualification of XOSC until amplitude is greater than the threshold."]
    #[inline(always)]
    pub fn amp_det(&mut self) -> AmpDetW<ClockCtrlSpec> {
        AmpDetW::new(self, 21)
    }
    #[doc = "Bit 24 - 32-kHz clock oscillator selection 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&mut self) -> Osc32kW<ClockCtrlSpec> {
        Osc32kW::new(self, 24)
    }
    #[doc = "Bit 25 - Disable calibration 32-kHz RC oscillator. 0: Enable calibration 1: Disable calibration"]
    #[inline(always)]
    pub fn osc32k_caldis(&mut self) -> Osc32kCaldisW<ClockCtrlSpec> {
        Osc32kCaldisW::new(self, 25)
    }
}
#[doc = "The clock control register handels clock settings in the CC2538. The settings in CLOCK_CTRL do not always reflect the current chip status which is found in CLOCK_STA register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockCtrlSpec;
impl crate::RegisterSpec for ClockCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_ctrl::R`](R) reader structure"]
impl crate::Readable for ClockCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clock_ctrl::W`](W) writer structure"]
impl crate::Writable for ClockCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLOCK_CTRL to value 0"]
impl crate::Resettable for ClockCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
