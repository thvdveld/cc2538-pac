#[doc = "Register `SRCEXTEN2` reader"]
pub type R = crate::R<Srcexten2Spec>;
#[doc = "Register `SRCEXTEN2` writer"]
pub type W = crate::W<Srcexten2Spec>;
#[doc = "Field `EXT_ADDR_EN` reader - 23:16 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
pub type ExtAddrEnR = crate::FieldReader;
#[doc = "Field `EXT_ADDR_EN` writer - 23:16 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
pub type ExtAddrEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 23:16 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
    #[inline(always)]
    pub fn ext_addr_en(&self) -> ExtAddrEnR {
        ExtAddrEnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 23:16 part of the 24-bit word EXT_ADDR_EN See description of SRCEXTEN0.EXT_ADDR_EN."]
    #[inline(always)]
    pub fn ext_addr_en(&mut self) -> ExtAddrEnW<Srcexten2Spec> {
        ExtAddrEnW::new(self, 0)
    }
}
#[doc = "Extended address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`srcexten2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcexten2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcexten2Spec;
impl crate::RegisterSpec for Srcexten2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcexten2::R`](R) reader structure"]
impl crate::Readable for Srcexten2Spec {}
#[doc = "`write(|w| ..)` method takes [`srcexten2::W`](W) writer structure"]
impl crate::Writable for Srcexten2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCEXTEN2 to value 0"]
impl crate::Resettable for Srcexten2Spec {
    const RESET_VALUE: u32 = 0;
}
