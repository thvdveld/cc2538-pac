#[doc = "Register `SRUART` reader"]
pub type R = crate::R<SruartSpec>;
#[doc = "Register `SRUART` writer"]
pub type W = crate::W<SruartSpec>;
#[doc = "Field `UART0` reader - 0: UART0 module is not reset 1: UART0 module is reset"]
pub type Uart0R = crate::BitReader;
#[doc = "Field `UART0` writer - 0: UART0 module is not reset 1: UART0 module is reset"]
pub type Uart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1` reader - 0: UART1 module is not reset 1: UART1 module is reset"]
pub type Uart1R = crate::BitReader;
#[doc = "Field `UART1` writer - 0: UART1 module is not reset 1: UART1 module is reset"]
pub type Uart1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: UART0 module is not reset 1: UART0 module is reset"]
    #[inline(always)]
    pub fn uart0(&self) -> Uart0R {
        Uart0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: UART1 module is not reset 1: UART1 module is reset"]
    #[inline(always)]
    pub fn uart1(&self) -> Uart1R {
        Uart1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: UART0 module is not reset 1: UART0 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> Uart0W<SruartSpec> {
        Uart0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: UART1 module is not reset 1: UART1 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> Uart1W<SruartSpec> {
        Uart1W::new(self, 1)
    }
}
#[doc = "This register controls the reset for UART\\[1:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sruart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sruart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SruartSpec;
impl crate::RegisterSpec for SruartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sruart::R`](R) reader structure"]
impl crate::Readable for SruartSpec {}
#[doc = "`write(|w| ..)` method takes [`sruart::W`](W) writer structure"]
impl crate::Writable for SruartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRUART to value 0"]
impl crate::Resettable for SruartSpec {
    const RESET_VALUE: u32 = 0;
}
