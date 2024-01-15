#[doc = "Register `SRCEXTEN2` reader"]
pub type R = crate::R<SRCEXTEN2_SPEC>;
#[doc = "Register `SRCEXTEN2` writer"]
pub type W = crate::W<SRCEXTEN2_SPEC>;
#[doc = "Field `EXT_ADDR_EN` reader - 23:16 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
pub type EXT_ADDR_EN_R = crate::FieldReader;
#[doc = "Field `EXT_ADDR_EN` writer - 23:16 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
pub type EXT_ADDR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 23:16 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
    #[inline(always)]
    pub fn ext_addr_en(&self) -> EXT_ADDR_EN_R {
        EXT_ADDR_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 23:16 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr_en(&mut self) -> EXT_ADDR_EN_W<SRCEXTEN2_SPEC> {
        EXT_ADDR_EN_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Extended address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcexten2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcexten2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCEXTEN2_SPEC;
impl crate::RegisterSpec for SRCEXTEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcexten2::R`](R) reader structure"]
impl crate::Readable for SRCEXTEN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcexten2::W`](W) writer structure"]
impl crate::Writable for SRCEXTEN2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCEXTEN2 to value 0"]
impl crate::Resettable for SRCEXTEN2_SPEC {
    const RESET_VALUE: u32 = 0;
}
