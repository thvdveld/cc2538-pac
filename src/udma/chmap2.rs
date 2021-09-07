#[doc = "Register `CHMAP2` reader"]
pub struct R(crate::R<CHMAP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHMAP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHMAP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHMAP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHMAP2` writer"]
pub struct W(crate::W<CHMAP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHMAP2_SPEC>;
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
impl From<crate::W<CHMAP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHMAP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH23SEL` reader - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH23SEL_R(crate::FieldReader<u8, u8>);
impl CH23SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH23SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH23SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH23SEL` writer - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH23SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `CH22SEL` reader - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH22SEL_R(crate::FieldReader<u8, u8>);
impl CH22SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH22SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH22SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH22SEL` writer - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH22SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `CH21SEL` reader - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH21SEL_R(crate::FieldReader<u8, u8>);
impl CH21SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH21SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH21SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH21SEL` writer - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH21SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `CH20SEL` reader - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH20SEL_R(crate::FieldReader<u8, u8>);
impl CH20SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH20SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH20SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH20SEL` writer - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH20SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `CH19SEL` reader - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH19SEL_R(crate::FieldReader<u8, u8>);
impl CH19SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH19SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH19SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH19SEL` writer - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH19SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH19SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `CH18SEL` reader - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH18SEL_R(crate::FieldReader<u8, u8>);
impl CH18SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH18SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH18SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH18SEL` writer - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH18SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `CH17SEL` reader - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH17SEL_R(crate::FieldReader<u8, u8>);
impl CH17SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH17SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH17SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH17SEL` writer - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH17SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH17SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CH16SEL` reader - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH16SEL_R(crate::FieldReader<u8, u8>);
impl CH16SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CH16SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH16SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH16SEL` writer - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub struct CH16SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH16SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch23sel(&self) -> CH23SEL_R {
        CH23SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch22sel(&self) -> CH22SEL_R {
        CH22SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch21sel(&self) -> CH21SEL_R {
        CH21SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch20sel(&self) -> CH20SEL_R {
        CH20SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch19sel(&self) -> CH19SEL_R {
        CH19SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch18sel(&self) -> CH18SEL_R {
        CH18SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch17sel(&self) -> CH17SEL_R {
        CH17SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch16sel(&self) -> CH16SEL_R {
        CH16SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch23sel(&mut self) -> CH23SEL_W {
        CH23SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch22sel(&mut self) -> CH22SEL_W {
        CH22SEL_W { w: self }
    }
    #[doc = "Bits 20:23 - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch21sel(&mut self) -> CH21SEL_W {
        CH21SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch20sel(&mut self) -> CH20SEL_W {
        CH20SEL_W { w: self }
    }
    #[doc = "Bits 12:15 - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch19sel(&mut self) -> CH19SEL_W {
        CH19SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch18sel(&mut self) -> CH18SEL_W {
        CH18SEL_W { w: self }
    }
    #[doc = "Bits 4:7 - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch17sel(&mut self) -> CH17SEL_W {
        CH17SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch16sel(&mut self) -> CH16SEL_W {
        CH16SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chmap2](index.html) module"]
pub struct CHMAP2_SPEC;
impl crate::RegisterSpec for CHMAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chmap2::R](R) reader structure"]
impl crate::Readable for CHMAP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chmap2::W](W) writer structure"]
impl crate::Writable for CHMAP2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHMAP2 to value 0"]
impl crate::Resettable for CHMAP2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
