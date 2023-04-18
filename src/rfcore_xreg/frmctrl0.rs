#[doc = "Register `FRMCTRL0` reader"]
pub struct R(crate::R<FRMCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRMCTRL0` writer"]
pub struct W(crate::W<FRMCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRMCTRL0_SPEC>;
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
impl From<crate::W<FRMCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRMCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_MODE` reader - Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
pub type TX_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_MODE` writer - Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
pub type TX_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRMCTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_MODE` reader - Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
pub type RX_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_MODE` writer - Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
pub type RX_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FRMCTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENERGY_SCAN` reader - Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
pub type ENERGY_SCAN_R = crate::BitReader<bool>;
#[doc = "Field `ENERGY_SCAN` writer - Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
pub type ENERGY_SCAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMCTRL0_SPEC, bool, O>;
#[doc = "Field `AUTOACK` reader - Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
pub type AUTOACK_R = crate::BitReader<bool>;
#[doc = "Field `AUTOACK` writer - Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
pub type AUTOACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMCTRL0_SPEC, bool, O>;
#[doc = "Field `AUTOCRC` reader - In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
pub type AUTOCRC_R = crate::BitReader<bool>;
#[doc = "Field `AUTOCRC` writer - In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
pub type AUTOCRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMCTRL0_SPEC, bool, O>;
#[doc = "Field `APPEND_DATA_MODE` reader - When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
pub type APPEND_DATA_MODE_R = crate::BitReader<bool>;
#[doc = "Field `APPEND_DATA_MODE` writer - When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
pub type APPEND_DATA_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FRMCTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
    #[inline(always)]
    pub fn tx_mode(&self) -> TX_MODE_R {
        TX_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
    #[inline(always)]
    pub fn rx_mode(&self) -> RX_MODE_R {
        RX_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
    #[inline(always)]
    pub fn energy_scan(&self) -> ENERGY_SCAN_R {
        ENERGY_SCAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
    #[inline(always)]
    pub fn autoack(&self) -> AUTOACK_R {
        AUTOACK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
    #[inline(always)]
    pub fn autocrc(&self) -> AUTOCRC_R {
        AUTOCRC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
    #[inline(always)]
    pub fn append_data_mode(&self) -> APPEND_DATA_MODE_R {
        APPEND_DATA_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mode(&mut self) -> TX_MODE_W<0> {
        TX_MODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
    #[inline(always)]
    #[must_use]
    pub fn rx_mode(&mut self) -> RX_MODE_W<2> {
        RX_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
    #[inline(always)]
    #[must_use]
    pub fn energy_scan(&mut self) -> ENERGY_SCAN_W<4> {
        ENERGY_SCAN_W::new(self)
    }
    #[doc = "Bit 5 - Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
    #[inline(always)]
    #[must_use]
    pub fn autoack(&mut self) -> AUTOACK_W<5> {
        AUTOACK_W::new(self)
    }
    #[doc = "Bit 6 - In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
    #[inline(always)]
    #[must_use]
    pub fn autocrc(&mut self) -> AUTOCRC_W<6> {
        AUTOCRC_W::new(self)
    }
    #[doc = "Bit 7 - When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
    #[inline(always)]
    #[must_use]
    pub fn append_data_mode(&mut self) -> APPEND_DATA_MODE_W<7> {
        APPEND_DATA_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame handling\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmctrl0](index.html) module"]
pub struct FRMCTRL0_SPEC;
impl crate::RegisterSpec for FRMCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frmctrl0::R](R) reader structure"]
impl crate::Readable for FRMCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frmctrl0::W](W) writer structure"]
impl crate::Writable for FRMCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRMCTRL0 to value 0"]
impl crate::Resettable for FRMCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
