#[doc = "Register `NINEBITAMASK` reader"]
pub struct R(crate::R<NINEBITAMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NINEBITAMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NINEBITAMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NINEBITAMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NINEBITAMASK` writer"]
pub struct W(crate::W<NINEBITAMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NINEBITAMASK_SPEC>;
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
impl From<crate::W<NINEBITAMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NINEBITAMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RANGE` reader - Self address range for 9-bit mode Writing to the RANGE field does not have any effect; reading it reflects the ANDed output of the ADDR field in the UART9BITADDR register and the MASK field."]
pub struct RANGE_R(crate::FieldReader<u8, u8>);
impl RANGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` reader - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
pub struct MASK_R(crate::FieldReader<u8, u8>);
impl MASK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Self address range for 9-bit mode Writing to the RANGE field does not have any effect; reading it reflects the ANDed output of the ADDR field in the UART9BITADDR register and the MASK field."]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Self Address Mask for 9-Bit Mode This field contains the address mask that creates a range of addresses that should be matched."]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 9-bit self address mask The NINEBITAMASK register is used to enable the address mask for 9-bit mode. The lower address bits are masked to create a range of address to be matched with the received address byte.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ninebitamask](index.html) module"]
pub struct NINEBITAMASK_SPEC;
impl crate::RegisterSpec for NINEBITAMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ninebitamask::R](R) reader structure"]
impl crate::Readable for NINEBITAMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ninebitamask::W](W) writer structure"]
impl crate::Writable for NINEBITAMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NINEBITAMASK to value 0"]
impl crate::Resettable for NINEBITAMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
