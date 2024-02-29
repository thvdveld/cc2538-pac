#[doc = "Register `FRMFILT0` reader"]
pub type R = crate::R<Frmfilt0Spec>;
#[doc = "Register `FRMFILT0` writer"]
pub type W = crate::W<Frmfilt0Spec>;
#[doc = "Field `FRAME_FILTER_EN` reader - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
pub type FrameFilterEnR = crate::BitReader;
#[doc = "Field `FRAME_FILTER_EN` writer - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
pub type FrameFilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAN_COORDINATOR` reader - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
pub type PanCoordinatorR = crate::BitReader;
#[doc = "Field `PAN_COORDINATOR` writer - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
pub type PanCoordinatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAX_FRAME_VERSION` reader - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
pub type MaxFrameVersionR = crate::FieldReader;
#[doc = "Field `MAX_FRAME_VERSION` writer - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
pub type MaxFrameVersionW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
    #[inline(always)]
    pub fn frame_filter_en(&self) -> FrameFilterEnR {
        FrameFilterEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
    #[inline(always)]
    pub fn pan_coordinator(&self) -> PanCoordinatorR {
        PanCoordinatorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    pub fn max_frame_version(&self) -> MaxFrameVersionR {
        MaxFrameVersionR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables frame filtering When this bit is set, the radio performs frame filtering as specified in section 7.5.6.2 of IEEE 802.15.4(b), third filtering level. FRMFILT0\\[6:1\\]
and FRMFILT1\\[7:1\\], together with the local address information, define the behavior of the filtering algorithm. 0: Frame filtering off. (FRMFILT0\\[6:1\\], FRMFILT1\\[7:1\\]
and SRCMATCH\\[2:0\\]
are don't care.) 1: Frame filtering on."]
    #[inline(always)]
    #[must_use]
    pub fn frame_filter_en(&mut self) -> FrameFilterEnW<Frmfilt0Spec> {
        FrameFilterEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Should be set high when the device is a PAN coordinator, to accept frames with no destination address (as specified in Section 7.5.6.2 in IEEE 802.15.4) 0: Device is not a PAN coordinator 1: Device is a PAN coordinator"]
    #[inline(always)]
    #[must_use]
    pub fn pan_coordinator(&mut self) -> PanCoordinatorW<Frmfilt0Spec> {
        PanCoordinatorW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Used for filtering on the frame version field of the frame control field (FCF) If FCF\\[13:12\\]
(the frame version subfield) is higher than MAX_FRAME_VERSION\\[1:0\\]
and frame filtering is enabled, the frame is rejected."]
    #[inline(always)]
    #[must_use]
    pub fn max_frame_version(&mut self) -> MaxFrameVersionW<Frmfilt0Spec> {
        MaxFrameVersionW::new(self, 2)
    }
}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmfilt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmfilt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frmfilt0Spec;
impl crate::RegisterSpec for Frmfilt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmfilt0::R`](R) reader structure"]
impl crate::Readable for Frmfilt0Spec {}
#[doc = "`write(|w| ..)` method takes [`frmfilt0::W`](W) writer structure"]
impl crate::Writable for Frmfilt0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRMFILT0 to value 0"]
impl crate::Resettable for Frmfilt0Spec {
    const RESET_VALUE: u32 = 0;
}
