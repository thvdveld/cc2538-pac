#[doc = "Register `PAN_ID1` reader"]
pub type R = crate::R<PAN_ID1_SPEC>;
#[doc = "Register `PAN_ID1` writer"]
pub type W = crate::W<PAN_ID1_SPEC>;
#[doc = "Field `PAN_ID1` reader - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
pub type PAN_ID1_R = crate::FieldReader;
#[doc = "Field `PAN_ID1` writer - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
pub type PAN_ID1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
    #[inline(always)]
    pub fn pan_id1(&self) -> PAN_ID1_R {
        PAN_ID1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id1(&mut self) -> PAN_ID1_W<PAN_ID1_SPEC> {
        PAN_ID1_W::new(self, 0)
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
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAN_ID1_SPEC;
impl crate::RegisterSpec for PAN_ID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pan_id1::R`](R) reader structure"]
impl crate::Readable for PAN_ID1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pan_id1::W`](W) writer structure"]
impl crate::Writable for PAN_ID1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAN_ID1 to value 0"]
impl crate::Resettable for PAN_ID1_SPEC {
    const RESET_VALUE: u32 = 0;
}
