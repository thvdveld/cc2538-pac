#[doc = "Register `CHMAP3` reader"]
pub struct R(crate::R<CHMAP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMAP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMAP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMAP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMAP3` writer"]
pub struct W(crate::W<CHMAP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMAP3_SPEC>;
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
impl From<crate::W<CHMAP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMAP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH31SEL` reader - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH31SEL_R(crate::FieldReader<u8, u8>);
impl CH31SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH31SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH31SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH31SEL` writer - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH31SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `CH30SEL` reader - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH30SEL_R(crate::FieldReader<u8, u8>);
impl CH30SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH30SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH30SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH30SEL` writer - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH30SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH30SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CH29SEL` reader - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH29SEL_R(crate::FieldReader<u8, u8>);
impl CH29SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH29SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH29SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH29SEL` writer - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH29SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH29SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `CH28SEL` reader - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH28SEL_R(crate::FieldReader<u8, u8>);
impl CH28SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH28SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH28SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH28SEL` writer - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH28SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH28SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CH27SEL` reader - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH27SEL_R(crate::FieldReader<u8, u8>);
impl CH27SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH27SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH27SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH27SEL` writer - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH27SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH27SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `CH26SEL` reader - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH26SEL_R(crate::FieldReader<u8, u8>);
impl CH26SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH26SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH26SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH26SEL` writer - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH26SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH26SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CH25SEL` reader - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH25SEL_R(crate::FieldReader<u8, u8>);
impl CH25SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH25SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH25SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH25SEL` writer - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH25SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH25SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CH24SEL` reader - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH24SEL_R(crate::FieldReader<u8, u8>);
impl CH24SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH24SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH24SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH24SEL` writer - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH24SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH24SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch31sel(&self) -> CH31SEL_R {
        CH31SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch30sel(&self) -> CH30SEL_R {
        CH30SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch29sel(&self) -> CH29SEL_R {
        CH29SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch28sel(&self) -> CH28SEL_R {
        CH28SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch27sel(&self) -> CH27SEL_R {
        CH27SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch26sel(&self) -> CH26SEL_R {
        CH26SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch25sel(&self) -> CH25SEL_R {
        CH25SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch24sel(&self) -> CH24SEL_R {
        CH24SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch31sel(&mut self) -> CH31SEL_W {
        CH31SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch30sel(&mut self) -> CH30SEL_W {
        CH30SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch29sel(&mut self) -> CH29SEL_W {
        CH29SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch28sel(&mut self) -> CH28SEL_W {
        CH28SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch27sel(&mut self) -> CH27SEL_W {
        CH27SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch26sel(&mut self) -> CH26SEL_W {
        CH26SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch25sel(&mut self) -> CH25SEL_W {
        CH25SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch24sel(&mut self) -> CH24SEL_W {
        CH24SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel map select 3 Each 4-bit field of the CHMAP3 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap3](index.html) module"]
pub struct CHMAP3_SPEC;
impl crate::RegisterSpec for CHMAP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmap3::R](R) reader structure"]
impl crate::Readable for CHMAP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmap3::W](W) writer structure"]
impl crate::Writable for CHMAP3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHMAP3 to value 0"]
impl crate::Resettable for CHMAP3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
