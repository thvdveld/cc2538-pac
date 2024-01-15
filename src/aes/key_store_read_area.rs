#[doc = "Register `KEY_STORE_READ_AREA` reader"]
pub type R = crate::R<KEY_STORE_READ_AREA_SPEC>;
#[doc = "Register `KEY_STORE_READ_AREA` writer"]
pub type W = crate::W<KEY_STORE_READ_AREA_SPEC>;
#[doc = "Field `RAM_AREA` reader - Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
pub type RAM_AREA_R = crate::FieldReader;
#[doc = "Field `RAM_AREA` writer - Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
pub type RAM_AREA_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BUSY` reader - Key store operation busy status flag (read only): 0: Operation is complete. 1: Operation is not completed and the key store is busy."]
pub type BUSY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&self) -> RAM_AREA_R {
        RAM_AREA_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Key store operation busy status flag (read only): 0: Operation is complete. 1: Operation is not completed and the key store is busy."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    #[must_use]
    pub fn ram_area(&mut self) -> RAM_AREA_W<KEY_STORE_READ_AREA_SPEC> {
        RAM_AREA_W::new(self, 0)
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
#[doc = "Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_store_read_area::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_store_read_area::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_STORE_READ_AREA_SPEC;
impl crate::RegisterSpec for KEY_STORE_READ_AREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_store_read_area::R`](R) reader structure"]
impl crate::Readable for KEY_STORE_READ_AREA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_store_read_area::W`](W) writer structure"]
impl crate::Writable for KEY_STORE_READ_AREA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_STORE_READ_AREA to value 0"]
impl crate::Resettable for KEY_STORE_READ_AREA_SPEC {
    const RESET_VALUE: u32 = 0;
}
