#[doc = "Register `ADCCON3` reader"]
pub struct R(crate::R<ADCCON3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCON3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCON3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCON3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCON3` writer"]
pub struct W(crate::W<ADCCON3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCON3_SPEC>;
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
impl From<crate::W<ADCCON3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCON3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECH` reader - Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
pub type ECH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ECH` writer - Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
pub type ECH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCON3_SPEC, u8, u8, 4, O>;
#[doc = "Field `EDIV` reader - Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
pub type EDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDIV` writer - Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
pub type EDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCON3_SPEC, u8, u8, 2, O>;
#[doc = "Field `EREF` reader - Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
pub type EREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EREF` writer - Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
pub type EREF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADCCON3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    pub fn ech(&self) -> ECH_R {
        ECH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
    #[inline(always)]
    pub fn ediv(&self) -> EDIV_R {
        EDIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    pub fn eref(&self) -> EREF_R {
        EREF_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    pub fn ech(&mut self) -> ECH_W<0> {
        ECH_W::new(self)
    }
    #[doc = "Bits 4:5 - Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
    #[inline(always)]
    pub fn ediv(&mut self) -> EDIV_W<4> {
        EDIV_W::new(self)
    }
    #[doc = "Bits 6:7 - Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    pub fn eref(&mut self) -> EREF_W<6> {
        EREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the ADC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adccon3](index.html) module"]
pub struct ADCCON3_SPEC;
impl crate::RegisterSpec for ADCCON3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adccon3::R](R) reader structure"]
impl crate::Readable for ADCCON3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adccon3::W](W) writer structure"]
impl crate::Writable for ADCCON3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCON3 to value 0"]
impl crate::Resettable for ADCCON3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
