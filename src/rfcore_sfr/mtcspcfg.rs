#[doc = "Register `MTCSPCFG` reader"]
pub type R = crate::R<MtcspcfgSpec>;
#[doc = "Register `MTCSPCFG` writer"]
pub type W = crate::W<MtcspcfgSpec>;
#[doc = "Field `MACTIMER_EVENT1_CFG` reader - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub type MactimerEvent1CfgR = crate::FieldReader;
#[doc = "Field `MACTIMER_EVENT1_CFG` writer - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub type MactimerEvent1CfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MACTIMER_EVENMT_CFG` reader - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub type MactimerEvenmtCfgR = crate::FieldReader;
#[doc = "Field `MACTIMER_EVENMT_CFG` writer - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
pub type MactimerEvenmtCfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_event1_cfg(&self) -> MactimerEvent1CfgR {
        MactimerEvent1CfgR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_evenmt_cfg(&self) -> MactimerEvenmtCfgR {
        MactimerEvenmtCfgR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects the event that triggers an MT_EVENT1 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_event1_cfg(&mut self) -> MactimerEvent1CfgW<MtcspcfgSpec> {
        MactimerEvent1CfgW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Selects the event that triggers an MT_EVENT2 pulse 000: MT_per_event 001: MT_cmp1_event 010: MT_cmp2_event 011: MTovf_per_event 100: MTovf_cmp1_event 101: MTovf_cmp2_event 110: Reserved 111: No event"]
    #[inline(always)]
    pub fn mactimer_evenmt_cfg(&mut self) -> MactimerEvenmtCfgW<MtcspcfgSpec> {
        MactimerEvenmtCfgW::new(self, 4)
    }
}
#[doc = "MAC Timer event configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mtcspcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtcspcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtcspcfgSpec;
impl crate::RegisterSpec for MtcspcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtcspcfg::R`](R) reader structure"]
impl crate::Readable for MtcspcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mtcspcfg::W`](W) writer structure"]
impl crate::Writable for MtcspcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTCSPCFG to value 0"]
impl crate::Resettable for MtcspcfgSpec {
    const RESET_VALUE: u32 = 0;
}
