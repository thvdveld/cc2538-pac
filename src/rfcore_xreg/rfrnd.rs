#[doc = "Register `RFRND` reader"]
pub struct R(crate::R<RFRND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFRND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFRND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFRND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IRND` reader - Random bit from the I channel of the receiver"]
pub type IRND_R = crate::BitReader<bool>;
#[doc = "Field `QRND` reader - Random bit from the Q channel of the receiver"]
pub type QRND_R = crate::BitReader<bool>;
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
#[doc = "Random data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfrnd](index.html) module"]
pub struct RFRND_SPEC;
impl crate::RegisterSpec for RFRND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfrnd::R](R) reader structure"]
impl crate::Readable for RFRND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFRND to value 0"]
impl crate::Resettable for RFRND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
