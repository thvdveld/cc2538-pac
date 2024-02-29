#[doc = "Register `UARTRXD_UART0` reader"]
pub type R = crate::R<UartrxdUart0Spec>;
#[doc = "Register `UARTRXD_UART0` writer"]
pub type W = crate::W<UartrxdUart0Spec>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as UART0 RX 1: PA1 selected as UART0 RX ... 31: PD7 selected as UART0 RX"]
pub type InputSelR = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as UART0 RX 1: PA1 selected as UART0 RX ... 31: PD7 selected as UART0 RX"]
pub type InputSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as UART0 RX 1: PA1 selected as UART0 RX ... 31: PD7 selected as UART0 RX"]
    #[inline(always)]
    pub fn input_sel(&self) -> InputSelR {
        InputSelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as UART0 RX 1: PA1 selected as UART0 RX ... 31: PD7 selected as UART0 RX"]
    #[inline(always)]
    #[must_use]
    pub fn input_sel(&mut self) -> InputSelW<UartrxdUart0Spec> {
        InputSelW::new(self, 0)
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART0 RX.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartrxd_uart0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartrxd_uart0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartrxdUart0Spec;
impl crate::RegisterSpec for UartrxdUart0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartrxd_uart0::R`](R) reader structure"]
impl crate::Readable for UartrxdUart0Spec {}
#[doc = "`write(|w| ..)` method takes [`uartrxd_uart0::W`](W) writer structure"]
impl crate::Writable for UartrxdUart0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTRXD_UART0 to value 0x04"]
impl crate::Resettable for UartrxdUart0Spec {
    const RESET_VALUE: u32 = 0x04;
}
