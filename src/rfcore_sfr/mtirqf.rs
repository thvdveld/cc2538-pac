#[doc = "Register `MTIRQF` reader"]
pub type R = crate::R<MtirqfSpec>;
#[doc = "Register `MTIRQF` writer"]
pub type W = crate::W<MtirqfSpec>;
#[doc = "Field `MACTIMER_PERF` reader - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
pub type MactimerPerfR = crate::BitReader;
#[doc = "Field `MACTIMER_PERF` writer - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
pub type MactimerPerfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_COMPARE1F` reader - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
pub type MactimerCompare1fR = crate::BitReader;
#[doc = "Field `MACTIMER_COMPARE1F` writer - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
pub type MactimerCompare1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_COMPARE2F` reader - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
pub type MactimerCompare2fR = crate::BitReader;
#[doc = "Field `MACTIMER_COMPARE2F` writer - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
pub type MactimerCompare2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_PERF` reader - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
pub type MactimerOvfPerfR = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_PERF` writer - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
pub type MactimerOvfPerfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_COMPARE1F` reader - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
pub type MactimerOvfCompare1fR = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_COMPARE1F` writer - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
pub type MactimerOvfCompare1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_COMPARE2F` reader - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
pub type MactimerOvfCompare2fR = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_COMPARE2F` writer - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
pub type MactimerOvfCompare2fW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_perf(&self) -> MactimerPerfR {
        MactimerPerfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
    #[inline(always)]
    pub fn mactimer_compare1f(&self) -> MactimerCompare1fR {
        MactimerCompare1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
    #[inline(always)]
    pub fn mactimer_compare2f(&self) -> MactimerCompare2fR {
        MactimerCompare2fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_ovf_perf(&self) -> MactimerOvfPerfR {
        MactimerOvfPerfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1f(&self) -> MactimerOvfCompare1fR {
        MactimerOvfCompare1fR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2f(&self) -> MactimerOvfCompare2fR {
        MactimerOvfCompare2fR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_perf(&mut self) -> MactimerPerfW<MtirqfSpec> {
        MactimerPerfW::new(self, 0)
    }
    #[doc = "Bit 1 - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
    #[inline(always)]
    pub fn mactimer_compare1f(&mut self) -> MactimerCompare1fW<MtirqfSpec> {
        MactimerCompare1fW::new(self, 1)
    }
    #[doc = "Bit 2 - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
    #[inline(always)]
    pub fn mactimer_compare2f(&mut self) -> MactimerCompare2fW<MtirqfSpec> {
        MactimerCompare2fW::new(self, 2)
    }
    #[doc = "Bit 3 - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_ovf_perf(&mut self) -> MactimerOvfPerfW<MtirqfSpec> {
        MactimerOvfPerfW::new(self, 3)
    }
    #[doc = "Bit 4 - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1f(&mut self) -> MactimerOvfCompare1fW<MtirqfSpec> {
        MactimerOvfCompare1fW::new(self, 4)
    }
    #[doc = "Bit 5 - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2f(&mut self) -> MactimerOvfCompare2fW<MtirqfSpec> {
        MactimerOvfCompare2fW::new(self, 5)
    }
}
#[doc = "MAC Timer interrupt flags\n\nYou can [`read`](crate::Reg::read) this register and get [`mtirqf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtirqf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtirqfSpec;
impl crate::RegisterSpec for MtirqfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtirqf::R`](R) reader structure"]
impl crate::Readable for MtirqfSpec {}
#[doc = "`write(|w| ..)` method takes [`mtirqf::W`](W) writer structure"]
impl crate::Writable for MtirqfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIRQF to value 0"]
impl crate::Resettable for MtirqfSpec {
    const RESET_VALUE: u32 = 0;
}
