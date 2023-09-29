#[doc = "Register `CHMAP0` reader"]
pub type R = crate::R<CHMAP0_SPEC>;
#[doc = "Register `CHMAP0` writer"]
pub type W = crate::W<CHMAP0_SPEC>;
#[doc = "Field `CH0SEL` reader - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH0SEL_R = crate::FieldReader;
#[doc = "Field `CH0SEL` writer - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH0SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH1SEL` reader - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH1SEL_R = crate::FieldReader;
#[doc = "Field `CH1SEL` writer - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH1SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH2SEL` reader - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH2SEL_R = crate::FieldReader;
#[doc = "Field `CH2SEL` writer - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH2SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH3SEL` reader - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH3SEL_R = crate::FieldReader;
#[doc = "Field `CH3SEL` writer - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH3SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH4SEL` reader - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH4SEL_R = crate::FieldReader;
#[doc = "Field `CH4SEL` writer - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH4SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH5SEL` reader - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH5SEL_R = crate::FieldReader;
#[doc = "Field `CH5SEL` writer - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH5SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH6SEL` reader - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH6SEL_R = crate::FieldReader;
#[doc = "Field `CH6SEL` writer - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH6SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH7SEL` reader - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH7SEL_R = crate::FieldReader;
#[doc = "Field `CH7SEL` writer - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH7SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch0sel(&self) -> CH0SEL_R {
        CH0SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch1sel(&self) -> CH1SEL_R {
        CH1SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch2sel(&self) -> CH2SEL_R {
        CH2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch3sel(&self) -> CH3SEL_R {
        CH3SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch4sel(&self) -> CH4SEL_R {
        CH4SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch5sel(&self) -> CH5SEL_R {
        CH5SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch6sel(&self) -> CH6SEL_R {
        CH6SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch7sel(&self) -> CH7SEL_R {
        CH7SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 0 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch0sel(&mut self) -> CH0SEL_W<CHMAP0_SPEC, 0> {
        CH0SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA channel 1 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch1sel(&mut self) -> CH1SEL_W<CHMAP0_SPEC, 4> {
        CH1SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA channel 2 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch2sel(&mut self) -> CH2SEL_W<CHMAP0_SPEC, 8> {
        CH2SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA channel 3 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch3sel(&mut self) -> CH3SEL_W<CHMAP0_SPEC, 12> {
        CH3SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA channel 4 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch4sel(&mut self) -> CH4SEL_W<CHMAP0_SPEC, 16> {
        CH4SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA channel 5 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch5sel(&mut self) -> CH5SEL_W<CHMAP0_SPEC, 20> {
        CH5SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA channel 6 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch6sel(&mut self) -> CH6SEL_W<CHMAP0_SPEC, 24> {
        CH6SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA channel 7 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch7sel(&mut self) -> CH7SEL_W<CHMAP0_SPEC, 28> {
        CH7SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel map select 0 Each 4-bit field of the CHMAP0 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHMAP0_SPEC;
impl crate::RegisterSpec for CHMAP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap0::R`](R) reader structure"]
impl crate::Readable for CHMAP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chmap0::W`](W) writer structure"]
impl crate::Writable for CHMAP0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHMAP0 to value 0"]
impl crate::Resettable for CHMAP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
