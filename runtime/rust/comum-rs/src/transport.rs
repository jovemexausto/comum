#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportProfile {
    Ble5,
    Nfc1,
    Qr1,
    Rns1,
}

impl TransportProfile {
    pub fn id(&self) -> &'static str {
        match self {
            TransportProfile::Ble5 => "ble-5",
            TransportProfile::Nfc1 => "nfc-1",
            TransportProfile::Qr1 => "qr-1",
            TransportProfile::Rns1 => "rns-1",
        }
    }

    pub fn mtu(&self) -> usize {
        match self {
            TransportProfile::Ble5 => 512,
            TransportProfile::Nfc1 => 8192,
            TransportProfile::Qr1 => 3072,
            TransportProfile::Rns1 => 500,
        }
    }

    pub fn framing(&self) -> &'static str {
        match self {
            TransportProfile::Ble5 => "gatt+cte",
            TransportProfile::Nfc1 => "ndef+cte",
            TransportProfile::Qr1 => "base45+qr",
            TransportProfile::Rns1 => "rns+cte",
        }
    }

    pub fn link_trigger(&self) -> &'static str {
        match self {
            TransportProfile::Ble5 => "ble connect",
            TransportProfile::Nfc1 => "nfc tap",
            TransportProfile::Qr1 => "qr reassembly",
            TransportProfile::Rns1 => "rns peer",
        }
    }

    pub fn requires_origin_hint(&self) -> bool {
        matches!(self, TransportProfile::Rns1)
    }
}

#[cfg(test)]
mod tests {
    use super::TransportProfile;

    #[test]
    fn transport_profiles_have_expected_ids_and_mtu() {
        assert_eq!(TransportProfile::Ble5.id(), "ble-5");
        assert_eq!(TransportProfile::Nfc1.id(), "nfc-1");
        assert_eq!(TransportProfile::Qr1.id(), "qr-1");
        assert_eq!(TransportProfile::Rns1.id(), "rns-1");

        assert_eq!(TransportProfile::Ble5.mtu(), 512);
        assert_eq!(TransportProfile::Nfc1.mtu(), 8192);
        assert_eq!(TransportProfile::Qr1.mtu(), 3072);
        assert_eq!(TransportProfile::Rns1.mtu(), 500);
    }
}
