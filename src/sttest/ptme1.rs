#[doc = "Register `PTME1` reader"]
pub type R = crate::R<PTME1_SPEC>;
#[doc = "Register `PTME1` writer"]
pub type W = crate::W<PTME1_SPEC>;
#[doc = "Field `UART0TME` reader - UART0 test mode enable"]
pub type UART0TME_R = crate::BitReader;
#[doc = "Field `UART0TME` writer - UART0 test mode enable"]
pub type UART0TME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `UART1TME` reader - UART1 test mode enable"]
pub type UART1TME_R = crate::BitReader;
#[doc = "Field `UART1TME` writer - UART1 test mode enable"]
pub type UART1TME_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 8 - UART0 test mode enable"]
    #[inline(always)]
    pub fn uart0tme(&self) -> UART0TME_R {
        UART0TME_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART1 test mode enable"]
    #[inline(always)]
    pub fn uart1tme(&self) -> UART1TME_R {
        UART1TME_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - UART0 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart0tme(&mut self) -> UART0TME_W<PTME1_SPEC, 8> {
        UART0TME_W::new(self)
    }
    #[doc = "Bit 9 - UART1 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart1tme(&mut self) -> UART1TME_W<PTME1_SPEC, 9> {
        UART1TME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Peripheral test mode enable 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptme1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptme1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTME1_SPEC;
impl crate::RegisterSpec for PTME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptme1::R`](R) reader structure"]
impl crate::Readable for PTME1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptme1::W`](W) writer structure"]
impl crate::Writable for PTME1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTME1 to value 0"]
impl crate::Resettable for PTME1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
