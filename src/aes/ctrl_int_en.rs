#[doc = "Register `CTRL_INT_EN` reader"]
pub struct R(crate::R<CTRL_INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_INT_EN` writer"]
pub struct W(crate::W<CTRL_INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_INT_EN_SPEC>;
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
impl From<crate::W<CTRL_INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESULT_AV` reader - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
pub type RESULT_AV_R = crate::BitReader<bool>;
#[doc = "Field `RESULT_AV` writer - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
pub type RESULT_AV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_INT_EN_SPEC, bool, O>;
#[doc = "Field `DMA_IN_DONE` reader - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
pub type DMA_IN_DONE_R = crate::BitReader<bool>;
#[doc = "Field `DMA_IN_DONE` writer - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
pub type DMA_IN_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_INT_EN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&self) -> RESULT_AV_R {
        RESULT_AV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&mut self) -> RESULT_AV_W<0> {
        RESULT_AV_W::new(self)
    }
    #[doc = "Bit 1 - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W<1> {
        DMA_IN_DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_int_en](index.html) module"]
pub struct CTRL_INT_EN_SPEC;
impl crate::RegisterSpec for CTRL_INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_int_en::R](R) reader structure"]
impl crate::Readable for CTRL_INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_int_en::W](W) writer structure"]
impl crate::Writable for CTRL_INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_INT_EN to value 0"]
impl crate::Resettable for CTRL_INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
