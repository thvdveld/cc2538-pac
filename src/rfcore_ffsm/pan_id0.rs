#[doc = "Register `PAN_ID0` reader"]
pub type R = crate::R<PAN_ID0_SPEC>;
#[doc = "Register `PAN_ID0` writer"]
pub type W = crate::W<PAN_ID0_SPEC>;
#[doc = "Field `PAN_ID0` reader - PAN_ID\\[7:0\\]
The PAN ID used during destination address filtering"]
pub type PAN_ID0_R = crate::FieldReader;
#[doc = "Field `PAN_ID0` writer - PAN_ID\\[7:0\\]
The PAN ID used during destination address filtering"]
pub type PAN_ID0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PAN_ID\\[7:0\\]
The PAN ID used during destination address filtering"]
    #[inline(always)]
    pub fn pan_id0(&self) -> PAN_ID0_R {
        PAN_ID0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAN_ID\\[7:0\\]
The PAN ID used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id0(&mut self) -> PAN_ID0_W<PAN_ID0_SPEC> {
        PAN_ID0_W::new(self, 0)
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
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAN_ID0_SPEC;
impl crate::RegisterSpec for PAN_ID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pan_id0::R`](R) reader structure"]
impl crate::Readable for PAN_ID0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pan_id0::W`](W) writer structure"]
impl crate::Writable for PAN_ID0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAN_ID0 to value 0"]
impl crate::Resettable for PAN_ID0_SPEC {
    const RESET_VALUE: u32 = 0;
}
