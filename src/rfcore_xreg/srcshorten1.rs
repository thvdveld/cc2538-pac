#[doc = "Register `SRCSHORTEN1` reader"]
pub type R = crate::R<Srcshorten1Spec>;
#[doc = "Register `SRCSHORTEN1` writer"]
pub type W = crate::W<Srcshorten1Spec>;
#[doc = "Field `SHORT_ADDR_EN` reader - 15:8 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
pub type ShortAddrEnR = crate::FieldReader;
#[doc = "Field `SHORT_ADDR_EN` writer - 15:8 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
pub type ShortAddrEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 15:8 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
    #[inline(always)]
    pub fn short_addr_en(&self) -> ShortAddrEnR {
        ShortAddrEnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 15:8 part of the 24-bit word SHORT_ADDR_EN See description of SRCSHORTEN0.SHORT_ADDR_EN."]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_en(&mut self) -> ShortAddrEnW<Srcshorten1Spec> {
        ShortAddrEnW::new(self, 0)
    }
}
#[doc = "Short address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshorten1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshorten1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcshorten1Spec;
impl crate::RegisterSpec for Srcshorten1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshorten1::R`](R) reader structure"]
impl crate::Readable for Srcshorten1Spec {}
#[doc = "`write(|w| ..)` method takes [`srcshorten1::W`](W) writer structure"]
impl crate::Writable for Srcshorten1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCSHORTEN1 to value 0"]
impl crate::Resettable for Srcshorten1Spec {
    const RESET_VALUE: u32 = 0;
}
