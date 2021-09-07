#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LME5IM` reader - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct LME5IM_R(crate::FieldReader<bool, bool>);
impl LME5IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LME5IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LME5IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LME5IM` writer - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct LME5IM_W<'a> {
    w: &'a mut W,
}
impl<'a> LME5IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `LME1IM` reader - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct LME1IM_R(crate::FieldReader<bool, bool>);
impl LME1IM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LME1IM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LME1IM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LME1IM` writer - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct LME1IM_W<'a> {
    w: &'a mut W,
}
impl<'a> LME1IM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `LMSBIM` reader - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct LMSBIM_R(crate::FieldReader<bool, bool>);
impl LMSBIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        LMSBIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LMSBIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LMSBIM` writer - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct LMSBIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LMSBIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `NINEBITIM` reader - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct NINEBITIM_R(crate::FieldReader<bool, bool>);
impl NINEBITIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NINEBITIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NINEBITIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NINEBITIM` writer - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct NINEBITIM_W<'a> {
    w: &'a mut W,
}
impl<'a> NINEBITIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `OEIM` reader - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct OEIM_R(crate::FieldReader<bool, bool>);
impl OEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEIM` writer - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIM_W<'a> {
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
#[doc = "Field `BEIM` reader - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct BEIM_R(crate::FieldReader<bool, bool>);
impl BEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEIM` writer - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIM_W<'a> {
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
#[doc = "Field `PEIM` reader - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct PEIM_R(crate::FieldReader<bool, bool>);
impl PEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEIM` writer - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIM_W<'a> {
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
#[doc = "Field `FEIM` reader - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct FEIM_R(crate::FieldReader<bool, bool>);
impl FEIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIM` writer - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RTIM` reader - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct RTIM_R(crate::FieldReader<bool, bool>);
impl RTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIM` writer - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
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
#[doc = "Field `TXIM` reader - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct TXIM_R(crate::FieldReader<bool, bool>);
impl TXIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIM` writer - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
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
#[doc = "Field `RXIM` reader - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct RXIM_R(crate::FieldReader<bool, bool>);
impl RXIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIM` writer - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
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
impl R {
    #[doc = "Bit 15 - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme5im(&self) -> LME5IM_R {
        LME5IM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme1im(&self) -> LME1IM_R {
        LME1IM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lmsbim(&self) -> LMSBIM_R {
        LMSBIM_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn ninebitim(&self) -> NINEBITIM_R {
        NINEBITIM_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10 - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme5im(&mut self) -> LME5IM_W {
        LME5IM_W { w: self }
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme1im(&mut self) -> LME1IM_W {
        LME1IM_W { w: self }
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lmsbim(&mut self) -> LMSBIM_W {
        LMSBIM_W { w: self }
    }
    #[doc = "Bit 12 - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn ninebitim(&mut self) -> NINEBITIM_W {
        NINEBITIM_W { w: self }
    }
    #[doc = "Bit 10 - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W {
        OEIM_W { w: self }
    }
    #[doc = "Bit 9 - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W {
        BEIM_W { w: self }
    }
    #[doc = "Bit 8 - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W {
        PEIM_W { w: self }
    }
    #[doc = "Bit 7 - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W {
        FEIM_W { w: self }
    }
    #[doc = "Bit 6 - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 5 - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
    }
    #[doc = "Bit 4 - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt mask The IM register is the interrupt mask set/clear register. On a read, this register gives the current value of the mask on the relevant interrupt. Setting a bit allows the corresponding raw interrupt signal to be routed to the interrupt controller. Clearing a bit prevents the raw interrupt signal from being sent to the interrupt controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
