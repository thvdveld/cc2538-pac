#[doc = "Register `SSIRXD_SSI0` reader"]
pub type R = crate::R<SsirxdSsi0Spec>;
#[doc = "Register `SSIRXD_SSI0` writer"]
pub type W = crate::W<SsirxdSsi0Spec>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as SSI0 RX 1: PA1 selected as SSI0 RX ... 31: PD7 selected as SSI0 RX"]
pub type InputSelR = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as SSI0 RX 1: PA1 selected as SSI0 RX ... 31: PD7 selected as SSI0 RX"]
pub type InputSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI0 RX 1: PA1 selected as SSI0 RX ... 31: PD7 selected as SSI0 RX"]
    #[inline(always)]
    pub fn input_sel(&self) -> InputSelR {
        InputSelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI0 RX 1: PA1 selected as SSI0 RX ... 31: PD7 selected as SSI0 RX"]
    #[inline(always)]
    pub fn input_sel(&mut self) -> InputSelW<SsirxdSsi0Spec> {
        InputSelW::new(self, 0)
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 RX.\n\nYou can [`read`](crate::Reg::read) this register and get [`ssirxd_ssi0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssirxd_ssi0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsirxdSsi0Spec;
impl crate::RegisterSpec for SsirxdSsi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssirxd_ssi0::R`](R) reader structure"]
impl crate::Readable for SsirxdSsi0Spec {}
#[doc = "`write(|w| ..)` method takes [`ssirxd_ssi0::W`](W) writer structure"]
impl crate::Writable for SsirxdSsi0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSIRXD_SSI0 to value 0"]
impl crate::Resettable for SsirxdSsi0Spec {
    const RESET_VALUE: u32 = 0;
}
