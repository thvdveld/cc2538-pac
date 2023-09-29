#[doc = "Register `SRSSI` reader"]
pub type R = crate::R<SRSSI_SPEC>;
#[doc = "Register `SRSSI` writer"]
pub type W = crate::W<SRSSI_SPEC>;
#[doc = "Field `SSI0` reader - 0: SSI0 module is not reset 1: SSI0 module is reset"]
pub type SSI0_R = crate::BitReader;
#[doc = "Field `SSI0` writer - 0: SSI0 module is not reset 1: SSI0 module is reset"]
pub type SSI0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SSI1` reader - 0: SSI1 module is not reset 1: SSI1 module is reset"]
pub type SSI1_R = crate::BitReader;
#[doc = "Field `SSI1` writer - 0: SSI1 module is not reset 1: SSI1 module is reset"]
pub type SSI1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: SSI0 module is not reset 1: SSI0 module is reset"]
    #[inline(always)]
    pub fn ssi0(&self) -> SSI0_R {
        SSI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: SSI1 module is not reset 1: SSI1 module is reset"]
    #[inline(always)]
    pub fn ssi1(&self) -> SSI1_R {
        SSI1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: SSI0 module is not reset 1: SSI0 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn ssi0(&mut self) -> SSI0_W<SRSSI_SPEC, 0> {
        SSI0_W::new(self)
    }
    #[doc = "Bit 1 - 0: SSI1 module is not reset 1: SSI1 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn ssi1(&mut self) -> SSI1_W<SRSSI_SPEC, 1> {
        SSI1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls the reset for SSI\\[1:0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRSSI_SPEC;
impl crate::RegisterSpec for SRSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srssi::R`](R) reader structure"]
impl crate::Readable for SRSSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srssi::W`](W) writer structure"]
impl crate::Writable for SRSSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRSSI to value 0"]
impl crate::Resettable for SRSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
