#[doc = "Register `RCGCI2C` reader"]
pub struct R(crate::R<RCGCI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCI2C` writer"]
pub struct W(crate::W<RCGCI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCI2C_SPEC>;
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
impl From<crate::W<RCGCI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0` reader - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCGCI2C_SPEC, bool, O>;
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
    #[must_use]
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
#[doc = "This register defines the module clocks for I2C when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgci2c](index.html) module"]
pub struct RCGCI2C_SPEC;
impl crate::RegisterSpec for RCGCI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgci2c::R](R) reader structure"]
impl crate::Readable for RCGCI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgci2c::W](W) writer structure"]
impl crate::Writable for RCGCI2C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGCI2C to value 0"]
impl crate::Resettable for RCGCI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
