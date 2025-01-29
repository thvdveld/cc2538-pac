#[doc = "Register `SSIRXD_SSI1` reader"]
pub type R = crate::R<SsirxdSsi1Spec>;
#[doc = "Register `SSIRXD_SSI1` writer"]
pub type W = crate::W<SsirxdSsi1Spec>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as SSI1 RX 1: PA1 selected as SSI1 RX ... 31: PD7 selected as SSI1 RX"]
pub type InputSelR = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as SSI1 RX 1: PA1 selected as SSI1 RX ... 31: PD7 selected as SSI1 RX"]
pub type InputSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI1 RX 1: PA1 selected as SSI1 RX ... 31: PD7 selected as SSI1 RX"]
    #[inline(always)]
    pub fn input_sel(&self) -> InputSelR {
        InputSelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI1 RX 1: PA1 selected as SSI1 RX ... 31: PD7 selected as SSI1 RX"]
    #[inline(always)]
    pub fn input_sel(&mut self) -> InputSelW<SsirxdSsi1Spec> {
        InputSelW::new(self, 0)
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 RX.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssirxd_ssi1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssirxd_ssi1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsirxdSsi1Spec;
impl crate::RegisterSpec for SsirxdSsi1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssirxd_ssi1::R`](R) reader structure"]
impl crate::Readable for SsirxdSsi1Spec {}
#[doc = "`write(|w| ..)` method takes [`ssirxd_ssi1::W`](W) writer structure"]
impl crate::Writable for SsirxdSsi1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSIRXD_SSI1 to value 0"]
impl crate::Resettable for SsirxdSsi1Spec {
    const RESET_VALUE: u32 = 0;
}
