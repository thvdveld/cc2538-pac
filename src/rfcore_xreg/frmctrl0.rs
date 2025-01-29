#[doc = "Register `FRMCTRL0` reader"]
pub type R = crate::R<Frmctrl0Spec>;
#[doc = "Register `FRMCTRL0` writer"]
pub type W = crate::W<Frmctrl0Spec>;
#[doc = "Field `TX_MODE` reader - Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
pub type TxModeR = crate::FieldReader;
#[doc = "Field `TX_MODE` writer - Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
pub type TxModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_MODE` reader - Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
pub type RxModeR = crate::FieldReader;
#[doc = "Field `RX_MODE` writer - Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
pub type RxModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENERGY_SCAN` reader - Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
pub type EnergyScanR = crate::BitReader;
#[doc = "Field `ENERGY_SCAN` writer - Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
pub type EnergyScanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOACK` reader - Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
pub type AutoackR = crate::BitReader;
#[doc = "Field `AUTOACK` writer - Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
pub type AutoackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOCRC` reader - In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
pub type AutocrcR = crate::BitReader;
#[doc = "Field `AUTOCRC` writer - In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
pub type AutocrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APPEND_DATA_MODE` reader - When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
pub type AppendDataModeR = crate::BitReader;
#[doc = "Field `APPEND_DATA_MODE` writer - When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
pub type AppendDataModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
    #[inline(always)]
    pub fn tx_mode(&self) -> TxModeR {
        TxModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
    #[inline(always)]
    pub fn rx_mode(&self) -> RxModeR {
        RxModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
    #[inline(always)]
    pub fn energy_scan(&self) -> EnergyScanR {
        EnergyScanR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
    #[inline(always)]
    pub fn autoack(&self) -> AutoackR {
        AutoackR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
    #[inline(always)]
    pub fn autocrc(&self) -> AutocrcR {
        AutocrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
    #[inline(always)]
    pub fn append_data_mode(&self) -> AppendDataModeR {
        AppendDataModeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set test modes for TX. 00: Normal operation, transmit TX FIFO 01: Reserved, should not be used 10: TX FIFO looping ignore underflow in TX FIFO and read cyclic; infinite transmission 11: Send random data from CRC; infinite transmission"]
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TxModeW<Frmctrl0Spec> {
        TxModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Set RX modes. 00: Normal operation, use RX FIFO 01: Receive serial mode, output received data on to IOC; infinite RX 10: RX FIFO looping ignore overflow in RX FIFO; infinite reception 11: Same as normal operation except that symbol search is disabled. Can be used for RSSI or CCA measurements when finding symbol is not desired."]
    #[inline(always)]
    pub fn rx_mode(&mut self) -> RxModeW<Frmctrl0Spec> {
        RxModeW::new(self, 2)
    }
    #[doc = "Bit 4 - Defines whether the RSSI register contains the most-recent signal strength or the peak signal strength since the energy scan was enabled. 0: Most-recent signal strength 1: Peak signal strength"]
    #[inline(always)]
    pub fn energy_scan(&mut self) -> EnergyScanW<Frmctrl0Spec> {
        EnergyScanW::new(self, 4)
    }
    #[doc = "Bit 5 - Defines whether the radio automatically transmits acknowledge frames or not. When autoack is enabled, all frames that are accepted by address filtering, have the acknowledge request flag set, and have a valid CRC are automatically acknowledged 12 symbol periods after being received. 0: Autoack disabled 1: Autoack enabled"]
    #[inline(always)]
    pub fn autoack(&mut self) -> AutoackW<Frmctrl0Spec> {
        AutoackW::new(self, 5)
    }
    #[doc = "Bit 6 - In TX 1: A CRC-16 (ITU-T) is generated in hardware and appended to the transmitted frame. There is no need to write the last 2 bytes to TXBUF. 0: No CRC-16 is appended to the frame. The last 2 bytes of the frame must be generated manually and written to TXBUF (if not, TX_UNDERFLOW occurs). In RX 1: The CRC-16 is checked in hardware, and replaced in the RXFIFO by a 16-bit status word which contains a CRC OK bit. The status word is controllable through APPEND_DATA_MODE. 0: The last 2 bytes of the frame (CRC-16 field) are stored in the RX FIFO. The CRC (if any) must be done manually. This setting does not influence acknowledgment transmission (including AUTOACK)."]
    #[inline(always)]
    pub fn autocrc(&mut self) -> AutocrcW<Frmctrl0Spec> {
        AutocrcW::new(self, 6)
    }
    #[doc = "Bit 7 - When AUTOCRC = 0: Don't care When AUTOCRC = 1: 0: RSSI + The CRC_OK bit and the 7-bit correlation value are appended at the end of each received frame 1: RSSI + The CRC_OK bit and the 7-bit SRCRESINDEX are appended at the end of each received frame."]
    #[inline(always)]
    pub fn append_data_mode(&mut self) -> AppendDataModeW<Frmctrl0Spec> {
        AppendDataModeW::new(self, 7)
    }
}
#[doc = "Frame handling\n\nYou can [`read`](crate::Reg::read) this register and get [`frmctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frmctrl0Spec;
impl crate::RegisterSpec for Frmctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmctrl0::R`](R) reader structure"]
impl crate::Readable for Frmctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`frmctrl0::W`](W) writer structure"]
impl crate::Writable for Frmctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRMCTRL0 to value 0"]
impl crate::Resettable for Frmctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
