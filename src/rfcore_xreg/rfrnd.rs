#[doc = "Register `RFRND` reader"]
pub type R = crate::R<RFRND_SPEC>;
#[doc = "Field `IRND` reader - Random bit from the I channel of the receiver"]
pub type IRND_R = crate::BitReader;
#[doc = "Field `QRND` reader - Random bit from the Q channel of the receiver"]
pub type QRND_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Random bit from the I channel of the receiver"]
    #[inline(always)]
    pub fn irnd(&self) -> IRND_R {
        IRND_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Random bit from the Q channel of the receiver"]
    #[inline(always)]
    pub fn qrnd(&self) -> QRND_R {
        QRND_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Random data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfrnd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFRND_SPEC;
impl crate::RegisterSpec for RFRND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfrnd::R`](R) reader structure"]
impl crate::Readable for RFRND_SPEC {}
#[doc = "`reset()` method sets RFRND to value 0"]
impl crate::Resettable for RFRND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
