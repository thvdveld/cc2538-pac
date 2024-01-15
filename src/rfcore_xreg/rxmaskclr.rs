#[doc = "Register `RXMASKCLR` reader"]
pub type R = crate::R<RXMASKCLR_SPEC>;
#[doc = "Register `RXMASKCLR` writer"]
pub type W = crate::W<RXMASKCLR_SPEC>;
#[doc = "Field `RXENMASKCLR` reader - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
pub type RXENMASKCLR_R = crate::FieldReader;
#[doc = "Field `RXENMASKCLR` writer - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
pub type RXENMASKCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
    #[inline(always)]
    pub fn rxenmaskclr(&self) -> RXENMASKCLR_R {
        RXENMASKCLR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
    #[inline(always)]
    #[must_use]
    pub fn rxenmaskclr(&mut self) -> RXENMASKCLR_W<RXMASKCLR_SPEC> {
        RXENMASKCLR_W::new(self, 0)
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
#[doc = "RX disabling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaskclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaskclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXMASKCLR_SPEC;
impl crate::RegisterSpec for RXMASKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmaskclr::R`](R) reader structure"]
impl crate::Readable for RXMASKCLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rxmaskclr::W`](W) writer structure"]
impl crate::Writable for RXMASKCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXMASKCLR to value 0"]
impl crate::Resettable for RXMASKCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
