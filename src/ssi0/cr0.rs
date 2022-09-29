#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSS` reader - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
pub type DSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSS` writer - SSI data size select (R/W) Reset value: 0x0 0000-0010: Reserved 0011: 4-bit data 0100: 5-bit data 0101: 6-bit data 0110: 7-bit data 0111: 8-bit data 1000: 9-bit data 1001: 10-bit data 1010: 11-bit data 1011: 12-bit data 1100: 13-bit data 1101: 14-bit data 1110: 15-bit data 1111: 16-bit data"]
pub type DSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FRF` reader - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
pub type FRF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRF` writer - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
pub type FRF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SPO` reader - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SPO_R = crate::BitReader<bool>;
#[doc = "Field `SPO` writer - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SPO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `SPH` reader - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SPH_R = crate::BitReader<bool>;
#[doc = "Field `SPH` writer - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
pub type SPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR0_SPEC, bool, O>;
#[doc = "Field `SCR` reader - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
pub type SCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCR` writer - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
pub type SCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR0_SPEC, u8, u8, 8, O>;
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
    pub fn dss(&mut self) -> DSS_W<0> {
        DSS_W::new(self)
    }
    #[doc = "Bits 4:5 - SSI frame format select (R/W) Reset value: 0x0 00: Motorola SPI frame format 01: TI synchronous serial frame format 10: National Microwire frame format 11: Reserved"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W<4> {
        FRF_W::new(self)
    }
    #[doc = "Bit 6 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn spo(&mut self) -> SPO_W<6> {
        SPO_W::new(self)
    }
    #[doc = "Bit 7 - SSI serial clock phase (R/W) Reset value: 0x0 This bit is only applicable to the Motorola SPI Format."]
    #[inline(always)]
    pub fn sph(&mut self) -> SPH_W<7> {
        SPH_W::new(self)
    }
    #[doc = "Bits 8:15 - SSI serial clock rate (R/W) Reset value: 0x0 The value SCR is used to generate the transmit and receive bit rate of the SSI. Where the bit rate is: BR = FSSICLK/(CPSDVR * (1 + SCR)) where CPSDVR is an even value from 2-254, programmed in the SSICPSR register and SCR is a value from 0-255."]
    #[inline(always)]
    pub fn scr(&mut self) -> SCR_W<8> {
        SCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The CR0 register contains bit fields that control various functions within the SSI module. Functionality such as protocol mode, clock rate, and data size are configured in this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
