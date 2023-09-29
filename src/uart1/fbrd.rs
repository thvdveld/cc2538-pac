#[doc = "Register `FBRD` reader"]
pub type R = crate::R<FBRD_SPEC>;
#[doc = "Register `FBRD` writer"]
pub type W = crate::W<FBRD_SPEC>;
#[doc = "Field `DIVFRAC` reader - Fractional baud-rate divisor"]
pub type DIVFRAC_R = crate::FieldReader;
#[doc = "Field `DIVFRAC` writer - Fractional baud-rate divisor"]
pub type DIVFRAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Fractional baud-rate divisor"]
    #[inline(always)]
    pub fn divfrac(&self) -> DIVFRAC_R {
        DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional baud-rate divisor"]
    #[inline(always)]
    #[must_use]
    pub fn divfrac(&mut self) -> DIVFRAC_W<FBRD_SPEC, 0> {
        DIVFRAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART fractional baud-rate divisor The FBRD register is the fractional part of the baud-rate divisor value. All the bits are cleared on reset. When changing the FBRD register, the new value does not take effect until transmission or reception of the current character is complete. Any changes to the baud-rate divisor must be followed by a write to the LCRH register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FBRD_SPEC;
impl crate::RegisterSpec for FBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbrd::R`](R) reader structure"]
impl crate::Readable for FBRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fbrd::W`](W) writer structure"]
impl crate::Writable for FBRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FBRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
