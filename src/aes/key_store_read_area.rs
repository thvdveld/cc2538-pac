#[doc = "Register `KEY_STORE_READ_AREA` reader"]
pub struct R(crate::R<KEY_STORE_READ_AREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_STORE_READ_AREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_STORE_READ_AREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_STORE_READ_AREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_STORE_READ_AREA` writer"]
pub struct W(crate::W<KEY_STORE_READ_AREA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_STORE_READ_AREA_SPEC>;
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
impl From<crate::W<KEY_STORE_READ_AREA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_STORE_READ_AREA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSY` reader - Key store operation busy status flag (read only): 0: Operation is complete. 1: Operation is not completed and the key store is busy."]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `RAM_AREA` reader - Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
pub type RAM_AREA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAM_AREA` writer - Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
pub type RAM_AREA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEY_STORE_READ_AREA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 31 - Key store operation busy status flag (read only): 0: Operation is complete. 1: Operation is not completed and the key store is busy."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bits 0:3 - Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&self) -> RAM_AREA_R {
        RAM_AREA_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects the area of the key store RAM from where the key needs to be read that will be writen to the AES engine RAM_AREA: 0000: RAM_AREA0 0001: RAM_AREA1 0010: RAM_AREA2 0011: RAM_AREA3 0100: RAM_AREA4 0101: RAM_AREA5 0110: RAM_AREA6 0111: RAM_AREA7 1000: no RAM area selected 1001-1111: Reserved RAM areas RAM_AREA0, RAM_AREA2, RAM_AREA4 and RAM_AREA6 are the only valid read areas for 192 and 256 bits key sizes. Only RAM areas that contain valid written keys can be selected."]
    #[inline(always)]
    pub fn ram_area(&mut self) -> RAM_AREA_W<0> {
        RAM_AREA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_store_read_area](index.html) module"]
pub struct KEY_STORE_READ_AREA_SPEC;
impl crate::RegisterSpec for KEY_STORE_READ_AREA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_store_read_area::R](R) reader structure"]
impl crate::Readable for KEY_STORE_READ_AREA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_store_read_area::W](W) writer structure"]
impl crate::Writable for KEY_STORE_READ_AREA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY_STORE_READ_AREA to value 0"]
impl crate::Resettable for KEY_STORE_READ_AREA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
