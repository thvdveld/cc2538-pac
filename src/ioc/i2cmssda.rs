#[doc = "Register `I2CMSSDA` reader"]
pub type R = crate::R<I2CMSSDA_SPEC>;
#[doc = "Register `I2CMSSDA` writer"]
pub type W = crate::W<I2CMSSDA_SPEC>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
pub type INPUT_SEL_R = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
pub type INPUT_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
    #[inline(always)]
    #[must_use]
    pub fn input_sel(&mut self) -> INPUT_SEL_W<I2CMSSDA_SPEC, 0> {
        INPUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cmssda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cmssda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2CMSSDA_SPEC;
impl crate::RegisterSpec for I2CMSSDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cmssda::R`](R) reader structure"]
impl crate::Readable for I2CMSSDA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2cmssda::W`](W) writer structure"]
impl crate::Writable for I2CMSSDA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2CMSSDA to value 0"]
impl crate::Resettable for I2CMSSDA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
