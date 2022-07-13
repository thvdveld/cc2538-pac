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
pub type CH15SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH15SEL` writer - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH15SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH14SEL` reader - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH14SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH14SEL` writer - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH14SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH13SEL` reader - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH13SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH13SEL` writer - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH13SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH12SEL` reader - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH12SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH12SEL` writer - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH12SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH11SEL` reader - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH11SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH11SEL` writer - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH11SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH10SEL` reader - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH10SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH10SEL` writer - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH10SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH9SEL` reader - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH9SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH9SEL` writer - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH9SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH8SEL` reader - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH8SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH8SEL` writer - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH8SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP1_SPEC, u8, u8, 4, O>;
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
    pub fn ch15sel(&mut self) -> CH15SEL_W<28> {
        CH15SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch14sel(&mut self) -> CH14SEL_W<24> {
        CH14SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch13sel(&mut self) -> CH13SEL_W<20> {
        CH13SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch12sel(&mut self) -> CH12SEL_W<16> {
        CH12SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch11sel(&mut self) -> CH11SEL_W<12> {
        CH11SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch10sel(&mut self) -> CH10SEL_W<8> {
        CH10SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch9sel(&mut self) -> CH9SEL_W<4> {
        CH9SEL_W::new(self)
    }
    #[doc = "Bits 0:3 - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch8sel(&mut self) -> CH8SEL_W<0> {
        CH8SEL_W::new(self)
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
