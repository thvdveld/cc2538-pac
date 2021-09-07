#[doc = "Register `RFERRF` reader"]
pub struct R(crate::R<RFERRF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFERRF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFERRF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFERRF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFERRF` writer"]
pub struct W(crate::W<RFERRF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFERRF_SPEC>;
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
impl From<crate::W<RFERRF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFERRF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STROBEERR` reader - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
pub struct STROBEERR_R(crate::FieldReader<bool, bool>);
impl STROBEERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        STROBEERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STROBEERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STROBEERR` writer - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
pub struct STROBEERR_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBEERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `TXUNDERF` reader - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub struct TXUNDERF_R(crate::FieldReader<bool, bool>);
impl TXUNDERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUNDERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXUNDERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXUNDERF` writer - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub struct TXUNDERF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNDERF_W<'a> {
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
#[doc = "Field `TXOVERF` reader - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub struct TXOVERF_R(crate::FieldReader<bool, bool>);
impl TXOVERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOVERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOVERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOVERF` writer - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub struct TXOVERF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOVERF_W<'a> {
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
#[doc = "Field `RXUNDERF` reader - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXUNDERF_R(crate::FieldReader<bool, bool>);
impl RXUNDERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXUNDERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUNDERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUNDERF` writer - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXUNDERF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUNDERF_W<'a> {
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
#[doc = "Field `RXOVERF` reader - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXOVERF_R(crate::FieldReader<bool, bool>);
impl RXOVERF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOVERF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOVERF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOVERF` writer - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXOVERF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERF_W<'a> {
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
#[doc = "Field `RXABO` reader - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXABO_R(crate::FieldReader<bool, bool>);
impl RXABO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXABO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXABO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXABO` writer - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
pub struct RXABO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXABO_W<'a> {
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
#[doc = "Field `NLOCK` reader - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
pub struct NLOCK_R(crate::FieldReader<bool, bool>);
impl NLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NLOCK` writer - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
pub struct NLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> NLOCK_W<'a> {
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
    #[doc = "Bit 6 - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn strobeerr(&self) -> STROBEERR_R {
        STROBEERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txunderf(&self) -> TXUNDERF_R {
        TXUNDERF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txoverf(&self) -> TXOVERF_R {
        TXOVERF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxunderf(&self) -> RXUNDERF_R {
        RXUNDERF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxoverf(&self) -> RXOVERF_R {
        RXOVERF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxabo(&self) -> RXABO_R {
        RXABO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn nlock(&self) -> NLOCK_R {
        NLOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - A command strobe was issued when it could not be processed. Triggered if trying to disable the radio when it is already disabled, or when trying to do a SACK, SACKPEND, or SNACK command when not in active RX. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn strobeerr(&mut self) -> STROBEERR_W {
        STROBEERR_W { w: self }
    }
    #[doc = "Bit 5 - TX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txunderf(&mut self) -> TXUNDERF_W {
        TXUNDERF_W { w: self }
    }
    #[doc = "Bit 4 - TX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn txoverf(&mut self) -> TXOVERF_W {
        TXOVERF_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO underflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxunderf(&mut self) -> RXUNDERF_W {
        RXUNDERF_W { w: self }
    }
    #[doc = "Bit 2 - RX FIFO overflowed. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxoverf(&mut self) -> RXOVERF_W {
        RXOVERF_W { w: self }
    }
    #[doc = "Bit 1 - Reception of a frame was aborted. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn rxabo(&mut self) -> RXABO_W {
        RXABO_W { w: self }
    }
    #[doc = "Bit 0 - The frequency synthesizer failed to achieve lock after time-out, or lock is lost during reception. The receiver must be restarted to clear this error situation. 0: No interrupt pending 1: Interrupt pending"]
    #[inline(always)]
    pub fn nlock(&mut self) -> NLOCK_W {
        NLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF error interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rferrf](index.html) module"]
pub struct RFERRF_SPEC;
impl crate::RegisterSpec for RFERRF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rferrf::R](R) reader structure"]
impl crate::Readable for RFERRF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rferrf::W](W) writer structure"]
impl crate::Writable for RFERRF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RFERRF to value 0"]
impl crate::Resettable for RFERRF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
