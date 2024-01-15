#[doc = "Register `SRCSHORTEN0` reader"]
pub type R = crate::R<SRCSHORTEN0_SPEC>;
#[doc = "Register `SRCSHORTEN0` writer"]
pub type W = crate::W<SRCSHORTEN0_SPEC>;
#[doc = "Field `SHORT_ADDR_EN` reader - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
pub type SHORT_ADDR_EN_R = crate::FieldReader;
#[doc = "Field `SHORT_ADDR_EN` writer - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
pub type SHORT_ADDR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    pub fn short_addr_en(&self) -> SHORT_ADDR_EN_R {
        SHORT_ADDR_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    #[must_use]
    pub fn short_addr_en(&mut self) -> SHORT_ADDR_EN_W<SRCSHORTEN0_SPEC> {
        SHORT_ADDR_EN_W::new(self, 0)
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
#[doc = "Short address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshorten0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshorten0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCSHORTEN0_SPEC;
impl crate::RegisterSpec for SRCSHORTEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshorten0::R`](R) reader structure"]
impl crate::Readable for SRCSHORTEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcshorten0::W`](W) writer structure"]
impl crate::Writable for SRCSHORTEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCSHORTEN0 to value 0"]
impl crate::Resettable for SRCSHORTEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
