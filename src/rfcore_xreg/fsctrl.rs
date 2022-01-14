#[doc = "Register `FSCTRL` reader"]
pub struct R(crate::R<FSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSCTRL` writer"]
pub struct W(crate::W<FSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSCTRL_SPEC>;
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
impl From<crate::W<FSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_CURRENT` reader - Prescaler current setting"]
pub struct PRE_CURRENT_R(crate::FieldReader<u8, u8>);
impl PRE_CURRENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRE_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_CURRENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRE_CURRENT` writer - Prescaler current setting"]
pub struct PRE_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `LODIV_BUF_CURRENT_TX` reader - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
pub struct LODIV_BUF_CURRENT_TX_R(crate::FieldReader<u8, u8>);
impl LODIV_BUF_CURRENT_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LODIV_BUF_CURRENT_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIV_BUF_CURRENT_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LODIV_BUF_CURRENT_TX` writer - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
pub struct LODIV_BUF_CURRENT_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_BUF_CURRENT_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `LODIV_BUF_CURRENT_RX` reader - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
pub struct LODIV_BUF_CURRENT_RX_R(crate::FieldReader<u8, u8>);
impl LODIV_BUF_CURRENT_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LODIV_BUF_CURRENT_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIV_BUF_CURRENT_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LODIV_BUF_CURRENT_RX` writer - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
pub struct LODIV_BUF_CURRENT_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_BUF_CURRENT_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `LODIV_CURRENT` reader - Adjusts divider currents, except mixer and PA buffers"]
pub struct LODIV_CURRENT_R(crate::FieldReader<u8, u8>);
impl LODIV_CURRENT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        LODIV_CURRENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LODIV_CURRENT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LODIV_CURRENT` writer - Adjusts divider currents, except mixer and PA buffers"]
pub struct LODIV_CURRENT_W<'a> {
    w: &'a mut W,
}
impl<'a> LODIV_CURRENT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - Prescaler current setting"]
    #[inline(always)]
    pub fn pre_current(&self) -> PRE_CURRENT_R {
        PRE_CURRENT_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
    #[inline(always)]
    pub fn lodiv_buf_current_tx(&self) -> LODIV_BUF_CURRENT_TX_R {
        LODIV_BUF_CURRENT_TX_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
    #[inline(always)]
    pub fn lodiv_buf_current_rx(&self) -> LODIV_BUF_CURRENT_RX_R {
        LODIV_BUF_CURRENT_RX_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - Adjusts divider currents, except mixer and PA buffers"]
    #[inline(always)]
    pub fn lodiv_current(&self) -> LODIV_CURRENT_R {
        LODIV_CURRENT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Prescaler current setting"]
    #[inline(always)]
    pub fn pre_current(&mut self) -> PRE_CURRENT_W {
        PRE_CURRENT_W { w: self }
    }
    #[doc = "Bits 4:5 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 1"]
    #[inline(always)]
    pub fn lodiv_buf_current_tx(&mut self) -> LODIV_BUF_CURRENT_TX_W {
        LODIV_BUF_CURRENT_TX_W { w: self }
    }
    #[doc = "Bits 2:3 - Adjusts current in mixer and PA buffers Used when TX_ACTIVE = 0"]
    #[inline(always)]
    pub fn lodiv_buf_current_rx(&mut self) -> LODIV_BUF_CURRENT_RX_W {
        LODIV_BUF_CURRENT_RX_W { w: self }
    }
    #[doc = "Bits 0:1 - Adjusts divider currents, except mixer and PA buffers"]
    #[inline(always)]
    pub fn lodiv_current(&mut self) -> LODIV_CURRENT_W {
        LODIV_CURRENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tune frequency synthesizer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsctrl](index.html) module"]
pub struct FSCTRL_SPEC;
impl crate::RegisterSpec for FSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsctrl::R](R) reader structure"]
impl crate::Readable for FSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsctrl::W](W) writer structure"]
impl crate::Writable for FSCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FSCTRL to value 0"]
impl crate::Resettable for FSCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
