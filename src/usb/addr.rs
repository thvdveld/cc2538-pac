#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBADDR` reader - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
pub type USBADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBADDR` writer - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
pub type USBADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADDR_SPEC, u8, u8, 7, O>;
#[doc = "Field `UPDATE` reader - This bit is set by hardware when writing to this register, and is cleared by hardware when the new address becomes effective."]
pub type UPDATE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:6 - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
    #[inline(always)]
    pub fn usbaddr(&self) -> USBADDR_R {
        USBADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - This bit is set by hardware when writing to this register, and is cleared by hardware when the new address becomes effective."]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Device address. The address shall be updated upon successful completion of the status stage of the SET_ADDRESS request."]
    #[inline(always)]
    #[must_use]
    pub fn usbaddr(&mut self) -> USBADDR_W<0> {
        USBADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Function address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
