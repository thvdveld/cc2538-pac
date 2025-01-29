#[doc = "Register `PTME1` reader"]
pub type R = crate::R<Ptme1Spec>;
#[doc = "Register `PTME1` writer"]
pub type W = crate::W<Ptme1Spec>;
#[doc = "Field `UART0TME` reader - UART0 test mode enable"]
pub type Uart0tmeR = crate::BitReader;
#[doc = "Field `UART0TME` writer - UART0 test mode enable"]
pub type Uart0tmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1TME` reader - UART1 test mode enable"]
pub type Uart1tmeR = crate::BitReader;
#[doc = "Field `UART1TME` writer - UART1 test mode enable"]
pub type Uart1tmeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - UART0 test mode enable"]
    #[inline(always)]
    pub fn uart0tme(&self) -> Uart0tmeR {
        Uart0tmeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART1 test mode enable"]
    #[inline(always)]
    pub fn uart1tme(&self) -> Uart1tmeR {
        Uart1tmeR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - UART0 test mode enable"]
    #[inline(always)]
    pub fn uart0tme(&mut self) -> Uart0tmeW<Ptme1Spec> {
        Uart0tmeW::new(self, 8)
    }
    #[doc = "Bit 9 - UART1 test mode enable"]
    #[inline(always)]
    pub fn uart1tme(&mut self) -> Uart1tmeW<Ptme1Spec> {
        Uart1tmeW::new(self, 9)
    }
}
#[doc = "Peripheral test mode enable 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ptme1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptme1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ptme1Spec;
impl crate::RegisterSpec for Ptme1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptme1::R`](R) reader structure"]
impl crate::Readable for Ptme1Spec {}
#[doc = "`write(|w| ..)` method takes [`ptme1::W`](W) writer structure"]
impl crate::Writable for Ptme1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTME1 to value 0"]
impl crate::Resettable for Ptme1Spec {
    const RESET_VALUE: u32 = 0;
}
