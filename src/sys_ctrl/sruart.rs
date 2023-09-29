#[doc = "Register `SRUART` reader"]
pub type R = crate::R<SRUART_SPEC>;
#[doc = "Register `SRUART` writer"]
pub type W = crate::W<SRUART_SPEC>;
#[doc = "Field `UART0` reader - 0: UART0 module is not reset 1: UART0 module is reset"]
pub type UART0_R = crate::BitReader;
#[doc = "Field `UART0` writer - 0: UART0 module is not reset 1: UART0 module is reset"]
pub type UART0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART1` reader - 0: UART1 module is not reset 1: UART1 module is reset"]
pub type UART1_R = crate::BitReader;
#[doc = "Field `UART1` writer - 0: UART1 module is not reset 1: UART1 module is reset"]
pub type UART1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: UART0 module is not reset 1: UART0 module is reset"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: UART1 module is not reset 1: UART1 module is reset"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: UART0 module is not reset 1: UART0 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<SRUART_SPEC, 0> {
        UART0_W::new(self)
    }
    #[doc = "Bit 1 - 0: UART1 module is not reset 1: UART1 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<SRUART_SPEC, 1> {
        UART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls the reset for UART\\[1:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sruart::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sruart::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRUART_SPEC;
impl crate::RegisterSpec for SRUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sruart::R`](R) reader structure"]
impl crate::Readable for SRUART_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sruart::W`](W) writer structure"]
impl crate::Writable for SRUART_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRUART to value 0"]
impl crate::Resettable for SRUART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
