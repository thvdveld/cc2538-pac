#[doc = "Register `ADCCON3` reader"]
pub type R = crate::R<Adccon3Spec>;
#[doc = "Register `ADCCON3` writer"]
pub type W = crate::W<Adccon3Spec>;
#[doc = "Field `ECH` reader - Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
pub type EchR = crate::FieldReader;
#[doc = "Field `ECH` writer - Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
pub type EchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EDIV` reader - Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
pub type EdivR = crate::FieldReader;
#[doc = "Field `EDIV` writer - Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
pub type EdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EREF` reader - Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
pub type ErefR = crate::FieldReader;
#[doc = "Field `EREF` writer - Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
pub type ErefW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    pub fn ech(&self) -> EchR {
        EchR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
    #[inline(always)]
    pub fn ediv(&self) -> EdivR {
        EdivR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    pub fn eref(&self) -> ErefR {
        ErefR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Single channel select. Selects the channel number of the single conversion that is triggered by writing to ADCCON3. 0000: AIN0 0001: AIN1 0010: AIN2 0011: AIN3 0100: AIN4 0101: AIN5 0110: AIN6 0111: AIN7 1000: AIN0-AIN1 1001: AIN2-AIN3 1010: AIN4-AIN5 1011: AIN6-AIN7 1100: GND 1101: Reserved 1110: Temperature sensor 1111: VDD/3"]
    #[inline(always)]
    pub fn ech(&mut self) -> EchW<Adccon3Spec> {
        EchW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Sets the decimation rate used for the extra conversion The decimation rate also determines the resolution and the time required to complete the conversion. 00: 64 decimation rate (7 bits ENOB) 01: 128 decimation rate (9 bits ENOB) 10: 256 decimation rate (10 bits ENOB) 11: 512 decimation rate (12 bits ENOB)"]
    #[inline(always)]
    pub fn ediv(&mut self) -> EdivW<Adccon3Spec> {
        EdivW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Selects reference voltage used for the extra conversion 00: Internal reference 01: External reference on AIN7 pin 10: AVDD5 pin 11: External reference on AIN6-AIN7 differential input"]
    #[inline(always)]
    pub fn eref(&mut self) -> ErefW<Adccon3Spec> {
        ErefW::new(self, 6)
    }
}
#[doc = "This register controls the ADC.\n\nYou can [`read`](crate::Reg::read) this register and get [`adccon3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccon3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adccon3Spec;
impl crate::RegisterSpec for Adccon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adccon3::R`](R) reader structure"]
impl crate::Readable for Adccon3Spec {}
#[doc = "`write(|w| ..)` method takes [`adccon3::W`](W) writer structure"]
impl crate::Writable for Adccon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCON3 to value 0"]
impl crate::Resettable for Adccon3Spec {
    const RESET_VALUE: u32 = 0;
}
