#[doc = "Register `CHMAP2` reader"]
pub type R = crate::R<CHMAP2_SPEC>;
#[doc = "Register `CHMAP2` writer"]
pub type W = crate::W<CHMAP2_SPEC>;
#[doc = "Field `CH16SEL` reader - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH16SEL_R = crate::FieldReader;
#[doc = "Field `CH16SEL` writer - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH16SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH17SEL` reader - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH17SEL_R = crate::FieldReader;
#[doc = "Field `CH17SEL` writer - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH17SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH18SEL` reader - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH18SEL_R = crate::FieldReader;
#[doc = "Field `CH18SEL` writer - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH18SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH19SEL` reader - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH19SEL_R = crate::FieldReader;
#[doc = "Field `CH19SEL` writer - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH19SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH20SEL` reader - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH20SEL_R = crate::FieldReader;
#[doc = "Field `CH20SEL` writer - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH20SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH21SEL` reader - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH21SEL_R = crate::FieldReader;
#[doc = "Field `CH21SEL` writer - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH21SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH22SEL` reader - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH22SEL_R = crate::FieldReader;
#[doc = "Field `CH22SEL` writer - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH22SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `CH23SEL` reader - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH23SEL_R = crate::FieldReader;
#[doc = "Field `CH23SEL` writer - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
pub type CH23SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch16sel(&self) -> CH16SEL_R {
        CH16SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch17sel(&self) -> CH17SEL_R {
        CH17SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch18sel(&self) -> CH18SEL_R {
        CH18SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch19sel(&self) -> CH19SEL_R {
        CH19SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch20sel(&self) -> CH20SEL_R {
        CH20SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch21sel(&self) -> CH21SEL_R {
        CH21SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch22sel(&self) -> CH22SEL_R {
        CH22SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    pub fn ch23sel(&self) -> CH23SEL_R {
        CH23SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - uDMA channel 16 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch16sel(&mut self) -> CH16SEL_W<CHMAP2_SPEC, 0> {
        CH16SEL_W::new(self)
    }
    #[doc = "Bits 4:7 - uDMA channel 17 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch17sel(&mut self) -> CH17SEL_W<CHMAP2_SPEC, 4> {
        CH17SEL_W::new(self)
    }
    #[doc = "Bits 8:11 - uDMA channel 18 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch18sel(&mut self) -> CH18SEL_W<CHMAP2_SPEC, 8> {
        CH18SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - uDMA channel 19 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch19sel(&mut self) -> CH19SEL_W<CHMAP2_SPEC, 12> {
        CH19SEL_W::new(self)
    }
    #[doc = "Bits 16:19 - uDMA channel 20 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch20sel(&mut self) -> CH20SEL_W<CHMAP2_SPEC, 16> {
        CH20SEL_W::new(self)
    }
    #[doc = "Bits 20:23 - uDMA channel 21 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch21sel(&mut self) -> CH21SEL_W<CHMAP2_SPEC, 20> {
        CH21SEL_W::new(self)
    }
    #[doc = "Bits 24:27 - uDMA channel 22 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch22sel(&mut self) -> CH22SEL_W<CHMAP2_SPEC, 24> {
        CH22SEL_W::new(self)
    }
    #[doc = "Bits 28:31 - uDMA channel 23 source select See section titled \"Channel Assignments\" in Micro Direct Memory Access chapter."]
    #[inline(always)]
    #[must_use]
    pub fn ch23sel(&mut self) -> CH23SEL_W<CHMAP2_SPEC, 28> {
        CH23SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel map select 2 Each 4-bit field of the CHMAP2 register configures the uDMA channel assignment as specified in the uDMA channel assignment table in the \"Channel Assignments\" section.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chmap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chmap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHMAP2_SPEC;
impl crate::RegisterSpec for CHMAP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chmap2::R`](R) reader structure"]
impl crate::Readable for CHMAP2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chmap2::W`](W) writer structure"]
impl crate::Writable for CHMAP2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHMAP2 to value 0"]
impl crate::Resettable for CHMAP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
