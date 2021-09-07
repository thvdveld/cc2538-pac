#[doc = "Register `IWE` reader"]
pub struct R(crate::R<IWE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IWE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IWE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IWE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IWE` writer"]
pub struct W(crate::W<IWE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IWE_SPEC>;
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
impl From<crate::W<IWE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IWE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM_TIMER_IWE` reader - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
pub struct SM_TIMER_IWE_R(crate::FieldReader<bool, bool>);
impl SM_TIMER_IWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SM_TIMER_IWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM_TIMER_IWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM_TIMER_IWE` writer - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
pub struct SM_TIMER_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_TIMER_IWE_W<'a> {
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
#[doc = "Field `USB_IWE` reader - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
pub struct USB_IWE_R(crate::FieldReader<bool, bool>);
impl USB_IWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_IWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_IWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_IWE` writer - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
pub struct USB_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_IWE_W<'a> {
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
#[doc = "Field `PORT_D_IWE` reader - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
pub struct PORT_D_IWE_R(crate::FieldReader<bool, bool>);
impl PORT_D_IWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORT_D_IWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_D_IWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_D_IWE` writer - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
pub struct PORT_D_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_D_IWE_W<'a> {
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
#[doc = "Field `PORT_C_IWE` reader - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
pub struct PORT_C_IWE_R(crate::FieldReader<bool, bool>);
impl PORT_C_IWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORT_C_IWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_C_IWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_C_IWE` writer - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
pub struct PORT_C_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_C_IWE_W<'a> {
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
#[doc = "Field `PORT_B_IWE` reader - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
pub struct PORT_B_IWE_R(crate::FieldReader<bool, bool>);
impl PORT_B_IWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORT_B_IWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_B_IWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_B_IWE` writer - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
pub struct PORT_B_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_B_IWE_W<'a> {
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
#[doc = "Field `PORT_A_IWE` reader - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
pub struct PORT_A_IWE_R(crate::FieldReader<bool, bool>);
impl PORT_A_IWE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORT_A_IWE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORT_A_IWE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORT_A_IWE` writer - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
pub struct PORT_A_IWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_A_IWE_W<'a> {
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
    #[doc = "Bit 5 - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
    #[inline(always)]
    pub fn sm_timer_iwe(&self) -> SM_TIMER_IWE_R {
        SM_TIMER_IWE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
    #[inline(always)]
    pub fn usb_iwe(&self) -> USB_IWE_R {
        USB_IWE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
    #[inline(always)]
    pub fn port_d_iwe(&self) -> PORT_D_IWE_R {
        PORT_D_IWE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
    #[inline(always)]
    pub fn port_c_iwe(&self) -> PORT_C_IWE_R {
        PORT_C_IWE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
    #[inline(always)]
    pub fn port_b_iwe(&self) -> PORT_B_IWE_R {
        PORT_B_IWE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
    #[inline(always)]
    pub fn port_a_iwe(&self) -> PORT_A_IWE_R {
        PORT_A_IWE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - 1: Enable SM Timer wake-up interrupt. 0: Disable SM Timer wake-up interrupt."]
    #[inline(always)]
    pub fn sm_timer_iwe(&mut self) -> SM_TIMER_IWE_W {
        SM_TIMER_IWE_W { w: self }
    }
    #[doc = "Bit 4 - 1: Enable USB wake-up interrupt. 0: Disable USB wake-up interrupt."]
    #[inline(always)]
    pub fn usb_iwe(&mut self) -> USB_IWE_W {
        USB_IWE_W { w: self }
    }
    #[doc = "Bit 3 - 1: Enable port D wake-up interrupt. 0: Disable port D wake-up interrupt."]
    #[inline(always)]
    pub fn port_d_iwe(&mut self) -> PORT_D_IWE_W {
        PORT_D_IWE_W { w: self }
    }
    #[doc = "Bit 2 - 1: Enable port C wake-up interrupt. 0: Disable port C wake-up interrupt."]
    #[inline(always)]
    pub fn port_c_iwe(&mut self) -> PORT_C_IWE_W {
        PORT_C_IWE_W { w: self }
    }
    #[doc = "Bit 1 - 1: Enable port B wake-up interrupt. 0: Disable port B wake-up interrupt."]
    #[inline(always)]
    pub fn port_b_iwe(&mut self) -> PORT_B_IWE_W {
        PORT_B_IWE_W { w: self }
    }
    #[doc = "Bit 0 - 1: Enable port A wake-up interrupt. 0: Disable port A wake-up interrupt."]
    #[inline(always)]
    pub fn port_a_iwe(&mut self) -> PORT_A_IWE_W {
        PORT_A_IWE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register controls interrupt wake-up.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iwe](index.html) module"]
pub struct IWE_SPEC;
impl crate::RegisterSpec for IWE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iwe::R](R) reader structure"]
impl crate::Readable for IWE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iwe::W](W) writer structure"]
impl crate::Writable for IWE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IWE to value 0"]
impl crate::Resettable for IWE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
