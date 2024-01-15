#[doc = "Register `GPIOCR` reader"]
pub type R = crate::R<GPIOCR_SPEC>;
#[doc = "Register `GPIOCR` writer"]
pub type W = crate::W<GPIOCR_SPEC>;
#[doc = "Field `CR` reader - On a bit-wise basis, any bit set allows the corresponding GPIOAFSEL bit to be set to its alternate function."]
pub type CR_R = crate::FieldReader;
#[doc = "Field `CR` writer - On a bit-wise basis, any bit set allows the corresponding GPIOAFSEL bit to be set to its alternate function."]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - On a bit-wise basis, any bit set allows the corresponding GPIOAFSEL bit to be set to its alternate function."]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - On a bit-wise basis, any bit set allows the corresponding GPIOAFSEL bit to be set to its alternate function."]
    #[inline(always)]
    #[must_use]
    pub fn cr(&mut self) -> CR_W<GPIOCR_SPEC> {
        CR_W::new(self, 0)
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
#[doc = "The GPIOCR register is the commit register. The value of the GPIOCR register determines which bits of the AFSEL register is committed when a write to the AFSEL register is performed. If a bit in the GPIOCR register is 0, the data being written to the corresponding bit in the AFSEL register is not committed and retains its previous value. If a bit in the GPIOCR register is set to 1, the data being written to the corresponding bit of the AFSEL register is committed to the register and will reflect the new value. The contents of the GPIOCR register can only be modified if the GPIOLOCK register is unlocked. Writes to the GPIOCR register will be ignored if the GPIOLOCK register is locked. Any write to the commit register causes the lock register to be locked.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOCR_SPEC;
impl crate::RegisterSpec for GPIOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiocr::R`](R) reader structure"]
impl crate::Readable for GPIOCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpiocr::W`](W) writer structure"]
impl crate::Writable for GPIOCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOCR to value 0xff"]
impl crate::Resettable for GPIOCR_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
