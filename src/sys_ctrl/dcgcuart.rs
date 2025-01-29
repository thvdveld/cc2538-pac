#[doc = "Register `DCGCUART` reader"]
pub type R = crate::R<DcgcuartSpec>;
#[doc = "Register `DCGCUART` writer"]
pub type W = crate::W<DcgcuartSpec>;
#[doc = "Field `UART0` reader - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART0` writer - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
    #[inline(always)]
    pub fn uart0(&mut self) -> Uart0W<DcgcuartSpec> {
        Uart0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
    #[inline(always)]
    pub fn uart1(&mut self) -> Uart1W<DcgcuartSpec> {
        Uart1W::new(self, 1)
    }
}
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcgcuart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcgcuart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcgcuartSpec;
impl crate::RegisterSpec for DcgcuartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcuart::R`](R) reader structure"]
impl crate::Readable for DcgcuartSpec {}
#[doc = "`write(|w| ..)` method takes [`dcgcuart::W`](W) writer structure"]
impl crate::Writable for DcgcuartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCGCUART to value 0"]
impl crate::Resettable for DcgcuartSpec {
    const RESET_VALUE: u32 = 0;
}
