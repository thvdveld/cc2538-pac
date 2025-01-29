#[doc = "Register `RXMASKCLR` reader"]
pub type R = crate::R<RxmaskclrSpec>;
#[doc = "Register `RXMASKCLR` writer"]
pub type W = crate::W<RxmaskclrSpec>;
#[doc = "Field `RXENMASKCLR` reader - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
pub type RxenmaskclrR = crate::FieldReader;
#[doc = "Field `RXENMASKCLR` writer - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
pub type RxenmaskclrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
    #[inline(always)]
    pub fn rxenmaskclr(&self) -> RxenmaskclrR {
        RxenmaskclrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
    #[inline(always)]
    pub fn rxenmaskclr(&mut self) -> RxenmaskclrW<RxmaskclrSpec> {
        RxenmaskclrW::new(self, 0)
    }
}
#[doc = "RX disabling\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmaskclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmaskclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmaskclrSpec;
impl crate::RegisterSpec for RxmaskclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmaskclr::R`](R) reader structure"]
impl crate::Readable for RxmaskclrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxmaskclr::W`](W) writer structure"]
impl crate::Writable for RxmaskclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXMASKCLR to value 0"]
impl crate::Resettable for RxmaskclrSpec {
    const RESET_VALUE: u32 = 0;
}
