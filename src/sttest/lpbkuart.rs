#[doc = "Register `LPBKUART` reader"]
pub type R = crate::R<LpbkuartSpec>;
#[doc = "Register `LPBKUART` writer"]
pub type W = crate::W<LpbkuartSpec>;
#[doc = "Field `LPBKUART` reader - UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
pub type LpbkuartR = crate::BitReader;
#[doc = "Field `LPBKUART` writer - UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
pub type LpbkuartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
    #[inline(always)]
    pub fn lpbkuart(&self) -> LpbkuartR {
        LpbkuartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
    #[inline(always)]
    pub fn lpbkuart(&mut self) -> LpbkuartW<LpbkuartSpec> {
        LpbkuartW::new(self, 0)
    }
}
#[doc = "UART internal loopback\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbkuart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbkuart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpbkuartSpec;
impl crate::RegisterSpec for LpbkuartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbkuart::R`](R) reader structure"]
impl crate::Readable for LpbkuartSpec {}
#[doc = "`write(|w| ..)` method takes [`lpbkuart::W`](W) writer structure"]
impl crate::Writable for LpbkuartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPBKUART to value 0"]
impl crate::Resettable for LpbkuartSpec {
    const RESET_VALUE: u32 = 0;
}
