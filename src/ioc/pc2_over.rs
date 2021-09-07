#[doc = "Register `PC2_OVER` reader"]
pub struct R(crate::R<PC2_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC2_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC2_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC2_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC2_OVER` writer"]
pub struct W(crate::W<PC2_OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC2_OVER_SPEC>;
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
impl From<crate::W<PC2_OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC2_OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC2_over` reader - 0: output disable 1: oe - output enable"]
pub struct PC2_OVER_R(crate::FieldReader<bool, bool>);
impl PC2_OVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PC2_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PC2_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PC2_over` writer - 0: output disable 1: oe - output enable"]
pub struct PC2_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2_OVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc2_over(&self) -> PC2_OVER_R {
        PC2_OVER_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc2_over(&mut self) -> PC2_OVER_W {
        PC2_OVER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the overide configuration register for each pad. PC2 has high drive capability.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc2_over](index.html) module"]
pub struct PC2_OVER_SPEC;
impl crate::RegisterSpec for PC2_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc2_over::R](R) reader structure"]
impl crate::Readable for PC2_OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc2_over::W](W) writer structure"]
impl crate::Writable for PC2_OVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC2_OVER to value 0x04"]
impl crate::Resettable for PC2_OVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}