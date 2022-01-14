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
#[doc = "Field `DMA_IN_DONE` reader - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
pub struct DMA_IN_DONE_R(crate::FieldReader<bool, bool>);
impl DMA_IN_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_IN_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_IN_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_IN_DONE` writer - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
pub struct DMA_IN_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_IN_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RESULT_AV` reader - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
pub struct RESULT_AV_R(crate::FieldReader<bool, bool>);
impl RESULT_AV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESULT_AV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_AV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESULT_AV` writer - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
pub struct RESULT_AV_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_AV_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&self) -> RESULT_AV_R {
        RESULT_AV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - If this bit is set to 0, the DMA input done (irq_dma_in_done) interrupt output is disabled and remains 0. If this bit is set to 1, the DMA input done interrupt output is enabled."]
    #[inline(always)]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W {
        DMA_IN_DONE_W { w: self }
    }
    #[doc = "Bit 0 - If this bit is set to 0, the result available (irq_result_av) interrupt output is disabled and remains 0. If this bit is set to 1, the result available interrupt output is enabled."]
    #[inline(always)]
    pub fn result_av(&mut self) -> RESULT_AV_W {
        RESULT_AV_W { w: self }
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
