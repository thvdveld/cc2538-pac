#[doc = "Register `SRCEXTEN0` reader"]
pub type R = crate::R<SRCEXTEN0_SPEC>;
#[doc = "Register `SRCEXTEN0` writer"]
pub type W = crate::W<SRCEXTEN0_SPEC>;
#[doc = "Field `EXT_ADDR_EN` reader - 7:0 part of the 24-bit word EXT_ADDR_EN that enables or disables source address matching for each of the 12 extended address table entries Write access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]. All EXT_ADDR_EN\\[2n + 1\\]
bits are read only. Read access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]
and EXT_ADDR_EN\\[2n + 1\\]. Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding EXT_ADDR_EN bit to 0 while updating."]
pub type EXT_ADDR_EN_R = crate::FieldReader;
#[doc = "Field `EXT_ADDR_EN` writer - 7:0 part of the 24-bit word EXT_ADDR_EN that enables or disables source address matching for each of the 12 extended address table entries Write access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]. All EXT_ADDR_EN\\[2n + 1\\]
bits are read only. Read access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]
and EXT_ADDR_EN\\[2n + 1\\]. Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding EXT_ADDR_EN bit to 0 while updating."]
pub type EXT_ADDR_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0 part of the 24-bit word EXT_ADDR_EN that enables or disables source address matching for each of the 12 extended address table entries Write access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]. All EXT_ADDR_EN\\[2n + 1\\]
bits are read only. Read access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]
and EXT_ADDR_EN\\[2n + 1\\]. Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding EXT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    pub fn ext_addr_en(&self) -> EXT_ADDR_EN_R {
        EXT_ADDR_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0 part of the 24-bit word EXT_ADDR_EN that enables or disables source address matching for each of the 12 extended address table entries Write access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]. All EXT_ADDR_EN\\[2n + 1\\]
bits are read only. Read access: Extended address enable for table entry n (0 to 11) is mapped to EXT_ADDR_EN\\[2n\\]
and EXT_ADDR_EN\\[2n + 1\\]. Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding EXT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr_en(&mut self) -> EXT_ADDR_EN_W<SRCEXTEN0_SPEC> {
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
#[doc = "Extended address matching\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcexten0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcexten0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCEXTEN0_SPEC;
impl crate::RegisterSpec for SRCEXTEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcexten0::R`](R) reader structure"]
impl crate::Readable for SRCEXTEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcexten0::W`](W) writer structure"]
impl crate::Writable for SRCEXTEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCEXTEN0 to value 0"]
impl crate::Resettable for SRCEXTEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
