#[doc = "Register `TXCTRL` reader"]
pub struct R(crate::R<TXCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXCTRL` writer"]
pub struct W(crate::W<TXCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXCTRL_SPEC>;
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
impl From<crate::W<TXCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC_CURR` reader - Change the current in the DAC."]
pub type DAC_CURR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_CURR` writer - Change the current in the DAC."]
pub type DAC_CURR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DAC_DC` reader - Adjusts the DC level to the TX mixer."]
pub type DAC_DC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAC_DC` writer - Adjusts the DC level to the TX mixer."]
pub type DAC_DC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXMIX_CURRENT` reader - Transmit mixers core current Current increases with increasing setting."]
pub type TXMIX_CURRENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXMIX_CURRENT` writer - Transmit mixers core current Current increases with increasing setting."]
pub type TXMIX_CURRENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXCTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 4:6 - Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&self) -> DAC_CURR_R {
        DAC_CURR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&self) -> DAC_DC_R {
        DAC_DC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&self) -> TXMIX_CURRENT_R {
        TXMIX_CURRENT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&mut self) -> DAC_CURR_W<4> {
        DAC_CURR_W::new(self)
    }
    #[doc = "Bits 2:3 - Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&mut self) -> DAC_DC_W<2> {
        DAC_DC_W::new(self)
    }
    #[doc = "Bits 0:1 - Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&mut self) -> TXMIX_CURRENT_W<0> {
        TXMIX_CURRENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the TX settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txctrl](index.html) module"]
pub struct TXCTRL_SPEC;
impl crate::RegisterSpec for TXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txctrl::R](R) reader structure"]
impl crate::Readable for TXCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txctrl::W](W) writer structure"]
impl crate::Writable for TXCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TXCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
