#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmac_ch0_ctrl: crate::Reg<dmac_ch0_ctrl::DMAC_CH0_CTRL_SPEC>,
    #[doc = "0x04 - Channel external address"]
    pub dmac_ch0_extaddr: crate::Reg<dmac_ch0_extaddr::DMAC_CH0_EXTADDR_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Channel DMA length"]
    pub dmac_ch0_dmalength: crate::Reg<dmac_ch0_dmalength::DMAC_CH0_DMALENGTH_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x18 - DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
    pub dmac_status: crate::Reg<dmac_status::DMAC_STATUS_SPEC>,
    #[doc = "0x1c - DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers."]
    pub dmac_swres: crate::Reg<dmac_swres::DMAC_SWRES_SPEC>,
    #[doc = "0x20 - Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
    pub dmac_ch1_ctrl: crate::Reg<dmac_ch1_ctrl::DMAC_CH1_CTRL_SPEC>,
    #[doc = "0x24 - Channel external address"]
    pub dmac_ch1_extaddr: crate::Reg<dmac_ch1_extaddr::DMAC_CH1_EXTADDR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - Channel DMA length"]
    pub dmac_ch1_dmalength: crate::Reg<dmac_ch1_dmalength::DMAC_CH1_DMALENGTH_SPEC>,
    _reserved8: [u8; 0x48],
    #[doc = "0x78 - DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
    pub dmac_mst_runparams: crate::Reg<dmac_mst_runparams::DMAC_MST_RUNPARAMS_SPEC>,
    #[doc = "0x7c - DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register."]
    pub dmac_persr: crate::Reg<dmac_persr::DMAC_PERSR_SPEC>,
    _reserved10: [u8; 0x78],
    #[doc = "0xf8 - DMAC options register These registers contain information regarding the different options configured in this DMAC."]
    pub dmac_options: crate::Reg<dmac_options::DMAC_OPTIONS_SPEC>,
    #[doc = "0xfc - DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
    pub dmac_version: crate::Reg<dmac_version::DMAC_VERSION_SPEC>,
    _reserved12: [u8; 0x0300],
    #[doc = "0x400 - Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
    pub key_store_write_area: crate::Reg<key_store_write_area::KEY_STORE_WRITE_AREA_SPEC>,
    #[doc = "0x404 - Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
    pub key_store_written_area: crate::Reg<key_store_written_area::KEY_STORE_WRITTEN_AREA_SPEC>,
    #[doc = "0x408 - Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
    pub key_store_size: crate::Reg<key_store_size::KEY_STORE_SIZE_SPEC>,
    #[doc = "0x40c - Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
    pub key_store_read_area: crate::Reg<key_store_read_area::KEY_STORE_READ_AREA_SPEC>,
    _reserved16: [u8; 0xf0],
    #[doc = "0x500 - AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key2_0: crate::Reg<aes_key2_0::AES_KEY2_0_SPEC>,
    #[doc = "0x504 - AES_KEY2_1 / AES_GHASH_H_IN_1 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key2_1: crate::Reg<aes_key2_1::AES_KEY2_1_SPEC>,
    #[doc = "0x508 - AES_KEY2_2 / AES_GHASH_H_IN_2 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key2_2: crate::Reg<aes_key2_2::AES_KEY2_2_SPEC>,
    #[doc = "0x50c - AES_KEY2_3 / AES_GHASH_H_IN_3 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key2_3: crate::Reg<aes_key2_3::AES_KEY2_3_SPEC>,
    #[doc = "0x510 - AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key3_0: crate::Reg<aes_key3_0::AES_KEY3_0_SPEC>,
    #[doc = "0x514 - AES_KEY3_1 / AES_KEY2_5 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key3_1: crate::Reg<aes_key3_1::AES_KEY3_1_SPEC>,
    #[doc = "0x518 - AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key3_2: crate::Reg<aes_key3_2::AES_KEY3_2_SPEC>,
    #[doc = "0x51c - AES_KEY3_3 / AES_KEY2_7 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
    pub aes_key3_3: crate::Reg<aes_key3_3::AES_KEY3_3_SPEC>,
    _reserved24: [u8; 0x20],
    #[doc = "0x540 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aes_iv_0: crate::Reg<aes_iv_0::AES_IV_0_SPEC>,
    #[doc = "0x544 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aes_iv_1: crate::Reg<aes_iv_1::AES_IV_1_SPEC>,
    #[doc = "0x548 - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aes_iv_2: crate::Reg<aes_iv_2::AES_IV_2_SPEC>,
    #[doc = "0x54c - AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
    pub aes_iv_3: crate::Reg<aes_iv_3::AES_IV_3_SPEC>,
    #[doc = "0x550 - AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
    pub aes_ctrl: crate::Reg<aes_ctrl::AES_CTRL_SPEC>,
    #[doc = "0x554 - AES crypto length registers (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aes_c_length_0: crate::Reg<aes_c_length_0::AES_C_LENGTH_0_SPEC>,
    #[doc = "0x558 - AES crypto length registers (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
    pub aes_c_length_1: crate::Reg<aes_c_length_1::AES_C_LENGTH_1_SPEC>,
    #[doc = "0x55c - Authentication length register"]
    pub aes_auth_length: crate::Reg<aes_auth_length::AES_AUTH_LENGTH_SPEC>,
    #[doc = "0x560 - Data input/output registers The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    pub aes_data_in_out_0: crate::Reg<aes_data_in_out_0::AES_DATA_IN_OUT_0_SPEC>,
    #[doc = "0x564 - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    pub aes_data_in_out_1: crate::Reg<aes_data_in_out_1::AES_DATA_IN_OUT_1_SPEC>,
    #[doc = "0x568 - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    pub aes_data_in_out_2: crate::Reg<aes_data_in_out_2::AES_DATA_IN_OUT_2_SPEC>,
    #[doc = "0x56c - Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
    pub aes_data_in_out_3: crate::Reg<aes_data_in_out_3::AES_DATA_IN_OUT_3_SPEC>,
    #[doc = "0x570 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
    pub aes_tag_out_0: crate::Reg<aes_tag_out_0::AES_TAG_OUT_0_SPEC>,
    #[doc = "0x574 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    pub aes_tag_out_1: crate::Reg<aes_tag_out_1::AES_TAG_OUT_1_SPEC>,
    #[doc = "0x578 - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    pub aes_tag_out_2: crate::Reg<aes_tag_out_2::AES_TAG_OUT_2_SPEC>,
    #[doc = "0x57c - TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
    pub aes_tag_out_3: crate::Reg<aes_tag_out_3::AES_TAG_OUT_3_SPEC>,
    _reserved40: [u8; 0x80],
    #[doc = "0x600 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_0: crate::Reg<hash_data_in_0::HASH_DATA_IN_0_SPEC>,
    #[doc = "0x604 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_1: crate::Reg<hash_data_in_1::HASH_DATA_IN_1_SPEC>,
    #[doc = "0x608 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_2: crate::Reg<hash_data_in_2::HASH_DATA_IN_2_SPEC>,
    #[doc = "0x60c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_3: crate::Reg<hash_data_in_3::HASH_DATA_IN_3_SPEC>,
    #[doc = "0x610 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_4: crate::Reg<hash_data_in_4::HASH_DATA_IN_4_SPEC>,
    #[doc = "0x614 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_5: crate::Reg<hash_data_in_5::HASH_DATA_IN_5_SPEC>,
    #[doc = "0x618 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_6: crate::Reg<hash_data_in_6::HASH_DATA_IN_6_SPEC>,
    #[doc = "0x61c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_7: crate::Reg<hash_data_in_7::HASH_DATA_IN_7_SPEC>,
    #[doc = "0x620 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_8: crate::Reg<hash_data_in_8::HASH_DATA_IN_8_SPEC>,
    #[doc = "0x624 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_9: crate::Reg<hash_data_in_9::HASH_DATA_IN_9_SPEC>,
    #[doc = "0x628 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_10: crate::Reg<hash_data_in_10::HASH_DATA_IN_10_SPEC>,
    #[doc = "0x62c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_11: crate::Reg<hash_data_in_11::HASH_DATA_IN_11_SPEC>,
    #[doc = "0x630 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_12: crate::Reg<hash_data_in_12::HASH_DATA_IN_12_SPEC>,
    #[doc = "0x634 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_13: crate::Reg<hash_data_in_13::HASH_DATA_IN_13_SPEC>,
    #[doc = "0x638 - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_14: crate::Reg<hash_data_in_14::HASH_DATA_IN_14_SPEC>,
    #[doc = "0x63c - HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
    pub hash_data_in_15: crate::Reg<hash_data_in_15::HASH_DATA_IN_15_SPEC>,
    #[doc = "0x640 - Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
    pub hash_io_buf_ctrl: crate::Reg<hash_io_buf_ctrl::HASH_IO_BUF_CTRL_SPEC>,
    #[doc = "0x644 - Hash mode register"]
    pub hash_mode_in: crate::Reg<hash_mode_in::HASH_MODE_IN_SPEC>,
    #[doc = "0x648 - Hash length register"]
    pub hash_length_in_l: crate::Reg<hash_length_in_l::HASH_LENGTH_IN_L_SPEC>,
    #[doc = "0x64c - Hash length register"]
    pub hash_length_in_h: crate::Reg<hash_length_in_h::HASH_LENGTH_IN_H_SPEC>,
    #[doc = "0x650 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_a: crate::Reg<hash_digest_a::HASH_DIGEST_A_SPEC>,
    #[doc = "0x654 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_b: crate::Reg<hash_digest_b::HASH_DIGEST_B_SPEC>,
    #[doc = "0x658 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_c: crate::Reg<hash_digest_c::HASH_DIGEST_C_SPEC>,
    #[doc = "0x65c - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_d: crate::Reg<hash_digest_d::HASH_DIGEST_D_SPEC>,
    #[doc = "0x660 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_e: crate::Reg<hash_digest_e::HASH_DIGEST_E_SPEC>,
    #[doc = "0x664 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_f: crate::Reg<hash_digest_f::HASH_DIGEST_F_SPEC>,
    #[doc = "0x668 - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_g: crate::Reg<hash_digest_g::HASH_DIGEST_G_SPEC>,
    #[doc = "0x66c - Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
    pub hash_digest_h: crate::Reg<hash_digest_h::HASH_DIGEST_H_SPEC>,
    _reserved68: [u8; 0x90],
    #[doc = "0x700 - Algorithm select This algorithm selection register configures the internal destination of the DMA controller."]
    pub ctrl_alg_sel: crate::Reg<ctrl_alg_sel::CTRL_ALG_SEL_SPEC>,
    #[doc = "0x704 - Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
    pub ctrl_prot_en: crate::Reg<ctrl_prot_en::CTRL_PROT_EN_SPEC>,
    _reserved70: [u8; 0x38],
    #[doc = "0x740 - Software reset"]
    pub ctrl_sw_reset: crate::Reg<ctrl_sw_reset::CTRL_SW_RESET_SPEC>,
    _reserved71: [u8; 0x3c],
    #[doc = "0x780 - Interrupt configuration"]
    pub ctrl_int_cfg: crate::Reg<ctrl_int_cfg::CTRL_INT_CFG_SPEC>,
    #[doc = "0x784 - Interrupt enable"]
    pub ctrl_int_en: crate::Reg<ctrl_int_en::CTRL_INT_EN_SPEC>,
    #[doc = "0x788 - Interrupt clear"]
    pub ctrl_int_clr: crate::Reg<ctrl_int_clr::CTRL_INT_CLR_SPEC>,
    #[doc = "0x78c - Interrupt set"]
    pub ctrl_int_set: crate::Reg<ctrl_int_set::CTRL_INT_SET_SPEC>,
    #[doc = "0x790 - Interrupt status"]
    pub ctrl_int_stat: crate::Reg<ctrl_int_stat::CTRL_INT_STAT_SPEC>,
    _reserved76: [u8; 0x64],
    #[doc = "0x7f8 - Options register"]
    pub ctrl_options: crate::Reg<ctrl_options::CTRL_OPTIONS_SPEC>,
    #[doc = "0x7fc - Version register"]
    pub ctrl_version: crate::Reg<ctrl_version::CTRL_VERSION_SPEC>,
}
#[doc = "DMAC_CH0_CTRL register accessor: an alias for `Reg<DMAC_CH0_CTRL_SPEC>`"]
pub type DMAC_CH0_CTRL = crate::Reg<dmac_ch0_ctrl::DMAC_CH0_CTRL_SPEC>;
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmac_ch0_ctrl;
#[doc = "DMAC_CH0_EXTADDR register accessor: an alias for `Reg<DMAC_CH0_EXTADDR_SPEC>`"]
pub type DMAC_CH0_EXTADDR = crate::Reg<dmac_ch0_extaddr::DMAC_CH0_EXTADDR_SPEC>;
#[doc = "Channel external address"]
pub mod dmac_ch0_extaddr;
#[doc = "DMAC_CH0_DMALENGTH register accessor: an alias for `Reg<DMAC_CH0_DMALENGTH_SPEC>`"]
pub type DMAC_CH0_DMALENGTH = crate::Reg<dmac_ch0_dmalength::DMAC_CH0_DMALENGTH_SPEC>;
#[doc = "Channel DMA length"]
pub mod dmac_ch0_dmalength;
#[doc = "DMAC_STATUS register accessor: an alias for `Reg<DMAC_STATUS_SPEC>`"]
pub type DMAC_STATUS = crate::Reg<dmac_status::DMAC_STATUS_SPEC>;
#[doc = "DMAC status This register provides the actual state of each DMA channel. It also reports port errors in case these were received by the master interface module during the data transfer."]
pub mod dmac_status;
#[doc = "DMAC_SWRES register accessor: an alias for `Reg<DMAC_SWRES_SPEC>`"]
pub type DMAC_SWRES = crate::Reg<dmac_swres::DMAC_SWRES_SPEC>;
#[doc = "DMAC software reset register Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMAC status registers."]
pub mod dmac_swres;
#[doc = "DMAC_CH1_CTRL register accessor: an alias for `Reg<DMAC_CH1_CTRL_SPEC>`"]
pub type DMAC_CH1_CTRL = crate::Reg<dmac_ch1_ctrl::DMAC_CH1_CTRL_SPEC>;
#[doc = "Channel control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished."]
pub mod dmac_ch1_ctrl;
#[doc = "DMAC_CH1_EXTADDR register accessor: an alias for `Reg<DMAC_CH1_EXTADDR_SPEC>`"]
pub type DMAC_CH1_EXTADDR = crate::Reg<dmac_ch1_extaddr::DMAC_CH1_EXTADDR_SPEC>;
#[doc = "Channel external address"]
pub mod dmac_ch1_extaddr;
#[doc = "DMAC_CH1_DMALENGTH register accessor: an alias for `Reg<DMAC_CH1_DMALENGTH_SPEC>`"]
pub type DMAC_CH1_DMALENGTH = crate::Reg<dmac_ch1_dmalength::DMAC_CH1_DMALENGTH_SPEC>;
#[doc = "Channel DMA length"]
pub mod dmac_ch1_dmalength;
#[doc = "DMAC_MST_RUNPARAMS register accessor: an alias for `Reg<DMAC_MST_RUNPARAMS_SPEC>`"]
pub type DMAC_MST_RUNPARAMS = crate::Reg<dmac_mst_runparams::DMAC_MST_RUNPARAMS_SPEC>;
#[doc = "DMAC master run-time parameters This register defines all the run-time parameters for the AHB master interface port. These parameters are required for the proper functioning of the EIP-101m AHB master adapter."]
pub mod dmac_mst_runparams;
#[doc = "DMAC_PERSR register accessor: an alias for `Reg<DMAC_PERSR_SPEC>`"]
pub type DMAC_PERSR = crate::Reg<dmac_persr::DMAC_PERSR_SPEC>;
#[doc = "DMAC port error raw status register This register provides the actual status of individual port errors. It also indicates which channel is serviced by an external AHB port (which is frozen by a port error). A port error aborts operations on all serviced channels (channel enable bit is forced to 0) and prevents further transfers via that port until the error is cleared by writing to the DMAC_SWRES register."]
pub mod dmac_persr;
#[doc = "DMAC_OPTIONS register accessor: an alias for `Reg<DMAC_OPTIONS_SPEC>`"]
pub type DMAC_OPTIONS = crate::Reg<dmac_options::DMAC_OPTIONS_SPEC>;
#[doc = "DMAC options register These registers contain information regarding the different options configured in this DMAC."]
pub mod dmac_options;
#[doc = "DMAC_VERSION register accessor: an alias for `Reg<DMAC_VERSION_SPEC>`"]
pub type DMAC_VERSION = crate::Reg<dmac_version::DMAC_VERSION_SPEC>;
#[doc = "DMAC version register This register contains an indication (or signature) of the EIP type of this DMAC, as well as the hardware version/patch numbers."]
pub mod dmac_version;
#[doc = "KEY_STORE_WRITE_AREA register accessor: an alias for `Reg<KEY_STORE_WRITE_AREA_SPEC>`"]
pub type KEY_STORE_WRITE_AREA = crate::Reg<key_store_write_area::KEY_STORE_WRITE_AREA_SPEC>;
#[doc = "Key store write area register This register defines where the keys should be written in the key store RAM. After writing this register, the key store module is ready to receive the keys through a DMA operation. In case the key data transfer triggered an error in the key store, the error will be available in the interrupt status register after the DMA is finished. The key store write-error is asserted when the programmed/selected area is not completely written. This error is also asserted when the DMA operation writes to ram areas that are not selected. The key store RAM is divided into 8 areas of 128 bits. 192-bit keys written in the key store RAM should start on boundaries of 256 bits. This means that writing a 192-bit key to the key store RAM must be done by writing 256 bits of data with the 64 most-significant bits set to 0. These bits are ignored by the AES engine."]
pub mod key_store_write_area;
#[doc = "KEY_STORE_WRITTEN_AREA register accessor: an alias for `Reg<KEY_STORE_WRITTEN_AREA_SPEC>`"]
pub type KEY_STORE_WRITTEN_AREA = crate::Reg<key_store_written_area::KEY_STORE_WRITTEN_AREA_SPEC>;
#[doc = "Key store written area register This register shows which areas of the key store RAM contain valid written keys. When a new key needs to be written to the key store, on a location that is already occupied by a valid key, this key area must be cleared first. This can be done by writing this register before the new key is written to the key store memory. Attempting to write to a key area that already contains a valid key is not allowed and results in an error."]
pub mod key_store_written_area;
#[doc = "KEY_STORE_SIZE register accessor: an alias for `Reg<KEY_STORE_SIZE_SPEC>`"]
pub type KEY_STORE_SIZE = crate::Reg<key_store_size::KEY_STORE_SIZE_SPEC>;
#[doc = "Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register."]
pub mod key_store_size;
#[doc = "KEY_STORE_READ_AREA register accessor: an alias for `Reg<KEY_STORE_READ_AREA_SPEC>`"]
pub type KEY_STORE_READ_AREA = crate::Reg<key_store_read_area::KEY_STORE_READ_AREA_SPEC>;
#[doc = "Key store read area register This register selects the key store RAM area from where the key needs to be read that will be used for an AES operation. The operation directly starts after writing this register. When the operation is finished, the status of the key store read operation is available in the interrupt status register. Key store read error is asserted when a RAM area is selected which does not contain valid written key."]
pub mod key_store_read_area;
#[doc = "AES_KEY2_0 register accessor: an alias for `Reg<AES_KEY2_0_SPEC>`"]
pub type AES_KEY2_0 = crate::Reg<aes_key2_0::AES_KEY2_0_SPEC>;
#[doc = "AES_KEY2_0 / AES_GHASH_H_IN_0 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_0;
#[doc = "AES_KEY2_1 register accessor: an alias for `Reg<AES_KEY2_1_SPEC>`"]
pub type AES_KEY2_1 = crate::Reg<aes_key2_1::AES_KEY2_1_SPEC>;
#[doc = "AES_KEY2_1 / AES_GHASH_H_IN_1 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_1;
#[doc = "AES_KEY2_2 register accessor: an alias for `Reg<AES_KEY2_2_SPEC>`"]
pub type AES_KEY2_2 = crate::Reg<aes_key2_2::AES_KEY2_2_SPEC>;
#[doc = "AES_KEY2_2 / AES_GHASH_H_IN_2 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_2;
#[doc = "AES_KEY2_3 register accessor: an alias for `Reg<AES_KEY2_3_SPEC>`"]
pub type AES_KEY2_3 = crate::Reg<aes_key2_3::AES_KEY2_3_SPEC>;
#[doc = "AES_KEY2_3 / AES_GHASH_H_IN_3 Second Key / GHASH Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key2_3;
#[doc = "AES_KEY3_0 register accessor: an alias for `Reg<AES_KEY3_0_SPEC>`"]
pub type AES_KEY3_0 = crate::Reg<aes_key3_0::AES_KEY3_0_SPEC>;
#[doc = "AES_KEY3_0 / AES_KEY2_4 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_0;
#[doc = "AES_KEY3_1 register accessor: an alias for `Reg<AES_KEY3_1_SPEC>`"]
pub type AES_KEY3_1 = crate::Reg<aes_key3_1::AES_KEY3_1_SPEC>;
#[doc = "AES_KEY3_1 / AES_KEY2_5 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_1;
#[doc = "AES_KEY3_2 register accessor: an alias for `Reg<AES_KEY3_2_SPEC>`"]
pub type AES_KEY3_2 = crate::Reg<aes_key3_2::AES_KEY3_2_SPEC>;
#[doc = "AES_KEY3_2 / AES_KEY2_6 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_2;
#[doc = "AES_KEY3_3 register accessor: an alias for `Reg<AES_KEY3_3_SPEC>`"]
pub type AES_KEY3_3 = crate::Reg<aes_key3_3::AES_KEY3_3_SPEC>;
#[doc = "AES_KEY3_3 / AES_KEY2_7 Third Key / Second Key (internal, but clearable) The following registers are not accessible through the host for reading and writing. They are used to store internally calculated key information and intermediate results. However, when the host performs a write to the any of the respective AES_KEY2_n or AES_KEY3_n addresses, respectively the whole 128-bit AES_KEY2_n or AES_KEY3_n register is cleared to 0s. The AES_GHASH_H_IN_n registers (required for GHASH, which is part of GCM) are mapped to the AES_KEY2_n registers. The (intermediate) authentication result for GCM and CCM is stored in the AES_KEY3_n register."]
pub mod aes_key3_3;
#[doc = "AES_IV_0 register accessor: an alias for `Reg<AES_IV_0_SPEC>`"]
pub type AES_IV_0 = crate::Reg<aes_iv_0::AES_IV_0_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_0;
#[doc = "AES_IV_1 register accessor: an alias for `Reg<AES_IV_1_SPEC>`"]
pub type AES_IV_1 = crate::Reg<aes_iv_1::AES_IV_1_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_1;
#[doc = "AES_IV_2 register accessor: an alias for `Reg<AES_IV_2_SPEC>`"]
pub type AES_IV_2 = crate::Reg<aes_iv_2::AES_IV_2_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_2;
#[doc = "AES_IV_3 register accessor: an alias for `Reg<AES_IV_3_SPEC>`"]
pub type AES_IV_3 = crate::Reg<aes_iv_3::AES_IV_3_SPEC>;
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine."]
pub mod aes_iv_3;
#[doc = "AES_CTRL register accessor: an alias for `Reg<AES_CTRL_SPEC>`"]
pub type AES_CTRL = crate::Reg<aes_ctrl::AES_CTRL_SPEC>;
#[doc = "AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0."]
pub mod aes_ctrl;
#[doc = "AES_C_LENGTH_0 register accessor: an alias for `Reg<AES_C_LENGTH_0_SPEC>`"]
pub type AES_C_LENGTH_0 = crate::Reg<aes_c_length_0::AES_C_LENGTH_0_SPEC>;
#[doc = "AES crypto length registers (LSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM, and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aes_c_length_0;
#[doc = "AES_C_LENGTH_1 register accessor: an alias for `Reg<AES_C_LENGTH_1_SPEC>`"]
pub type AES_C_LENGTH_1 = crate::Reg<aes_c_length_1::AES_C_LENGTH_1_SPEC>;
#[doc = "AES crypto length registers (MSW) These registers are used to write the Length values to the EIP-120t. While processing, the length values decrement to 0. If both lengths are 0, the data stream is finished and a new context is requested. For basic AES modes (ECB, CBC, and CTR), a crypto length of 0 can be written if multiple streams need to be processed with the same key. Writing 0 length results in continued data requests until a new context is written. For the other modes (CBC-MAC, GCM and CCM) no (new) data requests are done if the length decrements to or equals 0. It is advised to write a new length per packet. If the length registers decrement to 0, no new data is processed until a new context or length value is written. When writing a new mode without writing the length registers, the length register values from the previous context is reused."]
pub mod aes_c_length_1;
#[doc = "AES_AUTH_LENGTH register accessor: an alias for `Reg<AES_AUTH_LENGTH_SPEC>`"]
pub type AES_AUTH_LENGTH = crate::Reg<aes_auth_length::AES_AUTH_LENGTH_SPEC>;
#[doc = "Authentication length register"]
pub mod aes_auth_length;
#[doc = "AES_DATA_IN_OUT_0 register accessor: an alias for `Reg<AES_DATA_IN_OUT_0_SPEC>`"]
pub type AES_DATA_IN_OUT_0 = crate::Reg<aes_data_in_out_0::AES_DATA_IN_OUT_0_SPEC>;
#[doc = "Data input/output registers The data registers are typically accessed through the DMA and not with host writes and/or reads. However, for debugging purposes the data input/output registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_0;
#[doc = "AES_DATA_IN_OUT_1 register accessor: an alias for `Reg<AES_DATA_IN_OUT_1_SPEC>`"]
pub type AES_DATA_IN_OUT_1 = crate::Reg<aes_data_in_out_1::AES_DATA_IN_OUT_1_SPEC>;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_1;
#[doc = "AES_DATA_IN_OUT_2 register accessor: an alias for `Reg<AES_DATA_IN_OUT_2_SPEC>`"]
pub type AES_DATA_IN_OUT_2 = crate::Reg<aes_data_in_out_2::AES_DATA_IN_OUT_2_SPEC>;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_2;
#[doc = "AES_DATA_IN_OUT_3 register accessor: an alias for `Reg<AES_DATA_IN_OUT_3_SPEC>`"]
pub type AES_DATA_IN_OUT_3 = crate::Reg<aes_data_in_out_3::AES_DATA_IN_OUT_3_SPEC>;
#[doc = "Data Input/Output Registers The data registers are typically accessed via DMA and not with host writes and/or reads. However, for debugging purposes the Data Input/Output Registers can be accessed via host write and read operations. The registers are used to buffer the input/output data blocks to/from the EIP-120t. Note: The data input buffer (AES_DATA_IN_n) and data output buffer (AES_DATA_OUT_n) are mapped to the same address locations. Writes (both DMA and host) to these addresses load the Input Buffer while reads pull from the Output Buffer. Therefore, for write access, the data input buffer is written; for read access, the data output buffer is read. The data input buffer must be written before starting an operation. The data output buffer contains valid data on completion of an operation. Therefore, any 128-bit data block can be split over multiple 32-bit word transfers; these can be mixed with other host transfers over the external interface."]
pub mod aes_data_in_out_3;
#[doc = "AES_TAG_OUT_0 register accessor: an alias for `Reg<AES_TAG_OUT_0_SPEC>`"]
pub type AES_TAG_OUT_0 = crate::Reg<aes_tag_out_0::AES_TAG_OUT_0_SPEC>;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice."]
pub mod aes_tag_out_0;
#[doc = "AES_TAG_OUT_1 register accessor: an alias for `Reg<AES_TAG_OUT_1_SPEC>`"]
pub type AES_TAG_OUT_1 = crate::Reg<aes_tag_out_1::AES_TAG_OUT_1_SPEC>;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_1;
#[doc = "AES_TAG_OUT_2 register accessor: an alias for `Reg<AES_TAG_OUT_2_SPEC>`"]
pub type AES_TAG_OUT_2 = crate::Reg<aes_tag_out_2::AES_TAG_OUT_2_SPEC>;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_2;
#[doc = "AES_TAG_OUT_3 register accessor: an alias for `Reg<AES_TAG_OUT_3_SPEC>`"]
pub type AES_TAG_OUT_3 = crate::Reg<aes_tag_out_3::AES_TAG_OUT_3_SPEC>;
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice."]
pub mod aes_tag_out_3;
#[doc = "HASH_DATA_IN_0 register accessor: an alias for `Reg<HASH_DATA_IN_0_SPEC>`"]
pub type HASH_DATA_IN_0 = crate::Reg<hash_data_in_0::HASH_DATA_IN_0_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_0;
#[doc = "HASH_DATA_IN_1 register accessor: an alias for `Reg<HASH_DATA_IN_1_SPEC>`"]
pub type HASH_DATA_IN_1 = crate::Reg<hash_data_in_1::HASH_DATA_IN_1_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_1;
#[doc = "HASH_DATA_IN_2 register accessor: an alias for `Reg<HASH_DATA_IN_2_SPEC>`"]
pub type HASH_DATA_IN_2 = crate::Reg<hash_data_in_2::HASH_DATA_IN_2_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_2;
#[doc = "HASH_DATA_IN_3 register accessor: an alias for `Reg<HASH_DATA_IN_3_SPEC>`"]
pub type HASH_DATA_IN_3 = crate::Reg<hash_data_in_3::HASH_DATA_IN_3_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_3;
#[doc = "HASH_DATA_IN_4 register accessor: an alias for `Reg<HASH_DATA_IN_4_SPEC>`"]
pub type HASH_DATA_IN_4 = crate::Reg<hash_data_in_4::HASH_DATA_IN_4_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_4;
#[doc = "HASH_DATA_IN_5 register accessor: an alias for `Reg<HASH_DATA_IN_5_SPEC>`"]
pub type HASH_DATA_IN_5 = crate::Reg<hash_data_in_5::HASH_DATA_IN_5_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_5;
#[doc = "HASH_DATA_IN_6 register accessor: an alias for `Reg<HASH_DATA_IN_6_SPEC>`"]
pub type HASH_DATA_IN_6 = crate::Reg<hash_data_in_6::HASH_DATA_IN_6_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_6;
#[doc = "HASH_DATA_IN_7 register accessor: an alias for `Reg<HASH_DATA_IN_7_SPEC>`"]
pub type HASH_DATA_IN_7 = crate::Reg<hash_data_in_7::HASH_DATA_IN_7_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_7;
#[doc = "HASH_DATA_IN_8 register accessor: an alias for `Reg<HASH_DATA_IN_8_SPEC>`"]
pub type HASH_DATA_IN_8 = crate::Reg<hash_data_in_8::HASH_DATA_IN_8_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_8;
#[doc = "HASH_DATA_IN_9 register accessor: an alias for `Reg<HASH_DATA_IN_9_SPEC>`"]
pub type HASH_DATA_IN_9 = crate::Reg<hash_data_in_9::HASH_DATA_IN_9_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_9;
#[doc = "HASH_DATA_IN_10 register accessor: an alias for `Reg<HASH_DATA_IN_10_SPEC>`"]
pub type HASH_DATA_IN_10 = crate::Reg<hash_data_in_10::HASH_DATA_IN_10_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_10;
#[doc = "HASH_DATA_IN_11 register accessor: an alias for `Reg<HASH_DATA_IN_11_SPEC>`"]
pub type HASH_DATA_IN_11 = crate::Reg<hash_data_in_11::HASH_DATA_IN_11_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_11;
#[doc = "HASH_DATA_IN_12 register accessor: an alias for `Reg<HASH_DATA_IN_12_SPEC>`"]
pub type HASH_DATA_IN_12 = crate::Reg<hash_data_in_12::HASH_DATA_IN_12_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_12;
#[doc = "HASH_DATA_IN_13 register accessor: an alias for `Reg<HASH_DATA_IN_13_SPEC>`"]
pub type HASH_DATA_IN_13 = crate::Reg<hash_data_in_13::HASH_DATA_IN_13_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_13;
#[doc = "HASH_DATA_IN_14 register accessor: an alias for `Reg<HASH_DATA_IN_14_SPEC>`"]
pub type HASH_DATA_IN_14 = crate::Reg<hash_data_in_14::HASH_DATA_IN_14_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_14;
#[doc = "HASH_DATA_IN_15 register accessor: an alias for `Reg<HASH_DATA_IN_15_SPEC>`"]
pub type HASH_DATA_IN_15 = crate::Reg<hash_data_in_15::HASH_DATA_IN_15_SPEC>;
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface."]
pub mod hash_data_in_15;
#[doc = "HASH_IO_BUF_CTRL register accessor: an alias for `Reg<HASH_IO_BUF_CTRL_SPEC>`"]
pub type HASH_IO_BUF_CTRL = crate::Reg<hash_io_buf_ctrl::HASH_IO_BUF_CTRL_SPEC>;
#[doc = "Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine."]
pub mod hash_io_buf_ctrl;
#[doc = "HASH_MODE_IN register accessor: an alias for `Reg<HASH_MODE_IN_SPEC>`"]
pub type HASH_MODE_IN = crate::Reg<hash_mode_in::HASH_MODE_IN_SPEC>;
#[doc = "Hash mode register"]
pub mod hash_mode_in;
#[doc = "HASH_LENGTH_IN_L register accessor: an alias for `Reg<HASH_LENGTH_IN_L_SPEC>`"]
pub type HASH_LENGTH_IN_L = crate::Reg<hash_length_in_l::HASH_LENGTH_IN_L_SPEC>;
#[doc = "Hash length register"]
pub mod hash_length_in_l;
#[doc = "HASH_LENGTH_IN_H register accessor: an alias for `Reg<HASH_LENGTH_IN_H_SPEC>`"]
pub type HASH_LENGTH_IN_H = crate::Reg<hash_length_in_h::HASH_LENGTH_IN_H_SPEC>;
#[doc = "Hash length register"]
pub mod hash_length_in_h;
#[doc = "HASH_DIGEST_A register accessor: an alias for `Reg<HASH_DIGEST_A_SPEC>`"]
pub type HASH_DIGEST_A = crate::Reg<hash_digest_a::HASH_DIGEST_A_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_a;
#[doc = "HASH_DIGEST_B register accessor: an alias for `Reg<HASH_DIGEST_B_SPEC>`"]
pub type HASH_DIGEST_B = crate::Reg<hash_digest_b::HASH_DIGEST_B_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_b;
#[doc = "HASH_DIGEST_C register accessor: an alias for `Reg<HASH_DIGEST_C_SPEC>`"]
pub type HASH_DIGEST_C = crate::Reg<hash_digest_c::HASH_DIGEST_C_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_c;
#[doc = "HASH_DIGEST_D register accessor: an alias for `Reg<HASH_DIGEST_D_SPEC>`"]
pub type HASH_DIGEST_D = crate::Reg<hash_digest_d::HASH_DIGEST_D_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_d;
#[doc = "HASH_DIGEST_E register accessor: an alias for `Reg<HASH_DIGEST_E_SPEC>`"]
pub type HASH_DIGEST_E = crate::Reg<hash_digest_e::HASH_DIGEST_E_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_e;
#[doc = "HASH_DIGEST_F register accessor: an alias for `Reg<HASH_DIGEST_F_SPEC>`"]
pub type HASH_DIGEST_F = crate::Reg<hash_digest_f::HASH_DIGEST_F_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_f;
#[doc = "HASH_DIGEST_G register accessor: an alias for `Reg<HASH_DIGEST_G_SPEC>`"]
pub type HASH_DIGEST_G = crate::Reg<hash_digest_g::HASH_DIGEST_G_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_g;
#[doc = "HASH_DIGEST_H register accessor: an alias for `Reg<HASH_DIGEST_H_SPEC>`"]
pub type HASH_DIGEST_H = crate::Reg<hash_digest_h::HASH_DIGEST_H_SPEC>;
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations."]
pub mod hash_digest_h;
#[doc = "CTRL_ALG_SEL register accessor: an alias for `Reg<CTRL_ALG_SEL_SPEC>`"]
pub type CTRL_ALG_SEL = crate::Reg<ctrl_alg_sel::CTRL_ALG_SEL_SPEC>;
#[doc = "Algorithm select This algorithm selection register configures the internal destination of the DMA controller."]
pub mod ctrl_alg_sel;
#[doc = "CTRL_PROT_EN register accessor: an alias for `Reg<CTRL_PROT_EN_SPEC>`"]
pub type CTRL_PROT_EN = crate::Reg<ctrl_prot_en::CTRL_PROT_EN_SPEC>;
#[doc = "Master PROT privileged access enable This register enables the second bit (bit \\[1\\]) of the AHB HPROT bus of the AHB master interface when a read action of key(s) is performed on the AHB master interface for writing keys into the store module."]
pub mod ctrl_prot_en;
#[doc = "CTRL_SW_RESET register accessor: an alias for `Reg<CTRL_SW_RESET_SPEC>`"]
pub type CTRL_SW_RESET = crate::Reg<ctrl_sw_reset::CTRL_SW_RESET_SPEC>;
#[doc = "Software reset"]
pub mod ctrl_sw_reset;
#[doc = "CTRL_INT_CFG register accessor: an alias for `Reg<CTRL_INT_CFG_SPEC>`"]
pub type CTRL_INT_CFG = crate::Reg<ctrl_int_cfg::CTRL_INT_CFG_SPEC>;
#[doc = "Interrupt configuration"]
pub mod ctrl_int_cfg;
#[doc = "CTRL_INT_EN register accessor: an alias for `Reg<CTRL_INT_EN_SPEC>`"]
pub type CTRL_INT_EN = crate::Reg<ctrl_int_en::CTRL_INT_EN_SPEC>;
#[doc = "Interrupt enable"]
pub mod ctrl_int_en;
#[doc = "CTRL_INT_CLR register accessor: an alias for `Reg<CTRL_INT_CLR_SPEC>`"]
pub type CTRL_INT_CLR = crate::Reg<ctrl_int_clr::CTRL_INT_CLR_SPEC>;
#[doc = "Interrupt clear"]
pub mod ctrl_int_clr;
#[doc = "CTRL_INT_SET register accessor: an alias for `Reg<CTRL_INT_SET_SPEC>`"]
pub type CTRL_INT_SET = crate::Reg<ctrl_int_set::CTRL_INT_SET_SPEC>;
#[doc = "Interrupt set"]
pub mod ctrl_int_set;
#[doc = "CTRL_INT_STAT register accessor: an alias for `Reg<CTRL_INT_STAT_SPEC>`"]
pub type CTRL_INT_STAT = crate::Reg<ctrl_int_stat::CTRL_INT_STAT_SPEC>;
#[doc = "Interrupt status"]
pub mod ctrl_int_stat;
#[doc = "CTRL_OPTIONS register accessor: an alias for `Reg<CTRL_OPTIONS_SPEC>`"]
pub type CTRL_OPTIONS = crate::Reg<ctrl_options::CTRL_OPTIONS_SPEC>;
#[doc = "Options register"]
pub mod ctrl_options;
#[doc = "CTRL_VERSION register accessor: an alias for `Reg<CTRL_VERSION_SPEC>`"]
pub type CTRL_VERSION = crate::Reg<ctrl_version::CTRL_VERSION_SPEC>;
#[doc = "Version register"]
pub mod ctrl_version;
