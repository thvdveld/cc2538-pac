#[doc = "Register `SCGCUART` reader"]
pub type R = crate::R<ScgcuartSpec>;
#[doc = "Register `SCGCUART` writer"]
pub type W = crate::W<ScgcuartSpec>;
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
    #[must_use]
    pub fn uart0(&mut self) -> Uart0W<ScgcuartSpec> {
        Uart0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> Uart1W<ScgcuartSpec> {
        Uart1W::new(self, 1)
    }
}
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcuart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcuart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScgcuartSpec;
impl crate::RegisterSpec for ScgcuartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcuart::R`](R) reader structure"]
impl crate::Readable for ScgcuartSpec {}
#[doc = "`write(|w| ..)` method takes [`scgcuart::W`](W) writer structure"]
impl crate::Writable for ScgcuartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGCUART to value 0"]
impl crate::Resettable for ScgcuartSpec {
    const RESET_VALUE: u32 = 0;
}
