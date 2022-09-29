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
#[doc = "Field `CH24SEL` reader - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH24SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH24SEL` writer - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH24SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH25SEL` reader - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH25SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH25SEL` writer - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH25SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH26SEL` reader - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH26SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH26SEL` writer - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH26SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH27SEL` reader - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH27SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH27SEL` writer - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH27SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH28SEL` reader - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH28SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH28SEL` writer - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH28SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH29SEL` reader - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH29SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH29SEL` writer - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH29SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH30SEL` reader - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH30SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH30SEL` writer - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH30SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH31SEL` reader - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH31SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH31SEL` writer - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH31SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP3_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch24sel(&self) -> CH24SEL_R {
        CH24SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch25sel(&self) -> CH25SEL_R {
        CH25SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch26sel(&self) -> CH26SEL_R {
        CH26SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch27sel(&self) -> CH27SEL_R {
        CH27SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch28sel(&self) -> CH28SEL_R {
        CH28SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch29sel(&self) -> CH29SEL_R {
        CH29SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch30sel(&self) -> CH30SEL_R {
        CH30SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch31sel(&self) -> CH31SEL_R {
        CH31SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 24 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch24sel(&mut self) -> CH24SEL_W<0> {
        CH24SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA channel 25 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch25sel(&mut self) -> CH25SEL_W<4> {
        CH25SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA channel 26 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch26sel(&mut self) -> CH26SEL_W<8> {
        CH26SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA channel 27 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch27sel(&mut self) -> CH27SEL_W<12> {
        CH27SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA channel 28 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch28sel(&mut self) -> CH28SEL_W<16> {
        CH28SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA channel 29 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch29sel(&mut self) -> CH29SEL_W<20> {
        CH29SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA channel 30 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch30sel(&mut self) -> CH30SEL_W<24> {
        CH30SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA channel 31 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch31sel(&mut self) -> CH31SEL_W<28> {
        CH31SEL_W::new(self)
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
