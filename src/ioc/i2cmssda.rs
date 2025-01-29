#[doc = "Register `I2CMSSDA` reader"]
pub type R = crate::R<I2cmssdaSpec>;
#[doc = "Register `I2CMSSDA` writer"]
pub type W = crate::W<I2cmssdaSpec>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
pub type InputSelR = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
pub type InputSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
    #[inline(always)]
    pub fn input_sel(&self) -> InputSelR {
        InputSelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
    #[inline(always)]
    pub fn input_sel(&mut self) -> InputSelW<I2cmssdaSpec> {
        InputSelW::new(self, 0)
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA.\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cmssda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cmssda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cmssdaSpec;
impl crate::RegisterSpec for I2cmssdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cmssda::R`](R) reader structure"]
impl crate::Readable for I2cmssdaSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cmssda::W`](W) writer structure"]
impl crate::Writable for I2cmssdaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2CMSSDA to value 0"]
impl crate::Resettable for I2cmssdaSpec {
    const RESET_VALUE: u32 = 0;
}
