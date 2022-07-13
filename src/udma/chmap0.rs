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
pub type CH7SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH7SEL` writer - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH7SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH6SEL` reader - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH6SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH6SEL` writer - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH6SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH5SEL` reader - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH5SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH5SEL` writer - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH5SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH4SEL` reader - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH4SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH4SEL` writer - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH4SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH3SEL` reader - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH3SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3SEL` writer - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH3SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH2SEL` reader - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH2SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2SEL` writer - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH2SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH1SEL` reader - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1SEL` writer - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH0SEL` reader - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH0SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0SEL` writer - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH0SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHMAP0_SPEC, u8, u8, 4, O>;
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
    pub fn ch7sel(&mut self) -> CH7SEL_W<28> {
        CH7SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch6sel(&mut self) -> CH6SEL_W<24> {
        CH6SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch5sel(&mut self) -> CH5SEL_W<20> {
        CH5SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch4sel(&mut self) -> CH4SEL_W<16> {
        CH4SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch3sel(&mut self) -> CH3SEL_W<12> {
        CH3SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch2sel(&mut self) -> CH2SEL_W<8> {
        CH2SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch1sel(&mut self) -> CH1SEL_W<4> {
        CH1SEL_W::new(self)
    }
    #[doc = "Bits 0:3 - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch0sel(&mut self) -> CH0SEL_W<0> {
        CH0SEL_W::new(self)
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
