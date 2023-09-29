#[doc = "Register `CHMAP1` reader"]
pub type R = crate::R<CHMAP1_SPEC>;
#[doc = "Register `CHMAP1` writer"]
pub type W = crate::W<CHMAP1_SPEC>;
#[doc = "Field `CH8SEL` reader - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH8SEL_R = crate::FieldReader;
#[doc = "Field `CH8SEL` writer - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH8SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH9SEL` reader - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH9SEL_R = crate::FieldReader;
#[doc = "Field `CH9SEL` writer - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH9SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH10SEL` reader - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH10SEL_R = crate::FieldReader;
#[doc = "Field `CH10SEL` writer - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH10SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH11SEL` reader - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH11SEL_R = crate::FieldReader;
#[doc = "Field `CH11SEL` writer - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH11SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH12SEL` reader - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH12SEL_R = crate::FieldReader;
#[doc = "Field `CH12SEL` writer - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH12SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH13SEL` reader - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH13SEL_R = crate::FieldReader;
#[doc = "Field `CH13SEL` writer - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH13SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH14SEL` reader - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH14SEL_R = crate::FieldReader;
#[doc = "Field `CH14SEL` writer - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH14SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH15SEL` reader - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH15SEL_R = crate::FieldReader;
#[doc = "Field `CH15SEL` writer - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH15SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch8sel(&self) -> CH8SEL_R {
        CH8SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch9sel(&self) -> CH9SEL_R {
        CH9SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch10sel(&self) -> CH10SEL_R {
        CH10SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch11sel(&self) -> CH11SEL_R {
        CH11SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch12sel(&self) -> CH12SEL_R {
        CH12SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch13sel(&self) -> CH13SEL_R {
        CH13SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch14sel(&self) -> CH14SEL_R {
        CH14SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch15sel(&self) -> CH15SEL_R {
        CH15SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 8 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch8sel(&mut self) -> CH8SEL_W<CHMAP1_SPEC, 0> {
        CH8SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA channel 9 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch9sel(&mut self) -> CH9SEL_W<CHMAP1_SPEC, 4> {
        CH9SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA channel 10 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch10sel(&mut self) -> CH10SEL_W<CHMAP1_SPEC, 8> {
        CH10SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA channel 11 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch11sel(&mut self) -> CH11SEL_W<CHMAP1_SPEC, 12> {
        CH11SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA channel 12 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch12sel(&mut self) -> CH12SEL_W<CHMAP1_SPEC, 16> {
        CH12SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA channel 13 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch13sel(&mut self) -> CH13SEL_W<CHMAP1_SPEC, 20> {
        CH13SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA channel 14 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch14sel(&mut self) -> CH14SEL_W<CHMAP1_SPEC, 24> {
        CH14SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA channel 15 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch15sel(&mut self) -> CH15SEL_W<CHMAP1_SPEC, 28> {
        CH15SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel map select 1 Each 4-bit field of the CHMAP1 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHMAP1_SPEC;
impl crate::RegisterSpec for CHMAP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap1::R`](R) reader structure"]
impl crate::Readable for CHMAP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chmap1::W`](W) writer structure"]
impl crate::Writable for CHMAP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHMAP1 to value 0"]
impl crate::Resettable for CHMAP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
