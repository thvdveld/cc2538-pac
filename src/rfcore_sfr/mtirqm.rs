#[doc = "Register `MTIRQM` reader"]
pub struct R(crate::R<MTIRQM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIRQM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIRQM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIRQM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTIRQM` writer"]
pub struct W(crate::W<MTIRQM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIRQM_SPEC>;
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
impl From<crate::W<MTIRQM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIRQM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACTIMER_OVF_COMPARE2M` reader - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
pub struct MACTIMER_OVF_COMPARE2M_R(crate::FieldReader<bool, bool>);
impl MACTIMER_OVF_COMPARE2M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_OVF_COMPARE2M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_OVF_COMPARE2M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_OVF_COMPARE2M` writer - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
pub struct MACTIMER_OVF_COMPARE2M_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_COMPARE2M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `MACTIMER_OVF_COMPARE1M` reader - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
pub struct MACTIMER_OVF_COMPARE1M_R(crate::FieldReader<bool, bool>);
impl MACTIMER_OVF_COMPARE1M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_OVF_COMPARE1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_OVF_COMPARE1M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_OVF_COMPARE1M` writer - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
pub struct MACTIMER_OVF_COMPARE1M_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_COMPARE1M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `MACTIMER_OVF_PERM` reader - Enables the MACTIMER_OVF_PER interrupt"]
pub struct MACTIMER_OVF_PERM_R(crate::FieldReader<bool, bool>);
impl MACTIMER_OVF_PERM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_OVF_PERM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_OVF_PERM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_OVF_PERM` writer - Enables the MACTIMER_OVF_PER interrupt"]
pub struct MACTIMER_OVF_PERM_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_PERM_W<'a> {
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
#[doc = "Field `MACTIMER_COMPARE2M` reader - Enables the MACTIMER_COMPARE2 interrupt"]
pub struct MACTIMER_COMPARE2M_R(crate::FieldReader<bool, bool>);
impl MACTIMER_COMPARE2M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_COMPARE2M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_COMPARE2M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_COMPARE2M` writer - Enables the MACTIMER_COMPARE2 interrupt"]
pub struct MACTIMER_COMPARE2M_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_COMPARE2M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `MACTIMER_COMPARE1M` reader - Enables the MACTIMER_COMPARE1 interrupt"]
pub struct MACTIMER_COMPARE1M_R(crate::FieldReader<bool, bool>);
impl MACTIMER_COMPARE1M_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_COMPARE1M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_COMPARE1M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_COMPARE1M` writer - Enables the MACTIMER_COMPARE1 interrupt"]
pub struct MACTIMER_COMPARE1M_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_COMPARE1M_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `MACTIMER_PERM` reader - Enables the MACTIMER_PER interrupt"]
pub struct MACTIMER_PERM_R(crate::FieldReader<bool, bool>);
impl MACTIMER_PERM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_PERM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_PERM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_PERM` writer - Enables the MACTIMER_PER interrupt"]
pub struct MACTIMER_PERM_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_PERM_W<'a> {
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
    #[doc = "Bit 5 - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2m(&self) -> MACTIMER_OVF_COMPARE2M_R {
        MACTIMER_OVF_COMPARE2M_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1m(&self) -> MACTIMER_OVF_COMPARE1M_R {
        MACTIMER_OVF_COMPARE1M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_perm(&self) -> MACTIMER_OVF_PERM_R {
        MACTIMER_OVF_PERM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare2m(&self) -> MACTIMER_COMPARE2M_R {
        MACTIMER_COMPARE2M_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare1m(&self) -> MACTIMER_COMPARE1M_R {
        MACTIMER_COMPARE1M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_perm(&self) -> MACTIMER_PERM_R {
        MACTIMER_PERM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2m(&mut self) -> MACTIMER_OVF_COMPARE2M_W {
        MACTIMER_OVF_COMPARE2M_W { w: self }
    }
    #[doc = "Bit 4 - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1m(&mut self) -> MACTIMER_OVF_COMPARE1M_W {
        MACTIMER_OVF_COMPARE1M_W { w: self }
    }
    #[doc = "Bit 3 - Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_perm(&mut self) -> MACTIMER_OVF_PERM_W {
        MACTIMER_OVF_PERM_W { w: self }
    }
    #[doc = "Bit 2 - Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare2m(&mut self) -> MACTIMER_COMPARE2M_W {
        MACTIMER_COMPARE2M_W { w: self }
    }
    #[doc = "Bit 1 - Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare1m(&mut self) -> MACTIMER_COMPARE1M_W {
        MACTIMER_COMPARE1M_W { w: self }
    }
    #[doc = "Bit 0 - Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_perm(&mut self) -> MACTIMER_PERM_W {
        MACTIMER_PERM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtirqm](index.html) module"]
pub struct MTIRQM_SPEC;
impl crate::RegisterSpec for MTIRQM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtirqm::R](R) reader structure"]
impl crate::Readable for MTIRQM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtirqm::W](W) writer structure"]
impl crate::Writable for MTIRQM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTIRQM to value 0"]
impl crate::Resettable for MTIRQM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
