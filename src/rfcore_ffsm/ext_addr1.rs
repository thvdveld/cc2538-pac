#[doc = "Register `EXT_ADDR1` reader"]
pub type R = crate::R<EXT_ADDR1_SPEC>;
#[doc = "Register `EXT_ADDR1` writer"]
pub type W = crate::W<EXT_ADDR1_SPEC>;
#[doc = "Field `EXT_ADDR1` reader - EXT_ADDR\\[15:8\\]
The IEEE extended address used during destination address filtering"]
pub type EXT_ADDR1_R = crate::FieldReader;
#[doc = "Field `EXT_ADDR1` writer - EXT_ADDR\\[15:8\\]
The IEEE extended address used during destination address filtering"]
pub type EXT_ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - EXT_ADDR\\[15:8\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr1(&self) -> EXT_ADDR1_R {
        EXT_ADDR1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXT_ADDR\\[15:8\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr1(&mut self) -> EXT_ADDR1_W<EXT_ADDR1_SPEC> {
        EXT_ADDR1_W::new(self, 0)
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
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_ADDR1_SPEC;
impl crate::RegisterSpec for EXT_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_addr1::R`](R) reader structure"]
impl crate::Readable for EXT_ADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_addr1::W`](W) writer structure"]
impl crate::Writable for EXT_ADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_ADDR1 to value 0"]
impl crate::Resettable for EXT_ADDR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
