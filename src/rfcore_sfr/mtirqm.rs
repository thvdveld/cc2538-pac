#[doc = "Register `MTIRQM` reader"]
pub type R = crate::R<MtirqmSpec>;
#[doc = "Register `MTIRQM` writer"]
pub type W = crate::W<MtirqmSpec>;
#[doc = "Field `MACTIMER_PERM` reader - Enables the MACTIMER_PER interrupt"]
pub type MactimerPermR = crate::BitReader;
#[doc = "Field `MACTIMER_PERM` writer - Enables the MACTIMER_PER interrupt"]
pub type MactimerPermW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_COMPARE1M` reader - Enables the MACTIMER_COMPARE1 interrupt"]
pub type MactimerCompare1mR = crate::BitReader;
#[doc = "Field `MACTIMER_COMPARE1M` writer - Enables the MACTIMER_COMPARE1 interrupt"]
pub type MactimerCompare1mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_COMPARE2M` reader - Enables the MACTIMER_COMPARE2 interrupt"]
pub type MactimerCompare2mR = crate::BitReader;
#[doc = "Field `MACTIMER_COMPARE2M` writer - Enables the MACTIMER_COMPARE2 interrupt"]
pub type MactimerCompare2mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_PERM` reader - Enables the MACTIMER_OVF_PER interrupt"]
pub type MactimerOvfPermR = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_PERM` writer - Enables the MACTIMER_OVF_PER interrupt"]
pub type MactimerOvfPermW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_COMPARE1M` reader - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
pub type MactimerOvfCompare1mR = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_COMPARE1M` writer - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
pub type MactimerOvfCompare1mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_COMPARE2M` reader - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
pub type MactimerOvfCompare2mR = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_COMPARE2M` writer - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
pub type MactimerOvfCompare2mW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_perm(&self) -> MactimerPermR {
        MactimerPermR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare1m(&self) -> MactimerCompare1mR {
        MactimerCompare1mR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare2m(&self) -> MactimerCompare2mR {
        MactimerCompare2mR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_perm(&self) -> MactimerOvfPermR {
        MactimerOvfPermR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1m(&self) -> MactimerOvfCompare1mR {
        MactimerOvfCompare1mR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2m(&self) -> MactimerOvfCompare2mR {
        MactimerOvfCompare2mR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_perm(&mut self) -> MactimerPermW<MtirqmSpec> {
        MactimerPermW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_compare1m(&mut self) -> MactimerCompare1mW<MtirqmSpec> {
        MactimerCompare1mW::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_compare2m(&mut self) -> MactimerCompare2mW<MtirqmSpec> {
        MactimerCompare2mW::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_perm(&mut self) -> MactimerOvfPermW<MtirqmSpec> {
        MactimerOvfPermW::new(self, 3)
    }
    #[doc = "Bit 4 - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_compare1m(&mut self) -> MactimerOvfCompare1mW<MtirqmSpec> {
        MactimerOvfCompare1mW::new(self, 4)
    }
    #[doc = "Bit 5 - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_compare2m(&mut self) -> MactimerOvfCompare2mW<MtirqmSpec> {
        MactimerOvfCompare2mW::new(self, 5)
    }
}
#[doc = "MAC Timer interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtirqm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtirqm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtirqmSpec;
impl crate::RegisterSpec for MtirqmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtirqm::R`](R) reader structure"]
impl crate::Readable for MtirqmSpec {}
#[doc = "`write(|w| ..)` method takes [`mtirqm::W`](W) writer structure"]
impl crate::Writable for MtirqmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIRQM to value 0"]
impl crate::Resettable for MtirqmSpec {
    const RESET_VALUE: u32 = 0;
}
