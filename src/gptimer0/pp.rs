#[doc = "Register `PP` reader"]
pub struct R(crate::R<PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALTCLK` reader - Alternate clock source 0: Timer is not capable of using an alternate clock. 1: Timer is capable of using an alternate clock."]
pub struct ALTCLK_R(crate::FieldReader<bool, bool>);
impl ALTCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALTCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALTCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNCNT` reader - Synchronized start 0: Timer is not capable of synchronizing the count value with other timers. 1: Timer is capable of synchronizing the count value with other timers."]
pub struct SYNCNT_R(crate::FieldReader<bool, bool>);
impl SYNCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYNCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNCNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHAIN` reader - Chain with other timers 0: Timer is not capable of chaining with previously numbered Timers. 1: Timer is capable of chaining with previously numbered timers."]
pub struct CHAIN_R(crate::FieldReader<bool, bool>);
impl CHAIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CHAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHAIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` reader - Timer size 0: Timer A and Timer B are 16 bits wide with 8-bit prescale. 1: Timer A and Timer B are 32 bits wide with 16-bit prescale."]
pub struct SIZE_R(crate::FieldReader<u8, u8>);
impl SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 6 - Alternate clock source 0: Timer is not capable of using an alternate clock. 1: Timer is capable of using an alternate clock."]
    #[inline(always)]
    pub fn altclk(&self) -> ALTCLK_R {
        ALTCLK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Synchronized start 0: Timer is not capable of synchronizing the count value with other timers. 1: Timer is capable of synchronizing the count value with other timers."]
    #[inline(always)]
    pub fn syncnt(&self) -> SYNCNT_R {
        SYNCNT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Chain with other timers 0: Timer is not capable of chaining with previously numbered Timers. 1: Timer is capable of chaining with previously numbered timers."]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:3 - Timer size 0: Timer A and Timer B are 16 bits wide with 8-bit prescale. 1: Timer A and Timer B are 32 bits wide with 16-bit prescale."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "GPTM peripheral properties The PP register provides information regarding the properties of the general-purpose Timer module.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pp](index.html) module"]
pub struct PP_SPEC;
impl crate::RegisterSpec for PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pp::R](R) reader structure"]
impl crate::Readable for PP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PP to value 0"]
impl crate::Resettable for PP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
