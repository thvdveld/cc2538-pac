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
#[doc = "Field `QRND` reader - Random bit from the Q channel of the receiver"]
pub struct QRND_R(crate::FieldReader<bool, bool>);
impl QRND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QRND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QRND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRND` reader - Random bit from the I channel of the receiver"]
pub struct IRND_R(crate::FieldReader<bool, bool>);
impl IRND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IRND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Random bit from the Q channel of the receiver"]
    #[inline(always)]
    pub fn qrnd(&self) -> QRND_R {
        QRND_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Random bit from the I channel of the receiver"]
    #[inline(always)]
    pub fn irnd(&self) -> IRND_R {
        IRND_R::new((self.bits & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
