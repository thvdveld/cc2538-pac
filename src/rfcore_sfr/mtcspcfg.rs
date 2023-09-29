#[doc = "Register `MTCSPCFG` reader"]
pub type R = crate::R<MTCSPCFG_SPEC>;
#[doc = "Register `MTCSPCFG` writer"]
pub type W = crate::W<MTCSPCFG_SPEC>;
#[doc = "Field `MACTIMER_EVENT1_CFG` reader - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub type MACTIMER_EVENT1_CFG_R = crate::FieldReader;
#[doc = "Field `MACTIMER_EVENT1_CFG` writer - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub type MACTIMER_EVENT1_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `MACTIMER_EVENMT_CFG` reader - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub type MACTIMER_EVENMT_CFG_R = crate::FieldReader;
#[doc = "Field `MACTIMER_EVENMT_CFG` writer - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub type MACTIMER_EVENMT_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_event1_cfg(&self) -> MACTIMER_EVENT1_CFG_R {
        MACTIMER_EVENT1_CFG_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_evenmt_cfg(&self) -> MACTIMER_EVENMT_CFG_R {
        MACTIMER_EVENMT_CFG_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_event1_cfg(&mut self) -> MACTIMER_EVENT1_CFG_W<MTCSPCFG_SPEC, 0> {
        MACTIMER_EVENT1_CFG_W::new(self)
    }
    #[doc = "Bits 4:6 - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_evenmt_cfg(&mut self) -> MACTIMER_EVENMT_CFG_W<MTCSPCFG_SPEC, 4> {
        MACTIMER_EVENMT_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MAC Timer event configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtcspcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtcspcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTCSPCFG_SPEC;
impl crate::RegisterSpec for MTCSPCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtcspcfg::R`](R) reader structure"]
impl crate::Readable for MTCSPCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtcspcfg::W`](W) writer structure"]
impl crate::Writable for MTCSPCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTCSPCFG to value 0"]
impl crate::Resettable for MTCSPCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
