#[doc = "Register `CLOCK_STA` reader"]
pub struct R(crate::R<CLOCK_STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS_DIV` reader - Returns current functional frequency for system clock (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type SYS_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTCLK_FREQ` reader - Returns current functional frequency for real-time clock. (may differ from setting in the CLOCK_CTRL register) 1x : 8 MHz 01: 2 MHz 00: 62.5 kHz"]
pub type RTCLK_FREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IO_DIV` reader - Returns current functional frequency for IO_CLK (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
pub type IO_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSC` reader - Current clock source selected 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
pub type OSC_R = crate::BitReader<bool>;
#[doc = "Field `OSC_PD` reader - 0: Both oscillators powered up and stable and OSC_PD_CMD = 0. 1: Oscillator not selected by CLOCK_CTRL.OSC bit is powered down."]
pub type OSC_PD_R = crate::BitReader<bool>;
#[doc = "Field `HSOSC_STB` reader - HSOSC stable status 0: HSOSC is not powered up or not yet stable. 1: HSOSC is powered up and stable."]
pub type HSOSC_STB_R = crate::BitReader<bool>;
#[doc = "Field `XOSC_STB` reader - XOSC stable status 0: XOSC is not powered up or not yet stable. 1: XOSC is powered up and stable."]
pub type XOSC_STB_R = crate::BitReader<bool>;
#[doc = "Field `SOURCE_CHANGE` reader - 0: System clock is not requested to change. 1: A change of system clock source has been initiated and is not finished. Same as when OSC bit in CLOCK_STA and CLOCK_CTRL register are not equal"]
pub type SOURCE_CHANGE_R = crate::BitReader<bool>;
#[doc = "Field `RST` reader - Returns last source of reset 00: POR 01: External reset 10: WDT 11: CLD or software reset"]
pub type RST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSC32K` reader - Current 32-kHz clock oscillator selected. 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
pub type OSC32K_R = crate::BitReader<bool>;
#[doc = "Field `OSC32K_CALDIS` reader - Disable calibration 32-kHz RC oscillator. 0: Calibration enabled 1: Calibration disabled"]
pub type OSC32K_CALDIS_R = crate::BitReader<bool>;
#[doc = "Field `SYNC_32K` reader - 32-kHz clock source synced to undivided system clock (16 or 32 MHz)."]
pub type SYNC_32K_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Returns current functional frequency for system clock (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn sys_div(&self) -> SYS_DIV_R {
        SYS_DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Returns current functional frequency for real-time clock. (may differ from setting in the CLOCK_CTRL register) 1x : 8 MHz 01: 2 MHz 00: 62.5 kHz"]
    #[inline(always)]
    pub fn rtclk_freq(&self) -> RTCLK_FREQ_R {
        RTCLK_FREQ_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Returns current functional frequency for IO_CLK (may differ from setting in the CLOCK_CTRL register) 000: 32 MHz 001: 16 MHz 010: 8 MHz 011: 4 MHz 100: 2 MHz 101: 1 MHz 110: 0.5 MHz 111: 0.25 MHz"]
    #[inline(always)]
    pub fn io_div(&self) -> IO_DIV_R {
        IO_DIV_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Current clock source selected 0: 32-MHz crystal oscillator 1: 16-MHz HF-RC oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: Both oscillators powered up and stable and OSC_PD_CMD = 0. 1: Oscillator not selected by CLOCK_CTRL.OSC bit is powered down."]
    #[inline(always)]
    pub fn osc_pd(&self) -> OSC_PD_R {
        OSC_PD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSOSC stable status 0: HSOSC is not powered up or not yet stable. 1: HSOSC is powered up and stable."]
    #[inline(always)]
    pub fn hsosc_stb(&self) -> HSOSC_STB_R {
        HSOSC_STB_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - XOSC stable status 0: XOSC is not powered up or not yet stable. 1: XOSC is powered up and stable."]
    #[inline(always)]
    pub fn xosc_stb(&self) -> XOSC_STB_R {
        XOSC_STB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 0: System clock is not requested to change. 1: A change of system clock source has been initiated and is not finished. Same as when OSC bit in CLOCK_STA and CLOCK_CTRL register are not equal"]
    #[inline(always)]
    pub fn source_change(&self) -> SOURCE_CHANGE_R {
        SOURCE_CHANGE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Returns last source of reset 00: POR 01: External reset 10: WDT 11: CLD or software reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Current 32-kHz clock oscillator selected. 0: 32-kHz crystal oscillator 1: 32-kHz RC oscillator"]
    #[inline(always)]
    pub fn osc32k(&self) -> OSC32K_R {
        OSC32K_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable calibration 32-kHz RC oscillator. 0: Calibration enabled 1: Calibration disabled"]
    #[inline(always)]
    pub fn osc32k_caldis(&self) -> OSC32K_CALDIS_R {
        OSC32K_CALDIS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 32-kHz clock source synced to undivided system clock (16 or 32 MHz)."]
    #[inline(always)]
    pub fn sync_32k(&self) -> SYNC_32K_R {
        SYNC_32K_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Clock status register This register reflects the current chip status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_sta](index.html) module"]
pub struct CLOCK_STA_SPEC;
impl crate::RegisterSpec for CLOCK_STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_sta::R](R) reader structure"]
impl crate::Readable for CLOCK_STA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLOCK_STA to value 0"]
impl crate::Resettable for CLOCK_STA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
