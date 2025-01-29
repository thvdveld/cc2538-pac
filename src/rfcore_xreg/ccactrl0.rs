#[doc = "Register `CCACTRL0` reader"]
pub type R = crate::R<Ccactrl0Spec>;
#[doc = "Register `CCACTRL0` writer"]
pub type W = crate::W<Ccactrl0Spec>;
#[doc = "Field `CCA_THR` reader - Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
pub type CcaThrR = crate::FieldReader;
#[doc = "Field `CCA_THR` writer - Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
pub type CcaThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
    #[inline(always)]
    pub fn cca_thr(&self) -> CcaThrR {
        CcaThrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clear-channel-assessment threshold value, signed 2's-complement number for comparison with the RSSI. The unit is 1 dB, offset is 73dB The CCA signal goes high when the received signal is below this value. The CCA signal is available on the CCA pin and in the FSMSTAT1 register. The value must never be set lower than CCA_HYST - 128 to avoid erroneous behavior of the CCA signal. Note: The reset value translates to an input level of approximately -32 - 73 = -105 dBm, which is well below the sensitivity limit. This means that the CCA signal never indicates a clear channel. This register should be updated to 0xF8, which translates to an input level of about -8 - 73 = -81 dBm."]
    #[inline(always)]
    pub fn cca_thr(&mut self) -> CcaThrW<Ccactrl0Spec> {
        CcaThrW::new(self, 0)
    }
}
#[doc = "CCA threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`ccactrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccactrl0Spec;
impl crate::RegisterSpec for Ccactrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccactrl0::R`](R) reader structure"]
impl crate::Readable for Ccactrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ccactrl0::W`](W) writer structure"]
impl crate::Writable for Ccactrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCACTRL0 to value 0"]
impl crate::Resettable for Ccactrl0Spec {
    const RESET_VALUE: u32 = 0;
}
