#[doc = "Register `RFC_OBS_CTRL0` reader"]
pub struct R(crate::R<RFC_OBS_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFC_OBS_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFC_OBS_CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFC_OBS_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RFC_OBS_CTRL0` writer"]
pub struct W(crate::W<RFC_OBS_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFC_OBS_CTRL0_SPEC>;
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
impl From<crate::W<RFC_OBS_CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFC_OBS_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC_OBS_MUX0` reader - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[0\\]. 00 0000: 0 - Constant value 00 0001: 1 - Constant value 00 1000: rfc_sniff_data - Data from packet sniffer. Sample data on rising edges of sniff_clk. 00 1001: rfc_sniff_clk - 250kHz clock for packet sniffer data. 00 1100: rssi_valid - Pin is high when the RSSI value has been updated at least once since RX was started. Cleared when leaving RX. 00 1101: demod_cca - Clear channel assessment. See FSMSTAT1 register for details on how to configure the behavior of this signal. 00 1110: sampled_cca - A sampled version of the CCA bit from demodulator. The value is updated whenever a SSAMPLECCA or STXONCCA strobe is issued. 00 1111: sfd_sync - Pin is high when a SFD has been received or transmitted. Cleared when leaving RX/TX respectively. Not to be confused with the SFD exception. 01 0000: tx_active - Indicates that FFCTRL is in one of the TX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0001: rx_active - Indicates that FFCTRL is in one of the RX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0010: ffctrl_fifo - Pin is high when one or more bytes are in the RXFIFO. Low during RXFIFO overflow. 01 0011: ffctrl_fifop - Pin is high when the number of bytes in the RXFIFO exceeds the programmable threshold or at least one complete frame is in the RXFIFO. Also high during RXFIFO overflow. Not to be confused with the FIFOP exception. 01 0100: packet_done - A complete frame has been received. I.e., the number of bytes set by the frame-length field has been received. 01 0110: rfc_xor_rand_i_q - XOR between I and Q random outputs. Updated at 8 MHz. 01 0111: rfc_rand_q - Random data output from the Q channel of the receiver. Updated at 8 MHz. 01 1000: rfc_rand_i - Random data output from the I channel of the receiver. Updated at 8 MHz 01 1001: lock_status - 1 when PLL is in lock, otherwise 0 10 1000: pa_pd - Power amplifier power-down signal 10 1010: lna_pd - LNA power-down signal Others: Reserved"]
pub type RFC_OBS_MUX0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFC_OBS_MUX0` writer - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[0\\]. 00 0000: 0 - Constant value 00 0001: 1 - Constant value 00 1000: rfc_sniff_data - Data from packet sniffer. Sample data on rising edges of sniff_clk. 00 1001: rfc_sniff_clk - 250kHz clock for packet sniffer data. 00 1100: rssi_valid - Pin is high when the RSSI value has been updated at least once since RX was started. Cleared when leaving RX. 00 1101: demod_cca - Clear channel assessment. See FSMSTAT1 register for details on how to configure the behavior of this signal. 00 1110: sampled_cca - A sampled version of the CCA bit from demodulator. The value is updated whenever a SSAMPLECCA or STXONCCA strobe is issued. 00 1111: sfd_sync - Pin is high when a SFD has been received or transmitted. Cleared when leaving RX/TX respectively. Not to be confused with the SFD exception. 01 0000: tx_active - Indicates that FFCTRL is in one of the TX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0001: rx_active - Indicates that FFCTRL is in one of the RX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0010: ffctrl_fifo - Pin is high when one or more bytes are in the RXFIFO. Low during RXFIFO overflow. 01 0011: ffctrl_fifop - Pin is high when the number of bytes in the RXFIFO exceeds the programmable threshold or at least one complete frame is in the RXFIFO. Also high during RXFIFO overflow. Not to be confused with the FIFOP exception. 01 0100: packet_done - A complete frame has been received. I.e., the number of bytes set by the frame-length field has been received. 01 0110: rfc_xor_rand_i_q - XOR between I and Q random outputs. Updated at 8 MHz. 01 0111: rfc_rand_q - Random data output from the Q channel of the receiver. Updated at 8 MHz. 01 1000: rfc_rand_i - Random data output from the I channel of the receiver. Updated at 8 MHz 01 1001: lock_status - 1 when PLL is in lock, otherwise 0 10 1000: pa_pd - Power amplifier power-down signal 10 1010: lna_pd - LNA power-down signal Others: Reserved"]
pub type RFC_OBS_MUX0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RFC_OBS_CTRL0_SPEC, u8, u8, 6, O>;
#[doc = "Field `RFC_OBS_POL0` reader - The signal chosen by RFC_OBS_MUX0 is XORed with this bit."]
pub type RFC_OBS_POL0_R = crate::BitReader<bool>;
#[doc = "Field `RFC_OBS_POL0` writer - The signal chosen by RFC_OBS_MUX0 is XORed with this bit."]
pub type RFC_OBS_POL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RFC_OBS_CTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[0\\]. 00 0000: 0 - Constant value 00 0001: 1 - Constant value 00 1000: rfc_sniff_data - Data from packet sniffer. Sample data on rising edges of sniff_clk. 00 1001: rfc_sniff_clk - 250kHz clock for packet sniffer data. 00 1100: rssi_valid - Pin is high when the RSSI value has been updated at least once since RX was started. Cleared when leaving RX. 00 1101: demod_cca - Clear channel assessment. See FSMSTAT1 register for details on how to configure the behavior of this signal. 00 1110: sampled_cca - A sampled version of the CCA bit from demodulator. The value is updated whenever a SSAMPLECCA or STXONCCA strobe is issued. 00 1111: sfd_sync - Pin is high when a SFD has been received or transmitted. Cleared when leaving RX/TX respectively. Not to be confused with the SFD exception. 01 0000: tx_active - Indicates that FFCTRL is in one of the TX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0001: rx_active - Indicates that FFCTRL is in one of the RX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0010: ffctrl_fifo - Pin is high when one or more bytes are in the RXFIFO. Low during RXFIFO overflow. 01 0011: ffctrl_fifop - Pin is high when the number of bytes in the RXFIFO exceeds the programmable threshold or at least one complete frame is in the RXFIFO. Also high during RXFIFO overflow. Not to be confused with the FIFOP exception. 01 0100: packet_done - A complete frame has been received. I.e., the number of bytes set by the frame-length field has been received. 01 0110: rfc_xor_rand_i_q - XOR between I and Q random outputs. Updated at 8 MHz. 01 0111: rfc_rand_q - Random data output from the Q channel of the receiver. Updated at 8 MHz. 01 1000: rfc_rand_i - Random data output from the I channel of the receiver. Updated at 8 MHz 01 1001: lock_status - 1 when PLL is in lock, otherwise 0 10 1000: pa_pd - Power amplifier power-down signal 10 1010: lna_pd - LNA power-down signal Others: Reserved"]
    #[inline(always)]
    pub fn rfc_obs_mux0(&self) -> RFC_OBS_MUX0_R {
        RFC_OBS_MUX0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX0 is XORed with this bit."]
    #[inline(always)]
    pub fn rfc_obs_pol0(&self) -> RFC_OBS_POL0_R {
        RFC_OBS_POL0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Controls which observable signal from RF Core is to be muxed out to rfc_obs_sigs\\[0\\]. 00 0000: 0 - Constant value 00 0001: 1 - Constant value 00 1000: rfc_sniff_data - Data from packet sniffer. Sample data on rising edges of sniff_clk. 00 1001: rfc_sniff_clk - 250kHz clock for packet sniffer data. 00 1100: rssi_valid - Pin is high when the RSSI value has been updated at least once since RX was started. Cleared when leaving RX. 00 1101: demod_cca - Clear channel assessment. See FSMSTAT1 register for details on how to configure the behavior of this signal. 00 1110: sampled_cca - A sampled version of the CCA bit from demodulator. The value is updated whenever a SSAMPLECCA or STXONCCA strobe is issued. 00 1111: sfd_sync - Pin is high when a SFD has been received or transmitted. Cleared when leaving RX/TX respectively. Not to be confused with the SFD exception. 01 0000: tx_active - Indicates that FFCTRL is in one of the TX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0001: rx_active - Indicates that FFCTRL is in one of the RX states. Active-high. Note: This signal might have glitches, because it has no output flip-flop and is based on the current state register of the FFCTRL FSM. 01 0010: ffctrl_fifo - Pin is high when one or more bytes are in the RXFIFO. Low during RXFIFO overflow. 01 0011: ffctrl_fifop - Pin is high when the number of bytes in the RXFIFO exceeds the programmable threshold or at least one complete frame is in the RXFIFO. Also high during RXFIFO overflow. Not to be confused with the FIFOP exception. 01 0100: packet_done - A complete frame has been received. I.e., the number of bytes set by the frame-length field has been received. 01 0110: rfc_xor_rand_i_q - XOR between I and Q random outputs. Updated at 8 MHz. 01 0111: rfc_rand_q - Random data output from the Q channel of the receiver. Updated at 8 MHz. 01 1000: rfc_rand_i - Random data output from the I channel of the receiver. Updated at 8 MHz 01 1001: lock_status - 1 when PLL is in lock, otherwise 0 10 1000: pa_pd - Power amplifier power-down signal 10 1010: lna_pd - LNA power-down signal Others: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rfc_obs_mux0(&mut self) -> RFC_OBS_MUX0_W<0> {
        RFC_OBS_MUX0_W::new(self)
    }
    #[doc = "Bit 6 - The signal chosen by RFC_OBS_MUX0 is XORed with this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_obs_pol0(&mut self) -> RFC_OBS_POL0_W<6> {
        RFC_OBS_POL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RF observation mux control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfc_obs_ctrl0](index.html) module"]
pub struct RFC_OBS_CTRL0_SPEC;
impl crate::RegisterSpec for RFC_OBS_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfc_obs_ctrl0::R](R) reader structure"]
impl crate::Readable for RFC_OBS_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfc_obs_ctrl0::W](W) writer structure"]
impl crate::Writable for RFC_OBS_CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RFC_OBS_CTRL0 to value 0"]
impl crate::Resettable for RFC_OBS_CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
