#[doc = "Register `EXT_ADDR0` reader"]
pub type R = crate::R<EXT_ADDR0_SPEC>;
#[doc = "Register `EXT_ADDR0` writer"]
pub type W = crate::W<EXT_ADDR0_SPEC>;
#[doc = "Field `EXT_ADDR0` reader - EXT_ADDR\\[7:0\\]
The IEEE extended address used during destination address filtering"]
pub type EXT_ADDR0_R = crate::FieldReader;
#[doc = "Field `EXT_ADDR0` writer - EXT_ADDR\\[7:0\\]
The IEEE extended address used during destination address filtering"]
pub type EXT_ADDR0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - EXT_ADDR\\[7:0\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    pub fn ext_addr0(&self) -> EXT_ADDR0_R {
        EXT_ADDR0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - EXT_ADDR\\[7:0\\]
The IEEE extended address used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn ext_addr0(&mut self) -> EXT_ADDR0_W<EXT_ADDR0_SPEC, 0> {
        EXT_ADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_addr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_addr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_ADDR0_SPEC;
impl crate::RegisterSpec for EXT_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_addr0::R`](R) reader structure"]
impl crate::Readable for EXT_ADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_addr0::W`](W) writer structure"]
impl crate::Writable for EXT_ADDR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_ADDR0 to value 0"]
impl crate::Resettable for EXT_ADDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
