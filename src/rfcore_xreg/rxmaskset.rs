#[doc = "Register `RXMASKSET` reader"]
pub type R = crate::R<RxmasksetSpec>;
#[doc = "Register `RXMASKSET` writer"]
pub type W = crate::W<RxmasksetSpec>;
#[doc = "Field `RXENMASKSET` reader - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
pub type RxenmasksetR = crate::FieldReader;
#[doc = "Field `RXENMASKSET` writer - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
pub type RxenmasksetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
    #[inline(always)]
    pub fn rxenmaskset(&self) -> RxenmasksetR {
        RxenmasksetR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - When written, the written data is ORed with the RXENMASK and stored in RXENMASK."]
    #[inline(always)]
    #[must_use]
    pub fn rxenmaskset(&mut self) -> RxenmasksetW<RxmasksetSpec> {
        RxenmasksetW::new(self, 0)
    }
}
#[doc = "RX enabling\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxmaskset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxmaskset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmasksetSpec;
impl crate::RegisterSpec for RxmasksetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmaskset::R`](R) reader structure"]
impl crate::Readable for RxmasksetSpec {}
#[doc = "`write(|w| ..)` method takes [`rxmaskset::W`](W) writer structure"]
impl crate::Writable for RxmasksetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXMASKSET to value 0"]
impl crate::Resettable for RxmasksetSpec {
    const RESET_VALUE: u32 = 0;
}
