#[doc = "Register `KEY_STORE_WRITE_AREA` reader"]
pub type R = crate::R<KeyStoreWriteAreaSpec>;
#[doc = "Register `KEY_STORE_WRITE_AREA` writer"]
pub type W = crate::W<KeyStoreWriteAreaSpec>;
#[doc = "Field `RAM_AREA0` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea0R = crate::BitReader;
#[doc = "Field `RAM_AREA0` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_AREA1` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea1R = crate::BitReader;
#[doc = "Field `RAM_AREA1` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_AREA2` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea2R = crate::BitReader;
#[doc = "Field `RAM_AREA2` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_AREA3` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea3R = crate::BitReader;
#[doc = "Field `RAM_AREA3` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_AREA4` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea4R = crate::BitReader;
#[doc = "Field `RAM_AREA4` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_AREA5` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea5R = crate::BitReader;
#[doc = "Field `RAM_AREA5` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_AREA6` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea6R = crate::BitReader;
#[doc = "Field `RAM_AREA6` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_AREA7` reader - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea7R = crate::BitReader;
#[doc = "Field `RAM_AREA7` writer - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
pub type RamArea7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area0(&self) -> RamArea0R {
        RamArea0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area1(&self) -> RamArea1R {
        RamArea1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area2(&self) -> RamArea2R {
        RamArea2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area3(&self) -> RamArea3R {
        RamArea3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area4(&self) -> RamArea4R {
        RamArea4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area5(&self) -> RamArea5R {
        RamArea5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area6(&self) -> RamArea6R {
        RamArea6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area7(&self) -> RamArea7R {
        RamArea7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA0 is not selected to be written. 1: RAM_AREA0 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area0(&mut self) -> RamArea0W<KeyStoreWriteAreaSpec> {
        RamArea0W::new(self, 0)
    }
    #[doc = "Bit 1 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA1 is not selected to be written. 1: RAM_AREA1 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area1(&mut self) -> RamArea1W<KeyStoreWriteAreaSpec> {
        RamArea1W::new(self, 1)
    }
    #[doc = "Bit 2 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA2 is not selected to be written. 1: RAM_AREA2 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area2(&mut self) -> RamArea2W<KeyStoreWriteAreaSpec> {
        RamArea2W::new(self, 2)
    }
    #[doc = "Bit 3 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA3 is not selected to be written. 1: RAM_AREA3 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area3(&mut self) -> RamArea3W<KeyStoreWriteAreaSpec> {
        RamArea3W::new(self, 3)
    }
    #[doc = "Bit 4 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA4 is not selected to be written. 1: RAM_AREA4 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area4(&mut self) -> RamArea4W<KeyStoreWriteAreaSpec> {
        RamArea4W::new(self, 4)
    }
    #[doc = "Bit 5 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA5 is not selected to be written. 1: RAM_AREA5 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area5(&mut self) -> RamArea5W<KeyStoreWriteAreaSpec> {
        RamArea5W::new(self, 5)
    }
    #[doc = "Bit 6 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA6 is not selected to be written. 1: RAM_AREA6 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area6(&mut self) -> RamArea6W<KeyStoreWriteAreaSpec> {
        RamArea6W::new(self, 6)
    }
    #[doc = "Bit 7 - Each RAM_AREAx represents an area of 128 bits. Select the key store RAM area(s) where the key(s) needs to be written 0: RAM_AREA7 is not selected to be written. 1: RAM_AREA7 is selected to be written. Writing to multiple RAM locations is possible only when the selected RAM areas are sequential. Keys that require more than one RAM locations (key size is 192 or 256 bits), must start at one of the following areas: RAM_AREA0, RAM_AREA2, RAM_AREA4, or RAM_AREA6."]
    #[inline(always)]
    pub fn ram_area7(&mut self) -> RamArea7W<KeyStoreWriteAreaSpec> {
        RamArea7W::new(self, 7)
    }
}
#[doc = "Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`key_store_write_area::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_store_write_area::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyStoreWriteAreaSpec;
impl crate::RegisterSpec for KeyStoreWriteAreaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_store_write_area::R`](R) reader structure"]
impl crate::Readable for KeyStoreWriteAreaSpec {}
#[doc = "`write(|w| ..)` method takes [`key_store_write_area::W`](W) writer structure"]
impl crate::Writable for KeyStoreWriteAreaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_STORE_WRITE_AREA to value 0"]
impl crate::Resettable for KeyStoreWriteAreaSpec {
    const RESET_VALUE: u32 = 0;
}
