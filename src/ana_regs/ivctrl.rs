#[doc = "Register `IVCTRL` reader"]
pub struct R(crate::R<IVCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IVCTRL` writer"]
pub struct W(crate::W<IVCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVCTRL_SPEC>;
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
impl From<crate::W<IVCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA_BIAS_CTRL` reader - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
pub type PA_BIAS_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA_BIAS_CTRL` writer - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
pub type PA_BIAS_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IVCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXMIX_DC_CTRL` reader - Controls DC bias in TXMIX"]
pub type TXMIX_DC_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `TXMIX_DC_CTRL` writer - Controls DC bias in TXMIX"]
pub type TXMIX_DC_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IVCTRL_SPEC, bool, O>;
#[doc = "Field `LODIV_BIAS_CTRL` reader - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
pub type LODIV_BIAS_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `LODIV_BIAS_CTRL` writer - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
pub type LODIV_BIAS_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IVCTRL_SPEC, bool, O>;
#[doc = "Field `DAC_CURR_CTRL` reader - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
pub type DAC_CURR_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_CURR_CTRL` writer - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
pub type DAC_CURR_CTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IVCTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&self) -> PA_BIAS_CTRL_R {
        PA_BIAS_CTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&self) -> TXMIX_DC_CTRL_R {
        TXMIX_DC_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&self) -> LODIV_BIAS_CTRL_R {
        LODIV_BIAS_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&self) -> DAC_CURR_CTRL_R {
        DAC_CURR_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&mut self) -> PA_BIAS_CTRL_W<0> {
        PA_BIAS_CTRL_W::new(self)
    }
    #[doc = "Bit 2 - Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&mut self) -> TXMIX_DC_CTRL_W<2> {
        TXMIX_DC_CTRL_W::new(self)
    }
    #[doc = "Bit 3 - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&mut self) -> LODIV_BIAS_CTRL_W<3> {
        LODIV_BIAS_CTRL_W::new(self)
    }
    #[doc = "Bits 4:5 - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&mut self) -> DAC_CURR_CTRL_W<4> {
        DAC_CURR_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivctrl](index.html) module"]
pub struct IVCTRL_SPEC;
impl crate::RegisterSpec for IVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivctrl::R](R) reader structure"]
impl crate::Readable for IVCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ivctrl::W](W) writer structure"]
impl crate::Writable for IVCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IVCTRL to value 0"]
impl crate::Resettable for IVCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
