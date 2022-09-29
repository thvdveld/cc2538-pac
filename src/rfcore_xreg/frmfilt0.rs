#[doc = "Register `FRMFILT0` reader"]
pub struct R(crate::R<FRMFILT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMFILT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMFILT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMFILT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRMFILT0` writer"]
pub struct W(crate::W<FRMFILT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRMFILT0_SPEC>;
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
impl From<crate::W<FRMFILT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRMFILT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAME_FILTER_EN` reader - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
pub type FRAME_FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `FRAME_FILTER_EN` writer - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
pub type FRAME_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMFILT0_SPEC, bool, O>;
#[doc = "Field `PAN_COORDINATOR` reader - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
pub type PAN_COORDINATOR_R = crate::BitReader<bool>;
#[doc = "Field `PAN_COORDINATOR` writer - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
pub type PAN_COORDINATOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMFILT0_SPEC, bool, O>;
#[doc = "Field `MAX_FRAME_VERSION` reader - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
pub type MAX_FRAME_VERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_FRAME_VERSION` writer - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
pub type MAX_FRAME_VERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRMFILT0_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
    #[inline(always)]
    pub fn frame_filter_en(&self) -> FRAME_FILTER_EN_R {
        FRAME_FILTER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
    #[inline(always)]
    pub fn pan_coordinator(&self) -> PAN_COORDINATOR_R {
        PAN_COORDINATOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    pub fn max_frame_version(&self) -> MAX_FRAME_VERSION_R {
        MAX_FRAME_VERSION_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
    #[inline(always)]
    pub fn frame_filter_en(&mut self) -> FRAME_FILTER_EN_W<0> {
        FRAME_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 1 - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
    #[inline(always)]
    pub fn pan_coordinator(&mut self) -> PAN_COORDINATOR_W<1> {
        PAN_COORDINATOR_W::new(self)
    }
    #[doc = "Bits 2:3 - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    pub fn max_frame_version(&mut self) -> MAX_FRAME_VERSION_W<2> {
        MAX_FRAME_VERSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmfilt0](index.html) module"]
pub struct FRMFILT0_SPEC;
impl crate::RegisterSpec for FRMFILT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frmfilt0::R](R) reader structure"]
impl crate::Readable for FRMFILT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frmfilt0::W](W) writer structure"]
impl crate::Writable for FRMFILT0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FRMFILT0 to value 0"]
impl crate::Resettable for FRMFILT0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
