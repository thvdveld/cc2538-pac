#[doc = "Register `TXCTRL` reader"]
pub type R = crate::R<TXCTRL_SPEC>;
#[doc = "Register `TXCTRL` writer"]
pub type W = crate::W<TXCTRL_SPEC>;
#[doc = "Field `TXMIX_CURRENT` reader - Transmit mixers core current Current increases with increasing setting."]
pub type TXMIX_CURRENT_R = crate::FieldReader;
#[doc = "Field `TXMIX_CURRENT` writer - Transmit mixers core current Current increases with increasing setting."]
pub type TXMIX_CURRENT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DAC_DC` reader - Adjusts the DC level to the TX mixer."]
pub type DAC_DC_R = crate::FieldReader;
#[doc = "Field `DAC_DC` writer - Adjusts the DC level to the TX mixer."]
pub type DAC_DC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DAC_CURR` reader - Change the current in the DAC."]
pub type DAC_CURR_R = crate::FieldReader;
#[doc = "Field `DAC_CURR` writer - Change the current in the DAC."]
pub type DAC_CURR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:1 - Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    pub fn txmix_current(&self) -> TXMIX_CURRENT_R {
        TXMIX_CURRENT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    pub fn dac_dc(&self) -> DAC_DC_R {
        DAC_DC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Change the current in the DAC."]
    #[inline(always)]
    pub fn dac_curr(&self) -> DAC_CURR_R {
        DAC_CURR_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit mixers core current Current increases with increasing setting."]
    #[inline(always)]
    #[must_use]
    pub fn txmix_current(&mut self) -> TXMIX_CURRENT_W<TXCTRL_SPEC, 0> {
        TXMIX_CURRENT_W::new(self)
    }
    #[doc = "Bits 2:3 - Adjusts the DC level to the TX mixer."]
    #[inline(always)]
    #[must_use]
    pub fn dac_dc(&mut self) -> DAC_DC_W<TXCTRL_SPEC, 2> {
        DAC_DC_W::new(self)
    }
    #[doc = "Bits 4:6 - Change the current in the DAC."]
    #[inline(always)]
    #[must_use]
    pub fn dac_curr(&mut self) -> DAC_CURR_W<TXCTRL_SPEC, 4> {
        DAC_CURR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Controls the TX settings\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXCTRL_SPEC;
impl crate::RegisterSpec for TXCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txctrl::R`](R) reader structure"]
impl crate::Readable for TXCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txctrl::W`](W) writer structure"]
impl crate::Writable for TXCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXCTRL to value 0"]
impl crate::Resettable for TXCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
