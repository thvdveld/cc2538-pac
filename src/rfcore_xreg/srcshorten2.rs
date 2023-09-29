#[doc = "Register `SRCSHORTEN2` reader"]
pub type R = crate::R<SRCSHORTEN2_SPEC>;
#[doc = "Register `SRCSHORTEN2` writer"]
pub type W = crate::W<SRCSHORTEN2_SPEC>;
#[doc = "Field `SHORT_ADDR_EN` reader - 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
pub type SHORT_ADDR_EN_R = crate::FieldReader;
#[doc = "Field `SHORT_ADDR_EN` writer - 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
pub type SHORT_ADDR_EN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
    #[inline(always)]
    pub fn short_addr_en(&self) -> SHORT_ADDR_EN_R {
        SHORT_ADDR_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_en(&mut self) -> SHORT_ADDR_EN_W<SRCSHORTEN2_SPEC, 0> {
        SHORT_ADDR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Short address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshorten2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshorten2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCSHORTEN2_SPEC;
impl crate::RegisterSpec for SRCSHORTEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshorten2::R`](R) reader structure"]
impl crate::Readable for SRCSHORTEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcshorten2::W`](W) writer structure"]
impl crate::Writable for SRCSHORTEN2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCSHORTEN2 to value 0"]
impl crate::Resettable for SRCSHORTEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
