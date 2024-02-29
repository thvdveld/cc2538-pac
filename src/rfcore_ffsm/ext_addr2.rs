#[doc = "Register `EXT_ADDR2` reader"]
pub type R = crate::R<ExtAddr2Spec>;
#[doc = "Register `EXT_ADDR2` writer"]
pub type W = crate::W<ExtAddr2Spec>;
#[doc = "Field `EXT_ADDR2` reader - EXT_ADDR\\[23:16\\]
The IEEE extended address used during destination address filtering"]
pub type ExtAddr2R = crate::FieldReader;
#[doc = "Field `EXT_ADDR2` writer - EXT_ADDR\\[23:16\\]
The IEEE extended address used during destination address filtering"]
pub type ExtAddr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXT_ADDR\\[23:16\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr2(&self) -> ExtAddr2R {
        ExtAddr2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXT_ADDR\\[23:16\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr2(&mut self) -> ExtAddr2W<ExtAddr2Spec> {
        ExtAddr2W::new(self, 0)
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtAddr2Spec;
impl crate::RegisterSpec for ExtAddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_addr2::R`](R) reader structure"]
impl crate::Readable for ExtAddr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ext_addr2::W`](W) writer structure"]
impl crate::Writable for ExtAddr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_ADDR2 to value 0"]
impl crate::Resettable for ExtAddr2Spec {
    const RESET_VALUE: u32 = 0;
}
