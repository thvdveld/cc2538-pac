#[doc = "Register `DCGCI2C` reader"]
pub struct R(crate::R<DCGCI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCGCI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCGCI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCGCI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCGCI2C` writer"]
pub struct W(crate::W<DCGCI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCGCI2C_SPEC>;
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
impl From<crate::W<DCGCI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCGCI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0` reader - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCGCI2C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2C0_W<0> {
        I2C0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgci2c](index.html) module"]
pub struct DCGCI2C_SPEC;
impl crate::RegisterSpec for DCGCI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcgci2c::R](R) reader structure"]
impl crate::Readable for DCGCI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcgci2c::W](W) writer structure"]
impl crate::Writable for DCGCI2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCGCI2C to value 0"]
impl crate::Resettable for DCGCI2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
