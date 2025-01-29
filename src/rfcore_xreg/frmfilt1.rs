#[doc = "Register `FRMFILT1` reader"]
pub type R = crate::R<Frmfilt1Spec>;
#[doc = "Register `FRMFILT1` writer"]
pub type W = crate::W<Frmfilt1Spec>;
#[doc = "Field `MODIFY_FT_FILTER` reader - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
pub type ModifyFtFilterR = crate::FieldReader;
#[doc = "Field `MODIFY_FT_FILTER` writer - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
pub type ModifyFtFilterW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACCEPT_FT_0_BEACON` reader - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
pub type AcceptFt0BeaconR = crate::BitReader;
#[doc = "Field `ACCEPT_FT_0_BEACON` writer - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
pub type AcceptFt0BeaconW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCEPT_FT_1_DATA` reader - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
pub type AcceptFt1DataR = crate::BitReader;
#[doc = "Field `ACCEPT_FT_1_DATA` writer - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
pub type AcceptFt1DataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCEPT_FT_2_ACK` reader - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
pub type AcceptFt2AckR = crate::BitReader;
#[doc = "Field `ACCEPT_FT_2_ACK` writer - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
pub type AcceptFt2AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCEPT_FT_3_MAC_CMD` reader - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
pub type AcceptFt3MacCmdR = crate::BitReader;
#[doc = "Field `ACCEPT_FT_3_MAC_CMD` writer - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
pub type AcceptFt3MacCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:2 - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
    #[inline(always)]
    pub fn modify_ft_filter(&self) -> ModifyFtFilterR {
        ModifyFtFilterR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_0_beacon(&self) -> AcceptFt0BeaconR {
        AcceptFt0BeaconR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_1_data(&self) -> AcceptFt1DataR {
        AcceptFt1DataR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_2_ack(&self) -> AcceptFt2AckR {
        AcceptFt2AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_3_mac_cmd(&self) -> AcceptFt3MacCmdR {
        AcceptFt3MacCmdR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - These bits are used to modify the frame type field of a received frame before frame type filtering is performed. The modification does not influence the frame that is written to the RX FIFO. 00: Leave the frame type as it is. 01: Invert MSB of the frame type. 10: Set MSB of the frame type to 0. 11: Set MSB of the frame type to 1."]
    #[inline(always)]
    pub fn modify_ft_filter(&mut self) -> ModifyFtFilterW<Frmfilt1Spec> {
        ModifyFtFilterW::new(self, 1)
    }
    #[doc = "Bit 3 - Defines whether beacon frames are accepted or not. Beacon frames have frame type = 000. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_0_beacon(&mut self) -> AcceptFt0BeaconW<Frmfilt1Spec> {
        AcceptFt0BeaconW::new(self, 3)
    }
    #[doc = "Bit 4 - Defines whether data frames are accepted or not. Data frames have frame type = 001. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_1_data(&mut self) -> AcceptFt1DataW<Frmfilt1Spec> {
        AcceptFt1DataW::new(self, 4)
    }
    #[doc = "Bit 5 - Defines whether acknowledgment frames are accepted or not. Acknowledgement frames have frame type = 010. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_2_ack(&mut self) -> AcceptFt2AckW<Frmfilt1Spec> {
        AcceptFt2AckW::new(self, 5)
    }
    #[doc = "Bit 6 - Defines whether MAC command frames are accepted or not. MAC command frames have frame type = 011. 0: Reject 1: Accept"]
    #[inline(always)]
    pub fn accept_ft_3_mac_cmd(&mut self) -> AcceptFt3MacCmdW<Frmfilt1Spec> {
        AcceptFt3MacCmdW::new(self, 6)
    }
}
#[doc = "The frame filtering function rejects unintended frames as specified by IEEE 802.15.4, section 7.5.6.2, third filtering level. In addition, it provides filtering on: - The eight different frame types (see the FRMFILT1 register) - The reserved bits in the frame control field (FCF) The function is controlled by: - The FRMFILT0 and FRMFILT1 registers - The PAN_ID, SHORT_ADDR, and EXT_ADDR values in RAM\n\nYou can [`read`](crate::Reg::read) this register and get [`frmfilt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmfilt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frmfilt1Spec;
impl crate::RegisterSpec for Frmfilt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmfilt1::R`](R) reader structure"]
impl crate::Readable for Frmfilt1Spec {}
#[doc = "`write(|w| ..)` method takes [`frmfilt1::W`](W) writer structure"]
impl crate::Writable for Frmfilt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRMFILT1 to value 0"]
impl crate::Resettable for Frmfilt1Spec {
    const RESET_VALUE: u32 = 0;
}
