#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `DSS` reader - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
pub type DssR = crate::FieldReader;
#[doc = "Field `DSS` writer - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FRF` reader - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
pub type FrfR = crate::FieldReader;
#[doc = "Field `FRF` writer - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPO` reader - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SpoR = crate::BitReader;
#[doc = "Field `SPO` writer - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SpoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPH` reader - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SphR = crate::BitReader;
#[doc = "Field `SPH` writer - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SphW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCR` reader - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
pub type ScrR = crate::FieldReader;
#[doc = "Field `SCR` writer - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn spo(&self) -> SpoR {
        SpoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn sph(&self) -> SphR {
        SphR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
    #[inline(always)]
    pub fn dss(&mut self) -> DssW<Cr0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 4:5 - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<Cr0Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 6 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn spo(&mut self) -> SpoW<Cr0Spec> {
        SpoW::new(self, 6)
    }
    #[doc = "Bit 7 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn sph(&mut self) -> SphW<Cr0Spec> {
        SphW::new(self, 7)
    }
    #[doc = "Bits 8:15 - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<Cr0Spec> {
        ScrW::new(self, 8)
    }
}
#[doc = "The CR0 register contains bit fields that control various functions within the SSI module. Functionality such as protocol mode, clock rate, and data size are configured in this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {
    const RESET_VALUE: u32 = 0;
}
