#[doc = "Register `MTCSPCFG` reader"]
pub struct R(crate::R<MTCSPCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTCSPCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTCSPCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTCSPCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTCSPCFG` writer"]
pub struct W(crate::W<MTCSPCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTCSPCFG_SPEC>;
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
impl From<crate::W<MTCSPCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTCSPCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACTIMER_EVENMT_CFG` reader - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub struct MACTIMER_EVENMT_CFG_R(crate::FieldReader<u8, u8>);
impl MACTIMER_EVENMT_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        MACTIMER_EVENMT_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_EVENMT_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_EVENMT_CFG` writer - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub struct MACTIMER_EVENMT_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_EVENMT_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `MACTIMER_EVENT1_CFG` reader - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub struct MACTIMER_EVENT1_CFG_R(crate::FieldReader<u8, u8>);
impl MACTIMER_EVENT1_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        MACTIMER_EVENT1_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACTIMER_EVENT1_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACTIMER_EVENT1_CFG` writer - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub struct MACTIMER_EVENT1_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> MACTIMER_EVENT1_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:6 - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_evenmt_cfg(&self) -> MACTIMER_EVENMT_CFG_R {
        MACTIMER_EVENMT_CFG_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_event1_cfg(&self) -> MACTIMER_EVENT1_CFG_R {
        MACTIMER_EVENT1_CFG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_evenmt_cfg(&mut self) -> MACTIMER_EVENMT_CFG_W {
        MACTIMER_EVENMT_CFG_W { w: self }
    }
    #[doc = "Bits 0:2 - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_event1_cfg(&mut self) -> MACTIMER_EVENT1_CFG_W {
        MACTIMER_EVENT1_CFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer event configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtcspcfg](index.html) module"]
pub struct MTCSPCFG_SPEC;
impl crate::RegisterSpec for MTCSPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtcspcfg::R](R) reader structure"]
impl crate::Readable for MTCSPCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtcspcfg::W](W) writer structure"]
impl crate::Writable for MTCSPCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTCSPCFG to value 0"]
impl crate::Resettable for MTCSPCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
