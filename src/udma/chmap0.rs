#[doc = "Register `CHMAP0` reader"]
pub struct R(crate::R<CHMAP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMAP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMAP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMAP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMAP0` writer"]
pub struct W(crate::W<CHMAP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMAP0_SPEC>;
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
impl From<crate::W<CHMAP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMAP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH7SEL` reader - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH7SEL_R(crate::FieldReader<u8, u8>);
impl CH7SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH7SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7SEL` writer - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `CH6SEL` reader - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH6SEL_R(crate::FieldReader<u8, u8>);
impl CH6SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH6SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6SEL` writer - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CH5SEL` reader - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH5SEL_R(crate::FieldReader<u8, u8>);
impl CH5SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH5SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5SEL` writer - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `CH4SEL` reader - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH4SEL_R(crate::FieldReader<u8, u8>);
impl CH4SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH4SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4SEL` writer - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CH3SEL` reader - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH3SEL_R(crate::FieldReader<u8, u8>);
impl CH3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3SEL` writer - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `CH2SEL` reader - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH2SEL_R(crate::FieldReader<u8, u8>);
impl CH2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2SEL` writer - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CH1SEL` reader - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH1SEL_R(crate::FieldReader<u8, u8>);
impl CH1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1SEL` writer - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CH0SEL` reader - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH0SEL_R(crate::FieldReader<u8, u8>);
impl CH0SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH0SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0SEL` writer - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH0SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch7sel(&self) -> CH7SEL_R {
        CH7SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch6sel(&self) -> CH6SEL_R {
        CH6SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch5sel(&self) -> CH5SEL_R {
        CH5SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch4sel(&self) -> CH4SEL_R {
        CH4SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch3sel(&self) -> CH3SEL_R {
        CH3SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch2sel(&self) -> CH2SEL_R {
        CH2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch1sel(&self) -> CH1SEL_R {
        CH1SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch0sel(&self) -> CH0SEL_R {
        CH0SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch7sel(&mut self) -> CH7SEL_W {
        CH7SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch6sel(&mut self) -> CH6SEL_W {
        CH6SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch5sel(&mut self) -> CH5SEL_W {
        CH5SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch4sel(&mut self) -> CH4SEL_W {
        CH4SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch3sel(&mut self) -> CH3SEL_W {
        CH3SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch2sel(&mut self) -> CH2SEL_W {
        CH2SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch1sel(&mut self) -> CH1SEL_W {
        CH1SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch0sel(&mut self) -> CH0SEL_W {
        CH0SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap0](index.html) module"]
pub struct CHMAP0_SPEC;
impl crate::RegisterSpec for CHMAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmap0::R](R) reader structure"]
impl crate::Readable for CHMAP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmap0::W](W) writer structure"]
impl crate::Writable for CHMAP0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHMAP0 to value 0"]
impl crate::Resettable for CHMAP0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
