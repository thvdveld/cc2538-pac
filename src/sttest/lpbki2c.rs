#[doc = "Register `LPBKI2C` reader"]
pub struct R(crate::R<LPBKI2C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPBKI2C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPBKI2C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPBKI2C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPBKI2C` writer"]
pub struct W(crate::W<LPBKI2C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPBKI2C_SPEC>;
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
impl From<crate::W<LPBKI2C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPBKI2C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPBKI2C` reader - I2C0 Master/slave loopback mode 0: Normal mode"]
pub type LPBKI2C_R = crate::BitReader<bool>;
#[doc = "Field `LPBKI2C` writer - I2C0 Master/slave loopback mode 0: Normal mode"]
pub type LPBKI2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPBKI2C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&self) -> LPBKI2C_R {
        LPBKI2C_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&mut self) -> LPBKI2C_W<0> {
        LPBKI2C_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C internal loopback\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpbki2c](index.html) module"]
pub struct LPBKI2C_SPEC;
impl crate::RegisterSpec for LPBKI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpbki2c::R](R) reader structure"]
impl crate::Readable for LPBKI2C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpbki2c::W](W) writer structure"]
impl crate::Writable for LPBKI2C_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPBKI2C to value 0"]
impl crate::Resettable for LPBKI2C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
