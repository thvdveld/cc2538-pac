#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUECINT` reader - GPTM write update error interrupt clear"]
pub struct WUECINT_R(crate::FieldReader<bool, bool>);
impl WUECINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WUECINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUECINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WUECINT` writer - GPTM write update error interrupt clear"]
pub struct WUECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> WUECINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TBMCINT` reader - GPTM Timer B match interrupt clear"]
pub struct TBMCINT_R(crate::FieldReader<bool, bool>);
impl TBMCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBMCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBMCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBMCINT` writer - GPTM Timer B match interrupt clear"]
pub struct TBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMCINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CBECINT` reader - GPTM Timer B capture event Interrupt clear"]
pub struct CBECINT_R(crate::FieldReader<bool, bool>);
impl CBECINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBECINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBECINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBECINT` writer - GPTM Timer B capture event Interrupt clear"]
pub struct CBECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBECINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CBMCINT` reader - GPTM Timer B capture match interrupt clear"]
pub struct CBMCINT_R(crate::FieldReader<bool, bool>);
impl CBMCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CBMCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBMCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBMCINT` writer - GPTM Timer B capture match interrupt clear"]
pub struct CBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMCINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TBTOCINT` reader - GPTM Timer B time-out interrupt clear"]
pub struct TBTOCINT_R(crate::FieldReader<bool, bool>);
impl TBTOCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TBTOCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TBTOCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TBTOCINT` writer - GPTM Timer B time-out interrupt clear"]
pub struct TBTOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOCINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `TAMCINT` reader - GPTM Timer A match interrupt clear"]
pub struct TAMCINT_R(crate::FieldReader<bool, bool>);
impl TAMCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMCINT` writer - GPTM Timer A match interrupt clear"]
pub struct TAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMCINT_W<'a> {
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
#[doc = "Field `CAECINT` reader - GPTM Timer A capture event Interrupt clear"]
pub struct CAECINT_R(crate::FieldReader<bool, bool>);
impl CAECINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAECINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAECINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAECINT` writer - GPTM Timer A capture event Interrupt clear"]
pub struct CAECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAECINT_W<'a> {
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
#[doc = "Field `CAMCINT` reader - GPTM Timer A capture match interrupt clear"]
pub struct CAMCINT_R(crate::FieldReader<bool, bool>);
impl CAMCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAMCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAMCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAMCINT` writer - GPTM Timer A capture match interrupt clear"]
pub struct CAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMCINT_W<'a> {
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
#[doc = "Field `TATOCINT` reader - GPTM Timer A time-out interrupt clear"]
pub struct TATOCINT_R(crate::FieldReader<bool, bool>);
impl TATOCINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TATOCINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TATOCINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TATOCINT` writer - GPTM Timer A time-out interrupt clear"]
pub struct TATOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOCINT_W<'a> {
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
    #[doc = "Bit 16 - GPTM write update error interrupt clear"]
    #[inline(always)]
    pub fn wuecint(&self) -> WUECINT_R {
        WUECINT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt clear"]
    #[inline(always)]
    pub fn tbmcint(&self) -> TBMCINT_R {
        TBMCINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event Interrupt clear"]
    #[inline(always)]
    pub fn cbecint(&self) -> CBECINT_R {
        CBECINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt clear"]
    #[inline(always)]
    pub fn cbmcint(&self) -> CBMCINT_R {
        CBMCINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt clear"]
    #[inline(always)]
    pub fn tbtocint(&self) -> TBTOCINT_R {
        TBTOCINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt clear"]
    #[inline(always)]
    pub fn tamcint(&self) -> TAMCINT_R {
        TAMCINT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event Interrupt clear"]
    #[inline(always)]
    pub fn caecint(&self) -> CAECINT_R {
        CAECINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt clear"]
    #[inline(always)]
    pub fn camcint(&self) -> CAMCINT_R {
        CAMCINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt clear"]
    #[inline(always)]
    pub fn tatocint(&self) -> TATOCINT_R {
        TATOCINT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - GPTM write update error interrupt clear"]
    #[inline(always)]
    pub fn wuecint(&mut self) -> WUECINT_W {
        WUECINT_W { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt clear"]
    #[inline(always)]
    pub fn tbmcint(&mut self) -> TBMCINT_W {
        TBMCINT_W { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B capture event Interrupt clear"]
    #[inline(always)]
    pub fn cbecint(&mut self) -> CBECINT_W {
        CBECINT_W { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt clear"]
    #[inline(always)]
    pub fn cbmcint(&mut self) -> CBMCINT_W {
        CBMCINT_W { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt clear"]
    #[inline(always)]
    pub fn tbtocint(&mut self) -> TBTOCINT_W {
        TBTOCINT_W { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt clear"]
    #[inline(always)]
    pub fn tamcint(&mut self) -> TAMCINT_W {
        TAMCINT_W { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A capture event Interrupt clear"]
    #[inline(always)]
    pub fn caecint(&mut self) -> CAECINT_W {
        CAECINT_W { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt clear"]
    #[inline(always)]
    pub fn camcint(&mut self) -> CAMCINT_W {
        CAMCINT_W { w: self }
    }
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt clear"]
    #[inline(always)]
    pub fn tatocint(&mut self) -> TATOCINT_W {
        TATOCINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
