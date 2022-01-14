#[doc = "Register `CLD` reader"]
pub struct R(crate::R<CLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLD` writer"]
pub struct W(crate::W<CLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLD_SPEC>;
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
impl From<crate::W<CLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALID` reader - 0: CLD status in always-on domain is not equal to status in the EN register. 1: CLD status in always-on domain and EN register are equal."]
pub struct VALID_R(crate::FieldReader<bool, bool>);
impl VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` reader - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bit 8 - 0: CLD status in always-on domain is not equal to status in the EN register. 1: CLD status in always-on domain and EN register are equal."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: CLD is disabled. 1: CLD is enabled. Writing to this register shall be ignored if VALID = 0"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls the clock loss detection feature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cld](index.html) module"]
pub struct CLD_SPEC;
impl crate::RegisterSpec for CLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cld::R](R) reader structure"]
impl crate::Readable for CLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cld::W](W) writer structure"]
impl crate::Writable for CLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLD to value 0"]
impl crate::Resettable for CLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
