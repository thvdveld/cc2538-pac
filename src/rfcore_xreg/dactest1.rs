#[doc = "Register `DACTEST1` reader"]
pub type R = crate::R<DACTEST1_SPEC>;
#[doc = "Register `DACTEST1` writer"]
pub type W = crate::W<DACTEST1_SPEC>;
#[doc = "Field `DAC_I_O` reader - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
pub type DAC_I_O_R = crate::FieldReader;
#[doc = "Field `DAC_I_O` writer - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
pub type DAC_I_O_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_i_o(&self) -> DAC_I_O_R {
        DAC_I_O_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    #[must_use]
    pub fn dac_i_o(&mut self) -> DAC_I_O_W<DACTEST1_SPEC, 0> {
        DAC_I_O_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DAC override value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dactest1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dactest1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DACTEST1_SPEC;
impl crate::RegisterSpec for DACTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dactest1::R`](R) reader structure"]
impl crate::Readable for DACTEST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dactest1::W`](W) writer structure"]
impl crate::Writable for DACTEST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACTEST1 to value 0"]
impl crate::Resettable for DACTEST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
