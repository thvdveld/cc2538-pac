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
pub struct LPBKI2C_R(crate::FieldReader<bool, bool>);
impl LPBKI2C_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LPBKI2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPBKI2C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPBKI2C` writer - I2C0 Master/slave loopback mode 0: Normal mode"]
pub struct LPBKI2C_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBKI2C_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&self) -> LPBKI2C_R {
        LPBKI2C_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&mut self) -> LPBKI2C_W {
        LPBKI2C_W { w: self }
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
