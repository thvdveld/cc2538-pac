#[doc = "Register `MTIRQF` reader"]
pub struct R(crate::R<MTIRQF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIRQF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIRQF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIRQF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTIRQF` writer"]
pub struct W(crate::W<MTIRQF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIRQF_SPEC>;
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
impl From<crate::W<MTIRQF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIRQF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACTIMER_OVF_COMPARE2F` reader - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
pub struct MACTIMER_OVF_COMPARE2F_R(crate::FieldReader<bool, bool>);
impl MACTIMER_OVF_COMPARE2F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_OVF_COMPARE2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_OVF_COMPARE2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_OVF_COMPARE2F` writer - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
pub struct MACTIMER_OVF_COMPARE2F_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_COMPARE2F_W<'a> {
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
#[doc = "Field `MACTIMER_OVF_COMPARE1F` reader - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
pub struct MACTIMER_OVF_COMPARE1F_R(crate::FieldReader<bool, bool>);
impl MACTIMER_OVF_COMPARE1F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_OVF_COMPARE1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_OVF_COMPARE1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_OVF_COMPARE1F` writer - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
pub struct MACTIMER_OVF_COMPARE1F_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_COMPARE1F_W<'a> {
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
#[doc = "Field `MACTIMER_OVF_PERF` reader - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
pub struct MACTIMER_OVF_PERF_R(crate::FieldReader<bool, bool>);
impl MACTIMER_OVF_PERF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_OVF_PERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_OVF_PERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_OVF_PERF` writer - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
pub struct MACTIMER_OVF_PERF_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_OVF_PERF_W<'a> {
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
#[doc = "Field `MACTIMER_COMPARE2F` reader - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
pub struct MACTIMER_COMPARE2F_R(crate::FieldReader<bool, bool>);
impl MACTIMER_COMPARE2F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_COMPARE2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_COMPARE2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_COMPARE2F` writer - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
pub struct MACTIMER_COMPARE2F_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_COMPARE2F_W<'a> {
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
#[doc = "Field `MACTIMER_COMPARE1F` reader - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
pub struct MACTIMER_COMPARE1F_R(crate::FieldReader<bool, bool>);
impl MACTIMER_COMPARE1F_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_COMPARE1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_COMPARE1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_COMPARE1F` writer - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
pub struct MACTIMER_COMPARE1F_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_COMPARE1F_W<'a> {
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
#[doc = "Field `MACTIMER_PERF` reader - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
pub struct MACTIMER_PERF_R(crate::FieldReader<bool, bool>);
impl MACTIMER_PERF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MACTIMER_PERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_PERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_PERF` writer - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
pub struct MACTIMER_PERF_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_PERF_W<'a> {
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
    #[doc = "Bit 5 - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2f(&self) -> MACTIMER_OVF_COMPARE2F_R {
        MACTIMER_OVF_COMPARE2F_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1f(&self) -> MACTIMER_OVF_COMPARE1F_R {
        MACTIMER_OVF_COMPARE1F_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_ovf_perf(&self) -> MACTIMER_OVF_PERF_R {
        MACTIMER_OVF_PERF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
    #[inline(always)]
    pub fn mactimer_compare2f(&self) -> MACTIMER_COMPARE2F_R {
        MACTIMER_COMPARE2F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
    #[inline(always)]
    pub fn mactimer_compare1f(&self) -> MACTIMER_COMPARE1F_R {
        MACTIMER_COMPARE1F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_perf(&self) -> MACTIMER_PERF_R {
        MACTIMER_PERF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2f(&mut self) -> MACTIMER_OVF_COMPARE2F_W {
        MACTIMER_OVF_COMPARE2F_W { w: self }
    }
    #[doc = "Bit 4 - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1f(&mut self) -> MACTIMER_OVF_COMPARE1F_W {
        MACTIMER_OVF_COMPARE1F_W { w: self }
    }
    #[doc = "Bit 3 - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_ovf_perf(&mut self) -> MACTIMER_OVF_PERF_W {
        MACTIMER_OVF_PERF_W { w: self }
    }
    #[doc = "Bit 2 - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
    #[inline(always)]
    pub fn mactimer_compare2f(&mut self) -> MACTIMER_COMPARE2F_W {
        MACTIMER_COMPARE2F_W { w: self }
    }
    #[doc = "Bit 1 - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
    #[inline(always)]
    pub fn mactimer_compare1f(&mut self) -> MACTIMER_COMPARE1F_W {
        MACTIMER_COMPARE1F_W { w: self }
    }
    #[doc = "Bit 0 - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_perf(&mut self) -> MACTIMER_PERF_W {
        MACTIMER_PERF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtirqf](index.html) module"]
pub struct MTIRQF_SPEC;
impl crate::RegisterSpec for MTIRQF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtirqf::R](R) reader structure"]
impl crate::Readable for MTIRQF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtirqf::W](W) writer structure"]
impl crate::Writable for MTIRQF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTIRQF to value 0"]
impl crate::Resettable for MTIRQF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
