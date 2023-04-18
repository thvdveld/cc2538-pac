#[doc = "Register `SRI2C` reader"]
pub struct R(crate::R<SRI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRI2C` writer"]
pub struct W(crate::W<SRI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRI2C_SPEC>;
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
impl From<crate::W<SRI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0` reader - 0: I2C0 module is not reset 1: I2C0 module is reset"]
pub type I2C0_R = crate::BitReader<bool>;
#[doc = "Field `I2C0` writer - 0: I2C0 module is not reset 1: I2C0 module is reset"]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRI2C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: I2C0 module is not reset 1: I2C0 module is reset"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: I2C0 module is not reset 1: I2C0 module is reset"]
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
#[doc = "This register controls the reset for I2C.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sri2c](index.html) module"]
pub struct SRI2C_SPEC;
impl crate::RegisterSpec for SRI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sri2c::R](R) reader structure"]
impl crate::Readable for SRI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sri2c::W](W) writer structure"]
impl crate::Writable for SRI2C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRI2C to value 0"]
impl crate::Resettable for SRI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
