#[doc = "Register `SRCEXTEN1` reader"]
pub type R = crate::R<SRCEXTEN1_SPEC>;
#[doc = "Register `SRCEXTEN1` writer"]
pub type W = crate::W<SRCEXTEN1_SPEC>;
#[doc = "Field `EXT_ADDR_EN` reader - 15:8 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
pub type EXT_ADDR_EN_R = crate::FieldReader;
#[doc = "Field `EXT_ADDR_EN` writer - 15:8 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
pub type EXT_ADDR_EN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 15:8 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
    #[inline(always)]
    pub fn ext_addr_en(&self) -> EXT_ADDR_EN_R {
        EXT_ADDR_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 15:8 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr_en(&mut self) -> EXT_ADDR_EN_W<SRCEXTEN1_SPEC, 0> {
        EXT_ADDR_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Extended address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcexten1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcexten1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCEXTEN1_SPEC;
impl crate::RegisterSpec for SRCEXTEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcexten1::R`](R) reader structure"]
impl crate::Readable for SRCEXTEN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcexten1::W`](W) writer structure"]
impl crate::Writable for SRCEXTEN1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCEXTEN1 to value 0"]
impl crate::Resettable for SRCEXTEN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
