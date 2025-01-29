#[doc = "Register `EXT_ADDR0` reader"]
pub type R = crate::R<ExtAddr0Spec>;
#[doc = "Register `EXT_ADDR0` writer"]
pub type W = crate::W<ExtAddr0Spec>;
#[doc = "Field `EXT_ADDR0` reader - EXT_ADDR\\[7:0\\]
The IEEE extended address used during destination address filtering"]
pub type ExtAddr0R = crate::FieldReader;
#[doc = "Field `EXT_ADDR0` writer - EXT_ADDR\\[7:0\\]
The IEEE extended address used during destination address filtering"]
pub type ExtAddr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXT_ADDR\\[7:0\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr0(&self) -> ExtAddr0R {
        ExtAddr0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXT_ADDR\\[7:0\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr0(&mut self) -> ExtAddr0W<ExtAddr0Spec> {
        ExtAddr0W::new(self, 0)
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtAddr0Spec;
impl crate::RegisterSpec for ExtAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_addr0::R`](R) reader structure"]
impl crate::Readable for ExtAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ext_addr0::W`](W) writer structure"]
impl crate::Writable for ExtAddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_ADDR0 to value 0"]
impl crate::Resettable for ExtAddr0Spec {
    const RESET_VALUE: u32 = 0;
}
