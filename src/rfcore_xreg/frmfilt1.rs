#[doc = "Register `FRMFILT1` reader"]
pub type R = crate::R<FRMFILT1_SPEC>;
#[doc = "Register `FRMFILT1` writer"]
pub type W = crate::W<FRMFILT1_SPEC>;
#[doc = "Field `MODIFY_FT_FILTER` reader - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
pub type MODIFY_FT_FILTER_R = crate::FieldReader;
#[doc = "Field `MODIFY_FT_FILTER` writer - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
pub type MODIFY_FT_FILTER_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ACCEPT_FT_0_BEACON` reader - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
pub type ACCEPT_FT_0_BEACON_R = crate::BitReader;
#[doc = "Field `ACCEPT_FT_0_BEACON` writer - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
pub type ACCEPT_FT_0_BEACON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCEPT_FT_1_DATA` reader - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
pub type ACCEPT_FT_1_DATA_R = crate::BitReader;
#[doc = "Field `ACCEPT_FT_1_DATA` writer - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
pub type ACCEPT_FT_1_DATA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCEPT_FT_2_ACK` reader - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
pub type ACCEPT_FT_2_ACK_R = crate::BitReader;
#[doc = "Field `ACCEPT_FT_2_ACK` writer - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
pub type ACCEPT_FT_2_ACK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ACCEPT_FT_3_MAC_CMD` reader - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
pub type ACCEPT_FT_3_MAC_CMD_R = crate::BitReader;
#[doc = "Field `ACCEPT_FT_3_MAC_CMD` writer - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
pub type ACCEPT_FT_3_MAC_CMD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 1:2 - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
    #[inline(always)]
    pub fn modify_ft_filter(&self) -> MODIFY_FT_FILTER_R {
        MODIFY_FT_FILTER_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_0_beacon(&self) -> ACCEPT_FT_0_BEACON_R {
        ACCEPT_FT_0_BEACON_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_1_data(&self) -> ACCEPT_FT_1_DATA_R {
        ACCEPT_FT_1_DATA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_2_ack(&self) -> ACCEPT_FT_2_ACK_R {
        ACCEPT_FT_2_ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_3_mac_cmd(&self) -> ACCEPT_FT_3_MAC_CMD_R {
        ACCEPT_FT_3_MAC_CMD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
    #[inline(always)]
    #[must_use]
    pub fn modify_ft_filter(&mut self) -> MODIFY_FT_FILTER_W<FRMFILT1_SPEC, 1> {
        MODIFY_FT_FILTER_W::new(self)
    }
    #[doc = "Bit 3 - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
    #[inline(always)]
    #[must_use]
    pub fn accept_ft_0_beacon(&mut self) -> ACCEPT_FT_0_BEACON_W<FRMFILT1_SPEC, 3> {
        ACCEPT_FT_0_BEACON_W::new(self)
    }
    #[doc = "Bit 4 - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
    #[inline(always)]
    #[must_use]
    pub fn accept_ft_1_data(&mut self) -> ACCEPT_FT_1_DATA_W<FRMFILT1_SPEC, 4> {
        ACCEPT_FT_1_DATA_W::new(self)
    }
    #[doc = "Bit 5 - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
    #[inline(always)]
    #[must_use]
    pub fn accept_ft_2_ack(&mut self) -> ACCEPT_FT_2_ACK_W<FRMFILT1_SPEC, 5> {
        ACCEPT_FT_2_ACK_W::new(self)
    }
    #[doc = "Bit 6 - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
    #[inline(always)]
    #[must_use]
    pub fn accept_ft_3_mac_cmd(&mut self) -> ACCEPT_FT_3_MAC_CMD_W<FRMFILT1_SPEC, 6> {
        ACCEPT_FT_3_MAC_CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmfilt1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmfilt1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRMFILT1_SPEC;
impl crate::RegisterSpec for FRMFILT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmfilt1::R`](R) reader structure"]
impl crate::Readable for FRMFILT1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frmfilt1::W`](W) writer structure"]
impl crate::Writable for FRMFILT1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRMFILT1 to value 0"]
impl crate::Resettable for FRMFILT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
