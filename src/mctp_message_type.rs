use bit_register::{NumBytes, TryFromBits, TryIntoBits};

/// Defined in https://www.dmtf.org/sites/default/files/standards/documents/DSP0239_1.11.1.pdf
#[repr(u8)]
#[derive(
    Copy, Clone, PartialEq, Eq, Debug, Default, num_enum::IntoPrimitive, num_enum::TryFromPrimitive,
)]
pub enum MctpMessageType {
    #[default]
    MctpControl = 0x00,
    PlatformLeveldataModel = 0x01,
    NcSiOverMctp = 0x02,
    EhternetOverMctp = 0x03,
    NvmeManagementMessagesOverMctp = 0x04,
    SpdmOverMctp = 0x05,
    SecuredMessages = 0x06,
    CxlFmApiOverMctp = 0x07,
    CxlCciOverMctp = 0x08,
    PcieMiOverMctp = 0x09,
    // TODO: Inject ODP message type here
    Odp = 0x7D,
    VendorDefinedPci = 0x7E,
    VendorDefinedIana = 0x7F,
}
impl TryFromBits<u32> for MctpMessageType {
    fn try_from_bits(bits: u32) -> Result<Self, &'static str> {
        if bits > 0xFF {
            return Err("Invalid value for MCTP message type");
        }
        (bits as u8)
            .try_into()
            .map_err(|_| "Invalid value for MCTP message type")
    }
}
impl TryIntoBits<u32> for MctpMessageType {
    fn try_into_bits(self) -> Result<u32, &'static str> {
        Ok(Into::<u8>::into(self) as u32)
    }
}
impl NumBytes for MctpMessageType {
    const NUM_BYTES: usize = 1;
}
