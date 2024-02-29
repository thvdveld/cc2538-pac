#[doc = "Register `EXT_ADDR7` reader"]
pub type R = crate::R<ExtAddr7Spec>;
#[doc = "Register `EXT_ADDR7` writer"]
pub type W = crate::W<ExtAddr7Spec>;
#[doc = "Field `EXT_ADDR7` reader - EXT_ADDR\\[63:56\\]
The IEEE extended address used during destination address filtering"]
pub type ExtAddr7R = crate::FieldReader;
#[doc = "Field `EXT_ADDR7` writer - EXT_ADDR\\[63:56\\]
The IEEE extended address used during destination address filtering"]
pub type ExtAddr7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXT_ADDR\\[63:56\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr7(&self) -> ExtAddr7R {
        ExtAddr7R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXT_ADDR\\[63:56\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr7(&mut self) -> ExtAddr7W<ExtAddr7Spec> {
        ExtAddr7W::new(self, 0)
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtAddr7Spec;
impl crate::RegisterSpec for ExtAddr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_addr7::R`](R) reader structure"]
impl crate::Readable for ExtAddr7Spec {}
#[doc = "`write(|w| ..)` method takes [`ext_addr7::W`](W) writer structure"]
impl crate::Writable for ExtAddr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_ADDR7 to value 0"]
impl crate::Resettable for ExtAddr7Spec {
    const RESET_VALUE: u32 = 0;
}
