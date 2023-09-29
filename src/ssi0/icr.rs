#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICR_SPEC>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `RORIC` reader - SSI receive overrun interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
pub type RORIC_R = crate::BitReader;
#[doc = "Field `RORIC` writer - SSI receive overrun interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
pub type RORIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTIC` reader - SSI receive time-out interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
pub type RTIC_R = crate::BitReader;
#[doc = "Field `RTIC` writer - SSI receive time-out interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
pub type RTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SSI receive overrun interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
    #[inline(always)]
    pub fn roric(&self) -> RORIC_R {
        RORIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI receive overrun interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn roric(&mut self) -> RORIC_W<ICR_SPEC, 0> {
        RORIC_W::new(self)
    }
    #[doc = "Bit 1 - SSI receive time-out interrupt clear (W1C) Reset value: 0x0 0: No effect on interrupt 1: Clears interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<ICR_SPEC, 1> {
        RTIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The ICR register is the interrupt clear register. On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
