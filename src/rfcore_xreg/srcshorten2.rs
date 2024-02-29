#[doc = "Register `SRCSHORTEN2` reader"]
pub type R = crate::R<Srcshorten2Spec>;
#[doc = "Register `SRCSHORTEN2` writer"]
pub type W = crate::W<Srcshorten2Spec>;
#[doc = "Field `SHORT_ADDR_EN` reader - 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
pub type ShortAddrEnR = crate::FieldReader;
#[doc = "Field `SHORT_ADDR_EN` writer - 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
pub type ShortAddrEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
    #[inline(always)]
    pub fn short_addr_en(&self) -> ShortAddrEnR {
        ShortAddrEnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 23:16 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_en(&mut self) -> ShortAddrEnW<Srcshorten2Spec> {
        ShortAddrEnW::new(self, 0)
    }
}
#[doc = "Short address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshorten2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshorten2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcshorten2Spec;
impl crate::RegisterSpec for Srcshorten2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshorten2::R`](R) reader structure"]
impl crate::Readable for Srcshorten2Spec {}
#[doc = "`write(|w| ..)` method takes [`srcshorten2::W`](W) writer structure"]
impl crate::Writable for Srcshorten2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCSHORTEN2 to value 0"]
impl crate::Resettable for Srcshorten2Spec {
    const RESET_VALUE: u32 = 0;
}
