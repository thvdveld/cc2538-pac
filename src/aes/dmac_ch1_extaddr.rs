#[doc = "Register `DMAC_CH1_EXTADDR` reader"]
pub struct R(crate::R<DMAC_CH1_EXTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CH1_EXTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CH1_EXTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CH1_EXTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_CH1_EXTADDR` writer"]
pub struct W(crate::W<DMAC_CH1_EXTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_CH1_EXTADDR_SPEC>;
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
impl From<crate::W<DMAC_CH1_EXTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_CH1_EXTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface."]
pub type ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR` writer - Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface."]
pub type ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMAC_CH1_EXTADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel external address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_ch1_extaddr](index.html) module"]
pub struct DMAC_CH1_EXTADDR_SPEC;
impl crate::RegisterSpec for DMAC_CH1_EXTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_ch1_extaddr::R](R) reader structure"]
impl crate::Readable for DMAC_CH1_EXTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_ch1_extaddr::W](W) writer structure"]
impl crate::Writable for DMAC_CH1_EXTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_CH1_EXTADDR to value 0"]
impl crate::Resettable for DMAC_CH1_EXTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
