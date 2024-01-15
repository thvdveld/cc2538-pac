#[doc = "Register `ADCCON2` reader"]
pub type R = crate::R<ADCCON2_SPEC>;
#[doc = "Register `ADCCON2` writer"]
pub type W = crate::W<ADCCON2_SPEC>;
#[doc = "Field `SCH` reader - Sequence channel select Selects the end of the sequence A sequence can either be from AIN0 to AIN7 (SCH &lt;= 7) or from differential input AIN0-AIN1 to AIN6-AIN7 (8 &lt;= SCH &lt;= 11). For other settings, only one conversions is performed. When read, these bits indicate the channel number on which a conversion is ongoing: 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
pub type SCH_R = crate::FieldReader;
#[doc = "Field `SCH` writer - Sequence channel select Selects the end of the sequence A sequence can either be from AIN0 to AIN7 (SCH &lt;= 7) or from differential input AIN0-AIN1 to AIN6-AIN7 (8 &lt;= SCH &lt;= 11). For other settings, only one conversions is performed. When read, these bits indicate the channel number on which a conversion is ongoing: 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
pub type SCH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIV` reader - Sets the decimation rate for channels included in the sequence of conversions. The decimation rate also determines the resolution and time required to complete a conversion. 00: 64 decimation rate (7 bits ENOB setting) 01: 128 decimation rate (9 bits ENOB setting) 10: 256 decimation rate (10 bits ENOB setting) 11: 512 decimation rate (12 bits ENOB setting)"]
pub type SDIV_R = crate::FieldReader;
#[doc = "Field `SDIV` writer - Sets the decimation rate for channels included in the sequence of conversions. The decimation rate also determines the resolution and time required to complete a conversion. 00: 64 decimation rate (7 bits ENOB setting) 01: 128 decimation rate (9 bits ENOB setting) 10: 256 decimation rate (10 bits ENOB setting) 11: 512 decimation rate (12 bits ENOB setting)"]
pub type SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SREF` reader - Selects reference voltage used for the sequence of conversions 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
pub type SREF_R = crate::FieldReader;
#[doc = "Field `SREF` writer - Selects reference voltage used for the sequence of conversions 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
pub type SREF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Sequence channel select Selects the end of the sequence A sequence can either be from AIN0 to AIN7 (SCH &lt;= 7) or from differential input AIN0-AIN1 to AIN6-AIN7 (8 &lt;= SCH &lt;= 11). For other settings, only one conversions is performed. When read, these bits indicate the channel number on which a conversion is ongoing: 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
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
    #[doc = "Bits 0:3 - Sequence channel select Selects the end of the sequence A sequence can either be from AIN0 to AIN7 (SCH &lt;= 7) or from differential input AIN0-AIN1 to AIN6-AIN7 (8 &lt;= SCH &lt;= 11). For other settings, only one conversions is performed. When read, these bits indicate the channel number on which a conversion is ongoing: 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    #[must_use]
    pub fn sch(&mut self) -> SCH_W<ADCCON2_SPEC> {
        SCH_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Sets the decimation rate for channels included in the sequence of conversions. The decimation rate also determines the resolution and time required to complete a conversion. 00: 64 decimation rate (7 bits ENOB setting) 01: 128 decimation rate (9 bits ENOB setting) 10: 256 decimation rate (10 bits ENOB setting) 11: 512 decimation rate (12 bits ENOB setting)"]
    #[inline(always)]
    #[must_use]
    pub fn sdiv(&mut self) -> SDIV_W<ADCCON2_SPEC> {
        SDIV_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Selects reference voltage used for the sequence of conversions 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    #[must_use]
    pub fn sref(&mut self) -> SREF_W<ADCCON2_SPEC> {
        SREF_W::new(self, 6)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls the ADC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adccon2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adccon2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCCON2_SPEC;
impl crate::RegisterSpec for ADCCON2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adccon2::R`](R) reader structure"]
impl crate::Readable for ADCCON2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adccon2::W`](W) writer structure"]
impl crate::Writable for ADCCON2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCON2 to value 0"]
impl crate::Resettable for ADCCON2_SPEC {
    const RESET_VALUE: u32 = 0;
}
