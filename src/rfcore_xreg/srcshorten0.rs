#[doc = "Register `SRCSHORTEN0` reader"]
pub type R = crate::R<Srcshorten0Spec>;
#[doc = "Register `SRCSHORTEN0` writer"]
pub type W = crate::W<Srcshorten0Spec>;
#[doc = "Field `SHORT_ADDR_EN` reader - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
pub type ShortAddrEnR = crate::FieldReader;
#[doc = "Field `SHORT_ADDR_EN` writer - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
pub type ShortAddrEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    pub fn short_addr_en(&self) -> ShortAddrEnR {
        ShortAddrEnR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    pub fn short_addr_en(&mut self) -> ShortAddrEnW<Srcshorten0Spec> {
        ShortAddrEnW::new(self, 0)
    }
}
#[doc = "Short address matching\n\nYou can [`read`](crate::Reg::read) this register and get [`srcshorten0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcshorten0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srcshorten0Spec;
impl crate::RegisterSpec for Srcshorten0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshorten0::R`](R) reader structure"]
impl crate::Readable for Srcshorten0Spec {}
#[doc = "`write(|w| ..)` method takes [`srcshorten0::W`](W) writer structure"]
impl crate::Writable for Srcshorten0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCSHORTEN0 to value 0"]
impl crate::Resettable for Srcshorten0Spec {
    const RESET_VALUE: u32 = 0;
}
