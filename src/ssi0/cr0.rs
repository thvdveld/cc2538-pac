#[doc = "Register `CR0` reader"]
pub type R = crate::R<CR0_SPEC>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<CR0_SPEC>;
#[doc = "Field `DSS` reader - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
pub type DSS_R = crate::FieldReader;
#[doc = "Field `DSS` writer - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
pub type DSS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `FRF` reader - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
pub type FRF_R = crate::FieldReader;
#[doc = "Field `FRF` writer - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
pub type FRF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SPO` reader - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SPO_R = crate::BitReader;
#[doc = "Field `SPO` writer - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SPO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPH` reader - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SPH_R = crate::BitReader;
#[doc = "Field `SPH` writer - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SPH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SCR` reader - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
pub type SCR_R = crate::FieldReader;
#[doc = "Field `SCR` writer - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
pub type SCR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:3 - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
    #[inline(always)]
    pub fn dss(&self) -> DSS_R {
        DSS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn spo(&self) -> SPO_R {
        SPO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn sph(&self) -> SPH_R {
        SPH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&self) -> SCR_R {
        SCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
    #[inline(always)]
    #[must_use]
    pub fn dss(&mut self) -> DSS_W<CR0_SPEC, 0> {
        DSS_W::new(self)
    }
    #[doc = "Bits 4:5 - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<CR0_SPEC, 4> {
        FRF_W::new(self)
    }
    #[doc = "Bit 6 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    #[must_use]
    pub fn spo(&mut self) -> SPO_W<CR0_SPEC, 6> {
        SPO_W::new(self)
    }
    #[doc = "Bit 7 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    #[must_use]
    pub fn sph(&mut self) -> SPH_W<CR0_SPEC, 7> {
        SPH_W::new(self)
    }
    #[doc = "Bits 8:15 - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    #[must_use]
    pub fn scr(&mut self) -> SCR_W<CR0_SPEC, 8> {
        SCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The CR0 register contains bit fields that control various functions within the SSI module. Functionality such as protocol mode, clock rate, and data size are configured in this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for CR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
