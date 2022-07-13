#[doc = "Register `KEY_STORE_WRITE_AREA` reader"]
pub struct R(crate::R<KEY_STORE_WRITE_AREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_STORE_WRITE_AREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_STORE_WRITE_AREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_STORE_WRITE_AREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_STORE_WRITE_AREA` writer"]
pub struct W(crate::W<KEY_STORE_WRITE_AREA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_STORE_WRITE_AREA_SPEC>;
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
impl From<crate::W<KEY_STORE_WRITE_AREA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_STORE_WRITE_AREA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAM_AREA7` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA7_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA7` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITE_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA6` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA6_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA6` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITE_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA5` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA5_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA5` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITE_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA4` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA4_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA4` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITE_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA3` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA3_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA3` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITE_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA2` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA2_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA2` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITE_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA1` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA1_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA1` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITE_AREA_SPEC, bool, O>;
#[doc = "Field `RAM_AREA0` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA0_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA0` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RAM_AREA0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, KEY_STORE_WRITE_AREA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 7 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area7(&self) -> RAM_AREA7_R {
        RAM_AREA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area6(&self) -> RAM_AREA6_R {
        RAM_AREA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area5(&self) -> RAM_AREA5_R {
        RAM_AREA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area4(&self) -> RAM_AREA4_R {
        RAM_AREA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area3(&self) -> RAM_AREA3_R {
        RAM_AREA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area2(&self) -> RAM_AREA2_R {
        RAM_AREA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area1(&self) -> RAM_AREA1_R {
        RAM_AREA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area0(&self) -> RAM_AREA0_R {
        RAM_AREA0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area7(&mut self) -> RAM_AREA7_W<7> {
        RAM_AREA7_W::new(self)
    }
    #[doc = "Bit 6 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area6(&mut self) -> RAM_AREA6_W<6> {
        RAM_AREA6_W::new(self)
    }
    #[doc = "Bit 5 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area5(&mut self) -> RAM_AREA5_W<5> {
        RAM_AREA5_W::new(self)
    }
    #[doc = "Bit 4 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area4(&mut self) -> RAM_AREA4_W<4> {
        RAM_AREA4_W::new(self)
    }
    #[doc = "Bit 3 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area3(&mut self) -> RAM_AREA3_W<3> {
        RAM_AREA3_W::new(self)
    }
    #[doc = "Bit 2 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area2(&mut self) -> RAM_AREA2_W<2> {
        RAM_AREA2_W::new(self)
    }
    #[doc = "Bit 1 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area1(&mut self) -> RAM_AREA1_W<1> {
        RAM_AREA1_W::new(self)
    }
    #[doc = "Bit 0 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area0(&mut self) -> RAM_AREA0_W<0> {
        RAM_AREA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_store_write_area](index.html) module"]
pub struct KEY_STORE_WRITE_AREA_SPEC;
impl crate::RegisterSpec for KEY_STORE_WRITE_AREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_store_write_area::R](R) reader structure"]
impl crate::Readable for KEY_STORE_WRITE_AREA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_store_write_area::W](W) writer structure"]
impl crate::Writable for KEY_STORE_WRITE_AREA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY_STORE_WRITE_AREA to value 0"]
impl crate::Resettable for KEY_STORE_WRITE_AREA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
