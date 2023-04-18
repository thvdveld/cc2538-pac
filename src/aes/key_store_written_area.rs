#[doc = "Register `KEY_STORE_WRITTEN_AREA` reader"]
pub struct R(crate::R<KEY_STORE_WRITTEN_AREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_STORE_WRITTEN_AREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_STORE_WRITTEN_AREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_STORE_WRITTEN_AREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_STORE_WRITTEN_AREA` writer"]
pub struct W(crate::W<KEY_STORE_WRITTEN_AREA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_STORE_WRITTEN_AREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<KEY_STORE_WRITTEN_AREA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_STORE_WRITTEN_AREA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_AREA_WRITTEN0` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN0_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA_WRITTEN0` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITTEN_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA_WRITTEN1` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN1_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA_WRITTEN1` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITTEN_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA_WRITTEN2` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN2_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA_WRITTEN2` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITTEN_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA_WRITTEN3` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN3_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA_WRITTEN3` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITTEN_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA_WRITTEN4` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN4_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA_WRITTEN4` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITTEN_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA_WRITTEN5` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN5_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA_WRITTEN5` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITTEN_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA_WRITTEN6` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN6_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA_WRITTEN6` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITTEN_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA_WRITTEN7` reader - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN7_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA_WRITTEN7` writer - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
pub type RAM_AREA_WRITTEN7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITTEN_AREA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written0(&self) -> RAM_AREA_WRITTEN0_R {
        RAM_AREA_WRITTEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written1(&self) -> RAM_AREA_WRITTEN1_R {
        RAM_AREA_WRITTEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written2(&self) -> RAM_AREA_WRITTEN2_R {
        RAM_AREA_WRITTEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written3(&self) -> RAM_AREA_WRITTEN3_R {
        RAM_AREA_WRITTEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written4(&self) -> RAM_AREA_WRITTEN4_R {
        RAM_AREA_WRITTEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written5(&self) -> RAM_AREA_WRITTEN5_R {
        RAM_AREA_WRITTEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written6(&self) -> RAM_AREA_WRITTEN6_R {
        RAM_AREA_WRITTEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    pub fn ram_area_written7(&self) -> RAM_AREA_WRITTEN7_R {
        RAM_AREA_WRITTEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written0(&mut self) -> RAM_AREA_WRITTEN0_W<0> {
        RAM_AREA_WRITTEN0_W::new(self)
    }
    #[doc = "Bit 1 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written1(&mut self) -> RAM_AREA_WRITTEN1_W<1> {
        RAM_AREA_WRITTEN1_W::new(self)
    }
    #[doc = "Bit 2 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written2(&mut self) -> RAM_AREA_WRITTEN2_W<2> {
        RAM_AREA_WRITTEN2_W::new(self)
    }
    #[doc = "Bit 3 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written3(&mut self) -> RAM_AREA_WRITTEN3_W<3> {
        RAM_AREA_WRITTEN3_W::new(self)
    }
    #[doc = "Bit 4 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written4(&mut self) -> RAM_AREA_WRITTEN4_W<4> {
        RAM_AREA_WRITTEN4_W::new(self)
    }
    #[doc = "Bit 5 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written5(&mut self) -> RAM_AREA_WRITTEN5_W<5> {
        RAM_AREA_WRITTEN5_W::new(self)
    }
    #[doc = "Bit 6 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written6(&mut self) -> RAM_AREA_WRITTEN6_W<6> {
        RAM_AREA_WRITTEN6_W::new(self)
    }
    #[doc = "Bit 7 - Read operation: 0: This RAM area is not written with valid key information. 1: This RAM area is written with valid key information. Each individual ram_area_writtenx bit can be reset by writing 1. Note: This register is reset on a soft reset from the master control module. After a soft reset, all keys must be rewritten to the key store memory."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area_written7(&mut self) -> RAM_AREA_WRITTEN7_W<7> {
        RAM_AREA_WRITTEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_store_written_area](index.html) module"]
pub struct KEY_STORE_WRITTEN_AREA_SPEC;
impl crate::RegisterSpec for KEY_STORE_WRITTEN_AREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_store_written_area::R](R) reader structure"]
impl crate::Readable for KEY_STORE_WRITTEN_AREA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_store_written_area::W](W) writer structure"]
impl crate::Writable for KEY_STORE_WRITTEN_AREA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_STORE_WRITTEN_AREA to value 0"]
impl crate::Resettable for KEY_STORE_WRITTEN_AREA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
