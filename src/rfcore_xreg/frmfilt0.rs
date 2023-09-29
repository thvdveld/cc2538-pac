#[doc = "Register `FRMFILT0` reader"]
pub type R = crate::R<FRMFILT0_SPEC>;
#[doc = "Register `FRMFILT0` writer"]
pub type W = crate::W<FRMFILT0_SPEC>;
#[doc = "Field `FRAME_FILTER_EN` reader - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
pub type FRAME_FILTER_EN_R = crate::BitReader;
#[doc = "Field `FRAME_FILTER_EN` writer - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
pub type FRAME_FILTER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PAN_COORDINATOR` reader - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
pub type PAN_COORDINATOR_R = crate::BitReader;
#[doc = "Field `PAN_COORDINATOR` writer - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
pub type PAN_COORDINATOR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MAX_FRAME_VERSION` reader - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
pub type MAX_FRAME_VERSION_R = crate::FieldReader;
#[doc = "Field `MAX_FRAME_VERSION` writer - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
pub type MAX_FRAME_VERSION_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
    #[must_use]
    pub fn frame_filter_en(&mut self) -> FRAME_FILTER_EN_W<FRMFILT0_SPEC, 0> {
        FRAME_FILTER_EN_W::new(self)
    }
    #[doc = "Bit 1 - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
    #[inline(always)]
    #[must_use]
    pub fn pan_coordinator(&mut self) -> PAN_COORDINATOR_W<FRMFILT0_SPEC, 1> {
        PAN_COORDINATOR_W::new(self)
    }
    #[doc = "Bits 2:3 - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    #[must_use]
    pub fn max_frame_version(&mut self) -> MAX_FRAME_VERSION_W<FRMFILT0_SPEC, 2> {
        MAX_FRAME_VERSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmfilt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmfilt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRMFILT0_SPEC;
impl crate::RegisterSpec for FRMFILT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmfilt0::R`](R) reader structure"]
impl crate::Readable for FRMFILT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frmfilt0::W`](W) writer structure"]
impl crate::Writable for FRMFILT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRMFILT0 to value 0"]
impl crate::Resettable for FRMFILT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
