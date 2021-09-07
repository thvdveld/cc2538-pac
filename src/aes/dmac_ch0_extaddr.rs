#[doc = "Register `DMAC_CH0_EXTADDR` reader"]
pub struct R(crate::R<DMAC_CH0_EXTADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_CH0_EXTADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_CH0_EXTADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_CH0_EXTADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_CH0_EXTADDR` writer"]
pub struct W(crate::W<DMAC_CH0_EXTADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_CH0_EXTADDR_SPEC>;
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
impl From<crate::W<DMAC_CH0_EXTADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_CH0_EXTADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface."]
pub struct ADDR_R(crate::FieldReader<u32, u32>);
impl ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface."]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits |= value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel external address value When read during operation, it holds the last updated external address after being sent to the master interface."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel external address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_ch0_extaddr](index.html) module"]
pub struct DMAC_CH0_EXTADDR_SPEC;
impl crate::RegisterSpec for DMAC_CH0_EXTADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_ch0_extaddr::R](R) reader structure"]
impl crate::Readable for DMAC_CH0_EXTADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_ch0_extaddr::W](W) writer structure"]
impl crate::Writable for DMAC_CH0_EXTADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_CH0_EXTADDR to value 0"]
impl crate::Resettable for DMAC_CH0_EXTADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
