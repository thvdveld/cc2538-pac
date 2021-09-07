#[doc = "Register `CHMAP1` reader"]
pub struct R(crate::R<CHMAP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMAP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMAP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMAP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMAP1` writer"]
pub struct W(crate::W<CHMAP1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMAP1_SPEC>;
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
impl From<crate::W<CHMAP1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMAP1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH15SEL` reader - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH15SEL_R(crate::FieldReader<u8, u8>);
impl CH15SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH15SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH15SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH15SEL` writer - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH15SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `CH14SEL` reader - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH14SEL_R(crate::FieldReader<u8, u8>);
impl CH14SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH14SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH14SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH14SEL` writer - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH14SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CH13SEL` reader - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH13SEL_R(crate::FieldReader<u8, u8>);
impl CH13SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH13SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH13SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH13SEL` writer - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH13SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `CH12SEL` reader - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH12SEL_R(crate::FieldReader<u8, u8>);
impl CH12SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH12SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH12SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH12SEL` writer - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH12SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CH11SEL` reader - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH11SEL_R(crate::FieldReader<u8, u8>);
impl CH11SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH11SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH11SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH11SEL` writer - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH11SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `CH10SEL` reader - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH10SEL_R(crate::FieldReader<u8, u8>);
impl CH10SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH10SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH10SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH10SEL` writer - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH10SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CH9SEL` reader - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH9SEL_R(crate::FieldReader<u8, u8>);
impl CH9SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH9SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH9SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH9SEL` writer - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH9SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CH8SEL` reader - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH8SEL_R(crate::FieldReader<u8, u8>);
impl CH8SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH8SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH8SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH8SEL` writer - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH8SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch15sel(&self) -> CH15SEL_R {
        CH15SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch14sel(&self) -> CH14SEL_R {
        CH14SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch13sel(&self) -> CH13SEL_R {
        CH13SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch12sel(&self) -> CH12SEL_R {
        CH12SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch11sel(&self) -> CH11SEL_R {
        CH11SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch10sel(&self) -> CH10SEL_R {
        CH10SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch9sel(&self) -> CH9SEL_R {
        CH9SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch8sel(&self) -> CH8SEL_R {
        CH8SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch15sel(&mut self) -> CH15SEL_W {
        CH15SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch14sel(&mut self) -> CH14SEL_W {
        CH14SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch13sel(&mut self) -> CH13SEL_W {
        CH13SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch12sel(&mut self) -> CH12SEL_W {
        CH12SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch11sel(&mut self) -> CH11SEL_W {
        CH11SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch10sel(&mut self) -> CH10SEL_W {
        CH10SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch9sel(&mut self) -> CH9SEL_W {
        CH9SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch8sel(&mut self) -> CH8SEL_W {
        CH8SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap1](index.html) module"]
pub struct CHMAP1_SPEC;
impl crate::RegisterSpec for CHMAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmap1::R](R) reader structure"]
impl crate::Readable for CHMAP1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmap1::W](W) writer structure"]
impl crate::Writable for CHMAP1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHMAP1 to value 0"]
impl crate::Resettable for CHMAP1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
