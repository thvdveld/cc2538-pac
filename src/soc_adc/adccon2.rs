#[doc = "Register `ADCCON2` reader"]
pub struct R(crate::R<ADCCON2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCON2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCON2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCON2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCON2` writer"]
pub struct W(crate::W<ADCCON2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCON2_SPEC>;
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
impl From<crate::W<ADCCON2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCON2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCH` reader - Sequence channel select Selects the end of the sequence A sequence can either be from AIN0 to AIN7 (SCH <= 7) or from differential input AIN0-AIN1 to AIN6-AIN7 (8 <= SCH <= 11). For other settings, only one conversions is performed. When read, these bits indicate the channel number on which a conversion is ongoing: 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
pub type SCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCH` writer - Sequence channel select Selects the end of the sequence A sequence can either be from AIN0 to AIN7 (SCH <= 7) or from differential input AIN0-AIN1 to AIN6-AIN7 (8 <= SCH <= 11). For other settings, only one conversions is performed. When read, these bits indicate the channel number on which a conversion is ongoing: 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
pub type SCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCON2_SPEC, u8, u8, 4, O>;
#[doc = "Field `SDIV` reader - Sets the decimation rate for channels included in the sequence of conversions. The decimation rate also determines the resolution and time required to complete a conversion. 00: 64 decimation rate (7 bits ENOB setting) 01: 128 decimation rate (9 bits ENOB setting) 10: 256 decimation rate (10 bits ENOB setting) 11: 512 decimation rate (12 bits ENOB setting)"]
pub type SDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SDIV` writer - Sets the decimation rate for channels included in the sequence of conversions. The decimation rate also determines the resolution and time required to complete a conversion. 00: 64 decimation rate (7 bits ENOB setting) 01: 128 decimation rate (9 bits ENOB setting) 10: 256 decimation rate (10 bits ENOB setting) 11: 512 decimation rate (12 bits ENOB setting)"]
pub type SDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCON2_SPEC, u8, u8, 2, O>;
#[doc = "Field `SREF` reader - Selects reference voltage used for the sequence of conversions 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
pub type SREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SREF` writer - Selects reference voltage used for the sequence of conversions 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
pub type SREF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCON2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Sequence channel select Selects the end of the sequence A sequence can either be from AIN0 to AIN7 (SCH <= 7) or from differential input AIN0-AIN1 to AIN6-AIN7 (8 <= SCH <= 11). For other settings, only one conversions is performed. When read, these bits indicate the channel number on which a conversion is ongoing: 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    pub fn sch(&self) -> SCH_R {
        SCH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Sets the decimation rate for channels included in the sequence of conversions. The decimation rate also determines the resolution and time required to complete a conversion. 00: 64 decimation rate (7 bits ENOB setting) 01: 128 decimation rate (9 bits ENOB setting) 10: 256 decimation rate (10 bits ENOB setting) 11: 512 decimation rate (12 bits ENOB setting)"]
    #[inline(always)]
    pub fn sdiv(&self) -> SDIV_R {
        SDIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Selects reference voltage used for the sequence of conversions 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    pub fn sref(&self) -> SREF_R {
        SREF_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sequence channel select Selects the end of the sequence A sequence can either be from AIN0 to AIN7 (SCH <= 7) or from differential input AIN0-AIN1 to AIN6-AIN7 (8 <= SCH <= 11). For other settings, only one conversions is performed. When read, these bits indicate the channel number on which a conversion is ongoing: 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    #[must_use]
    pub fn sch(&mut self) -> SCH_W<0> {
        SCH_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets the decimation rate for channels included in the sequence of conversions. The decimation rate also determines the resolution and time required to complete a conversion. 00: 64 decimation rate (7 bits ENOB setting) 01: 128 decimation rate (9 bits ENOB setting) 10: 256 decimation rate (10 bits ENOB setting) 11: 512 decimation rate (12 bits ENOB setting)"]
    #[inline(always)]
    #[must_use]
    pub fn sdiv(&mut self) -> SDIV_W<4> {
        SDIV_W::new(self)
    }
    #[doc = "Bits 6:7 - Selects reference voltage used for the sequence of conversions 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    #[must_use]
    pub fn sref(&mut self) -> SREF_W<6> {
        SREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the ADC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adccon2](index.html) module"]
pub struct ADCCON2_SPEC;
impl crate::RegisterSpec for ADCCON2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adccon2::R](R) reader structure"]
impl crate::Readable for ADCCON2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adccon2::W](W) writer structure"]
impl crate::Writable for ADCCON2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCON2 to value 0"]
impl crate::Resettable for ADCCON2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
