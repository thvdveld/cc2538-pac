#[doc = "Register `DMAC_CH0_DMALENGTH` reader"]
pub struct R(crate::R<DMAC_CH0_DMALENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CH0_DMALENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CH0_DMALENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CH0_DMALENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_CH0_DMALENGTH` writer"]
pub struct W(crate::W<DMAC_CH0_DMALENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_CH0_DMALENGTH_SPEC>;
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
impl From<crate::W<DMAC_CH0_DMALENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_CH0_DMALENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMALEN` reader - Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
pub type DMALEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMALEN` writer - Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
pub type DMALEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAC_CH0_DMALENGTH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    pub fn dmalen(&self) -> DMALEN_R {
        DMALEN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel DMA length in bytes During configuration, this register contains the DMA transfer length in bytes. During operation, it contains the last updated value of the DMA transfer length after being sent to the master interface. Note: Setting this register to a nonzero value starts the transfer if the channel is enabled. Therefore, this register must be written last when setting up a DMA channel."]
    #[inline(always)]
    pub fn dmalen(&mut self) -> DMALEN_W<0> {
        DMALEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel DMA length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_ch0_dmalength](index.html) module"]
pub struct DMAC_CH0_DMALENGTH_SPEC;
impl crate::RegisterSpec for DMAC_CH0_DMALENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_ch0_dmalength::R](R) reader structure"]
impl crate::Readable for DMAC_CH0_DMALENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_ch0_dmalength::W](W) writer structure"]
impl crate::Writable for DMAC_CH0_DMALENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_CH0_DMALENGTH to value 0"]
impl crate::Resettable for DMAC_CH0_DMALENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
