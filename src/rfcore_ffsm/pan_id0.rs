#[doc = "Register `PAN_ID0` reader"]
pub type R = crate::R<PanId0Spec>;
#[doc = "Register `PAN_ID0` writer"]
pub type W = crate::W<PanId0Spec>;
#[doc = "Field `PAN_ID0` reader - PAN_ID\\[7:0\\]
The PAN ID used during destination address filtering"]
pub type PanId0R = crate::FieldReader;
#[doc = "Field `PAN_ID0` writer - PAN_ID\\[7:0\\]
The PAN ID used during destination address filtering"]
pub type PanId0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - PAN_ID\\[7:0\\]
The PAN ID used during destination address filtering"]
    #[inline(always)]
    pub fn pan_id0(&self) -> PanId0R {
        PanId0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAN_ID\\[7:0\\]
The PAN ID used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id0(&mut self) -> PanId0W<PanId0Spec> {
        PanId0W::new(self, 0)
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pan_id0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pan_id0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PanId0Spec;
impl crate::RegisterSpec for PanId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pan_id0::R`](R) reader structure"]
impl crate::Readable for PanId0Spec {}
#[doc = "`write(|w| ..)` method takes [`pan_id0::W`](W) writer structure"]
impl crate::Writable for PanId0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAN_ID0 to value 0"]
impl crate::Resettable for PanId0Spec {
    const RESET_VALUE: u32 = 0;
}
