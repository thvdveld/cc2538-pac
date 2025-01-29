#[doc = "Register `EXT_ADDR1` reader"]
pub type R = crate::R<ExtAddr1Spec>;
#[doc = "Register `EXT_ADDR1` writer"]
pub type W = crate::W<ExtAddr1Spec>;
#[doc = "Field `EXT_ADDR1` reader - EXT_ADDR\\[15:8\\]
The IEEE extended address used during destination address filtering"]
pub type ExtAddr1R = crate::FieldReader;
#[doc = "Field `EXT_ADDR1` writer - EXT_ADDR\\[15:8\\]
The IEEE extended address used during destination address filtering"]
pub type ExtAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXT_ADDR\\[15:8\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr1(&self) -> ExtAddr1R {
        ExtAddr1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXT_ADDR\\[15:8\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr1(&mut self) -> ExtAddr1W<ExtAddr1Spec> {
        ExtAddr1W::new(self, 0)
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtAddr1Spec;
impl crate::RegisterSpec for ExtAddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_addr1::R`](R) reader structure"]
impl crate::Readable for ExtAddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ext_addr1::W`](W) writer structure"]
impl crate::Writable for ExtAddr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_ADDR1 to value 0"]
impl crate::Resettable for ExtAddr1Spec {
    const RESET_VALUE: u32 = 0;
}
