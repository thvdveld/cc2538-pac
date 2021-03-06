#[doc = "Register `I2CMSSDA` reader"]
pub struct R(crate::R<I2CMSSDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CMSSDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CMSSDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CMSSDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CMSSDA` writer"]
pub struct W(crate::W<I2CMSSDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CMSSDA_SPEC>;
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
impl From<crate::W<I2CMSSDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CMSSDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
pub type INPUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as I2C SDA 1: PA1 selected as I2C SDA ... 31: PD7 selected as I2C SDA"]
pub type INPUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2CMSSDA_SPEC, u8, u8, 5, O>;
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
    pub fn input_sel(&mut self) -> INPUT_SEL_W<0> {
        INPUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the I2C SDA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cmssda](index.html) module"]
pub struct I2CMSSDA_SPEC;
impl crate::RegisterSpec for I2CMSSDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cmssda::R](R) reader structure"]
impl crate::Readable for I2CMSSDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cmssda::W](W) writer structure"]
impl crate::Writable for I2CMSSDA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2CMSSDA to value 0"]
impl crate::Resettable for I2CMSSDA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
