#[doc = "Register `RFIRQF1` reader"]
pub struct R(crate::R<RFIRQF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIRQF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFIRQF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFIRQF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFIRQF1` writer"]
pub struct W(crate::W<RFIRQF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIRQF1_SPEC>;
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
impl From<crate::W<RFIRQF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFIRQF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSP_WAIT` reader - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
pub struct CSP_WAIT_R(crate::FieldReader<bool, bool>);
impl CSP_WAIT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSP_WAIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSP_WAIT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSP_WAIT` writer - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
pub struct CSP_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSP_WAIT_W<'a> {
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
#[doc = "Field `CSP_STOP` reader - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
pub struct CSP_STOP_R(crate::FieldReader<bool, bool>);
impl CSP_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSP_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSP_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSP_STOP` writer - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
pub struct CSP_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSP_STOP_W<'a> {
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
#[doc = "Field `CSP_MANINT` reader - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
pub struct CSP_MANINT_R(crate::FieldReader<bool, bool>);
impl CSP_MANINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSP_MANINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSP_MANINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSP_MANINT` writer - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
pub struct CSP_MANINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CSP_MANINT_W<'a> {
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
#[doc = "Field `RFIDLE` reader - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
pub struct RFIDLE_R(crate::FieldReader<bool, bool>);
impl RFIDLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFIDLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFIDLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFIDLE` writer - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
pub struct RFIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFIDLE_W<'a> {
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
#[doc = "Field `TXDONE` reader - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub struct TXDONE_R(crate::FieldReader<bool, bool>);
impl TXDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDONE` writer - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub struct TXDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDONE_W<'a> {
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
#[doc = "Field `TXACKDONE` reader - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub struct TXACKDONE_R(crate::FieldReader<bool, bool>);
impl TXACKDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXACKDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXACKDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXACKDONE` writer - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
pub struct TXACKDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXACKDONE_W<'a> {
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
    #[doc = "Bit 5 - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_wait(&self) -> CSP_WAIT_R {
        CSP_WAIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_stop(&self) -> CSP_STOP_R {
        CSP_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_manint(&self) -> CSP_MANINT_R {
        CSP_MANINT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rfidle(&self) -> RFIDLE_R {
        RFIDLE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txdone(&self) -> TXDONE_R {
        TXDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txackdone(&self) -> TXACKDONE_R {
        TXACKDONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Execution continued after a wait instruction in CSP. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_wait(&mut self) -> CSP_WAIT_W {
        CSP_WAIT_W { w: self }
    }
    #[doc = "Bit 4 - CSP has stopped program execution. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_stop(&mut self) -> CSP_STOP_W {
        CSP_STOP_W { w: self }
    }
    #[doc = "Bit 3 - Manual interrupt generated from CSP 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn csp_manint(&mut self) -> CSP_MANINT_W {
        CSP_MANINT_W { w: self }
    }
    #[doc = "Bit 2 - Radio state-machine has entered the IDLE state. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rfidle(&mut self) -> RFIDLE_W {
        RFIDLE_W { w: self }
    }
    #[doc = "Bit 1 - A complete frame has been transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txdone(&mut self) -> TXDONE_W {
        TXDONE_W { w: self }
    }
    #[doc = "Bit 0 - An acknowledgement frame has been completely transmitted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txackdone(&mut self) -> TXACKDONE_W {
        TXACKDONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfirqf1](index.html) module"]
pub struct RFIRQF1_SPEC;
impl crate::RegisterSpec for RFIRQF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfirqf1::R](R) reader structure"]
impl crate::Readable for RFIRQF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfirqf1::W](W) writer structure"]
impl crate::Writable for RFIRQF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFIRQF1 to value 0"]
impl crate::Resettable for RFIRQF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
