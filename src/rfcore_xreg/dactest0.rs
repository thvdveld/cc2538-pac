#[doc = "Register `DACTEST0` reader"]
pub type R = crate::R<DACTEST0_SPEC>;
#[doc = "Register `DACTEST0` writer"]
pub type W = crate::W<DACTEST0_SPEC>;
#[doc = "Field `DAC_Q_O` reader - Q-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, or channel filtered data, then DAC_Q_O controls the part of the word in question that is actually multiplexed to the DAC, as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, the DAC outputs only zeros (minimum value)."]
pub type DAC_Q_O_R = crate::FieldReader;
#[doc = "Field `DAC_Q_O` writer - Q-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, or channel filtered data, then DAC_Q_O controls the part of the word in question that is actually multiplexed to the DAC, as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, the DAC outputs only zeros (minimum value)."]
pub type DAC_Q_O_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Q-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, or channel filtered data, then DAC_Q_O controls the part of the word in question that is actually multiplexed to the DAC, as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_q_o(&self) -> DAC_Q_O_R {
        DAC_Q_O_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Q-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, or channel filtered data, then DAC_Q_O controls the part of the word in question that is actually multiplexed to the DAC, as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    #[must_use]
    pub fn dac_q_o(&mut self) -> DAC_Q_O_W<DACTEST0_SPEC, 0> {
        DAC_Q_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC override value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dactest0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dactest0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACTEST0_SPEC;
impl crate::RegisterSpec for DACTEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dactest0::R`](R) reader structure"]
impl crate::Readable for DACTEST0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dactest0::W`](W) writer structure"]
impl crate::Writable for DACTEST0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACTEST0 to value 0"]
impl crate::Resettable for DACTEST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
