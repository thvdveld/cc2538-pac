#[doc = "Register `MDMCTRL1` reader"]
pub type R = crate::R<MDMCTRL1_SPEC>;
#[doc = "Register `MDMCTRL1` writer"]
pub type W = crate::W<MDMCTRL1_SPEC>;
#[doc = "Field `CORR_THR` reader - Demodulator correlator threshold value, required before SFD search. Threshold value adjusts how the receiver synchronizes to data from the radio. If the threshold is set too low, sync can more easily be found on noise. If set too high, the sensitivity is reduced, but sync is not likely to be found on noise. In combination with DEM_NUM_ZEROS, the system can be tuned so sensitivity is high with less sync found on noise."]
pub type CORR_THR_R = crate::FieldReader;
#[doc = "Field `CORR_THR` writer - Demodulator correlator threshold value, required before SFD search. Threshold value adjusts how the receiver synchronizes to data from the radio. If the threshold is set too low, sync can more easily be found on noise. If set too high, the sensitivity is reduced, but sync is not likely to be found on noise. In combination with DEM_NUM_ZEROS, the system can be tuned so sensitivity is high with less sync found on noise."]
pub type CORR_THR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CORR_THR_SFD` reader - Defines requirements for SFD detection: 0: The correlation value of one of the zero symbols of the preamble must be above the correlation threshold. 1: The correlation value of one zero symbol of the preamble and both symbols in the SFD must be above the correlation threshold."]
pub type CORR_THR_SFD_R = crate::BitReader;
#[doc = "Field `CORR_THR_SFD` writer - Defines requirements for SFD detection: 0: The correlation value of one of the zero symbols of the preamble must be above the correlation threshold. 1: The correlation value of one zero symbol of the preamble and both symbols in the SFD must be above the correlation threshold."]
pub type CORR_THR_SFD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Demodulator correlator threshold value, required before SFD search. Threshold value adjusts how the receiver synchronizes to data from the radio. If the threshold is set too low, sync can more easily be found on noise. If set too high, the sensitivity is reduced, but sync is not likely to be found on noise. In combination with DEM_NUM_ZEROS, the system can be tuned so sensitivity is high with less sync found on noise."]
    #[inline(always)]
    pub fn corr_thr(&self) -> CORR_THR_R {
        CORR_THR_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Defines requirements for SFD detection: 0: The correlation value of one of the zero symbols of the preamble must be above the correlation threshold. 1: The correlation value of one zero symbol of the preamble and both symbols in the SFD must be above the correlation threshold."]
    #[inline(always)]
    pub fn corr_thr_sfd(&self) -> CORR_THR_SFD_R {
        CORR_THR_SFD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Demodulator correlator threshold value, required before SFD search. Threshold value adjusts how the receiver synchronizes to data from the radio. If the threshold is set too low, sync can more easily be found on noise. If set too high, the sensitivity is reduced, but sync is not likely to be found on noise. In combination with DEM_NUM_ZEROS, the system can be tuned so sensitivity is high with less sync found on noise."]
    #[inline(always)]
    #[must_use]
    pub fn corr_thr(&mut self) -> CORR_THR_W<MDMCTRL1_SPEC> {
        CORR_THR_W::new(self, 0)
    }
    #[doc = "Bit 5 - Defines requirements for SFD detection: 0: The correlation value of one of the zero symbols of the preamble must be above the correlation threshold. 1: The correlation value of one zero symbol of the preamble and both symbols in the SFD must be above the correlation threshold."]
    #[inline(always)]
    #[must_use]
    pub fn corr_thr_sfd(&mut self) -> CORR_THR_SFD_W<MDMCTRL1_SPEC> {
        CORR_THR_SFD_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Controls modem\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdmctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdmctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMCTRL1_SPEC;
impl crate::RegisterSpec for MDMCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdmctrl1::R`](R) reader structure"]
impl crate::Readable for MDMCTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mdmctrl1::W`](W) writer structure"]
impl crate::Writable for MDMCTRL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMCTRL1 to value 0"]
impl crate::Resettable for MDMCTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
