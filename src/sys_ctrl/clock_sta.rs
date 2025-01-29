#[doc = "Register `CLOCK_STA` reader"]
pub type R = crate::R<ClockStaSpec>;
#[doc = "Field `SYS_DIV` reader - Returns current functional frequency for system clock (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type SysDivR = crate::FieldReader;
#[doc = "Field `RTCLK_FREQ` reader - Returns current functional frequency for real-time clock. (may differ from setting in the CLOCK_CTRL register) 1x : 8 MHz 01: 2 MHz 00: 62.5 kHz"]
pub type RtclkFreqR = crate::FieldReader;
#[doc = "Field `IO_DIV` reader - Returns current functional frequency for IO_CLK (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type IoDivR = crate::FieldReader;
#[doc = "Field `OSC` reader - Current clock source selected 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
pub type OscR = crate::BitReader;
#[doc = "Field `OSC_PD` reader - 0: Both oscillators powered up and stable and OSC_PD_CMD = 0. 1: Oscillator not selected by CLOCK_CTRL.OSC bit is powered down."]
pub type OscPdR = crate::BitReader;
#[doc = "Field `HSOSC_STB` reader - HSOSC stable status 0: HSOSC is not powered up or not yet stable. 1: HSOSC is powered up and stable."]
pub type HsoscStbR = crate::BitReader;
#[doc = "Field `XOSC_STB` reader - XOSC stable status 0: XOSC is not powered up or not yet stable. 1: XOSC is powered up and stable."]
pub type XoscStbR = crate::BitReader;
#[doc = "Field `SOURCE_CHANGE` reader - 0: System clock is not requested to change. 1: A change of system clock source has been initiated and is not finished. Same as when OSC bit in CLOCK_STA and CLOCK_CTRL register are not equal"]
pub type SourceChangeR = crate::BitReader;
#[doc = "Field `RST` reader - Returns last source of reset 00: POR 01: External reset 10: WDT 11: CLD or software reset"]
pub type RstR = crate::FieldReader;
#[doc = "Field `OSC32K` reader - Current 32-kHz clock oscillator selected. 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
pub type Osc32kR = crate::BitReader;
#[doc = "Field `OSC32K_CALDIS` reader - Disable calibration 32-kHz RC oscillator. 0: Calibration enabled 1: Calibration disabled"]
pub type Osc32kCaldisR = crate::BitReader;
#[doc = "Field `SYNC_32K` reader - 32-kHz clock source synced to undivided system clock (16 or 32 MHz)."]
pub type Sync32kR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Returns current functional frequency for system clock (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&self) -> SysDivR {
        SysDivR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Returns current functional frequency for real-time clock. (may differ from setting in the CLOCK_CTRL register) 1x : 8 MHz 01: 2 MHz 00: 62.5 kHz"]
    #[inline(always)]
    pub fn rtclk_freq(&self) -> RtclkFreqR {
        RtclkFreqR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Returns current functional frequency for IO_CLK (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&self) -> IoDivR {
        IoDivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Current clock source selected 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OscR {
        OscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: Both oscillators powered up and stable and OSC_PD_CMD = 0. 1: Oscillator not selected by CLOCK_CTRL.OSC bit is powered down."]
    #[inline(always)]
    pub fn osc_pd(&self) -> OscPdR {
        OscPdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSOSC stable status 0: HSOSC is not powered up or not yet stable. 1: HSOSC is powered up and stable."]
    #[inline(always)]
    pub fn hsosc_stb(&self) -> HsoscStbR {
        HsoscStbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XOSC stable status 0: XOSC is not powered up or not yet stable. 1: XOSC is powered up and stable."]
    #[inline(always)]
    pub fn xosc_stb(&self) -> XoscStbR {
        XoscStbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 0: System clock is not requested to change. 1: A change of system clock source has been initiated and is not finished. Same as when OSC bit in CLOCK_STA and CLOCK_CTRL register are not equal"]
    #[inline(always)]
    pub fn source_change(&self) -> SourceChangeR {
        SourceChangeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Returns last source of reset 00: POR 01: External reset 10: WDT 11: CLD or software reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Current 32-kHz clock oscillator selected. 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&self) -> Osc32kR {
        Osc32kR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable calibration 32-kHz RC oscillator. 0: Calibration enabled 1: Calibration disabled"]
    #[inline(always)]
    pub fn osc32k_caldis(&self) -> Osc32kCaldisR {
        Osc32kCaldisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 32-kHz clock source synced to undivided system clock (16 or 32 MHz)."]
    #[inline(always)]
    pub fn sync_32k(&self) -> Sync32kR {
        Sync32kR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Clock status register This register reflects the current chip status.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock_sta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockStaSpec;
impl crate::RegisterSpec for ClockStaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock_sta::R`](R) reader structure"]
impl crate::Readable for ClockStaSpec {}
#[doc = "`reset()` method sets CLOCK_STA to value 0"]
impl crate::Resettable for ClockStaSpec {
    const RESET_VALUE: u32 = 0;
}
