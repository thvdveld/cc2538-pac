#[doc = "Register `DACTEST1` reader"]
pub type R = crate::R<Dactest1Spec>;
#[doc = "Register `DACTEST1` writer"]
pub type W = crate::W<Dactest1Spec>;
#[doc = "Field `DAC_I_O` reader - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
pub type DacIOR = crate::FieldReader;
#[doc = "Field `DAC_I_O` writer - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
pub type DacIOW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_i_o(&self) -> DacIOR {
        DacIOR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I-branch DAC override value when DAC_SRC = 001 If DAC_SRC is set to be ADC data, CORDIC magnitude, channel filtered data, or DC filtered data, then DAC_I_O controls the part of the word in question that is actually multiplexed to the DAC as described below. 000111: Bits 7:0 001000: Bits 8:1 001001: Bits 9:2 ... If an invalid setting is chosen, then the DAC outputs only zeros (minimum value)."]
    #[inline(always)]
    pub fn dac_i_o(&mut self) -> DacIOW<Dactest1Spec> {
        DacIOW::new(self, 0)
    }
}
#[doc = "DAC override value\n\nYou can [`read`](crate::Reg::read) this register and get [`dactest1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dactest1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dactest1Spec;
impl crate::RegisterSpec for Dactest1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dactest1::R`](R) reader structure"]
impl crate::Readable for Dactest1Spec {}
#[doc = "`write(|w| ..)` method takes [`dactest1::W`](W) writer structure"]
impl crate::Writable for Dactest1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACTEST1 to value 0"]
impl crate::Resettable for Dactest1Spec {
    const RESET_VALUE: u32 = 0;
}
