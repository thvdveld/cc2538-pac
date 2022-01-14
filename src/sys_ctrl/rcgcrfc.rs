#[doc = "Register `RCGCRFC` reader"]
pub struct R(crate::R<RCGCRFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCRFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCRFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCRFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCRFC` writer"]
pub struct W(crate::W<RCGCRFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCRFC_SPEC>;
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
impl From<crate::W<RCGCRFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCRFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC0` reader - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub struct RFC0_R(crate::FieldReader<bool, bool>);
impl RFC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFC0` writer - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
pub struct RFC0_W<'a> {
    w: &'a mut W,
}
impl<'a> RFC0_W<'a> {
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
    #[doc = "Bit 0 - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    pub fn rfc0(&self) -> RFC0_R {
        RFC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for RF CORE is gated. 1: Clock for RF CORE is enabled."]
    #[inline(always)]
    pub fn rfc0(&mut self) -> RFC0_W {
        RFC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for RF CORE when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcrfc](index.html) module"]
pub struct RCGCRFC_SPEC;
impl crate::RegisterSpec for RCGCRFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgcrfc::R](R) reader structure"]
impl crate::Readable for RCGCRFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgcrfc::W](W) writer structure"]
impl crate::Writable for RCGCRFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCRFC to value 0"]
impl crate::Resettable for RCGCRFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
