#[doc = "Register `SHORT_ADDR1` reader"]
pub type R = crate::R<ShortAddr1Spec>;
#[doc = "Register `SHORT_ADDR1` writer"]
pub type W = crate::W<ShortAddr1Spec>;
#[doc = "Field `SHORT_ADDR1` reader - SHORT_ADDR\\[15:8\\]
The short address used during destination address filtering"]
pub type ShortAddr1R = crate::FieldReader;
#[doc = "Field `SHORT_ADDR1` writer - SHORT_ADDR\\[15:8\\]
The short address used during destination address filtering"]
pub type ShortAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[15:8\\]
The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr1(&self) -> ShortAddr1R {
        ShortAddr1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[15:8\\]
The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr1(&mut self) -> ShortAddr1W<ShortAddr1Spec> {
        ShortAddr1W::new(self, 0)
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::Reg::read) this register and get [`short_addr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`short_addr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortAddr1Spec;
impl crate::RegisterSpec for ShortAddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`short_addr1::R`](R) reader structure"]
impl crate::Readable for ShortAddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`short_addr1::W`](W) writer structure"]
impl crate::Writable for ShortAddr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORT_ADDR1 to value 0"]
impl crate::Resettable for ShortAddr1Spec {
    const RESET_VALUE: u32 = 0;
}
