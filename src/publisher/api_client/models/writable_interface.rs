/*
 * NetBox API
 *
 * API to access NetBox
 *
 * The version of the OpenAPI document: 3.3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WritableInterface {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "display", skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(rename = "device")]
    pub device: i32,
    #[serde(rename = "module", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub module: Option<Option<i32>>,
    #[serde(rename = "name")]
    pub name: String,
    /// Physical label
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "parent", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Option<i32>>,
    #[serde(rename = "bridge", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<Option<i32>>,
    #[serde(rename = "lag", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub lag: Option<Option<i32>>,
    #[serde(rename = "mtu", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mtu: Option<Option<i32>>,
    #[serde(rename = "mac_address", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<Option<String>>,
    #[serde(rename = "speed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub speed: Option<Option<i32>>,
    #[serde(rename = "duplex", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub duplex: Option<Option<Duplex>>,
    /// 64-bit World Wide Name
    #[serde(rename = "wwn", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wwn: Option<Option<String>>,
    /// This interface is used only for out-of-band management
    #[serde(rename = "mgmt_only", skip_serializing_if = "Option::is_none")]
    pub mgmt_only: Option<bool>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[serde(rename = "rf_role", skip_serializing_if = "Option::is_none")]
    pub rf_role: Option<RfRole>,
    #[serde(rename = "rf_channel", skip_serializing_if = "Option::is_none")]
    pub rf_channel: Option<RfChannel>,
    #[serde(rename = "poe_mode", skip_serializing_if = "Option::is_none")]
    pub poe_mode: Option<PoeMode>,
    #[serde(rename = "poe_type", skip_serializing_if = "Option::is_none")]
    pub poe_type: Option<PoeType>,
    #[serde(rename = "rf_channel_frequency", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rf_channel_frequency: Option<Option<f32>>,
    #[serde(rename = "rf_channel_width", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rf_channel_width: Option<Option<f32>>,
    #[serde(rename = "tx_power", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tx_power: Option<Option<i32>>,
    #[serde(rename = "untagged_vlan", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub untagged_vlan: Option<Option<i32>>,
    #[serde(rename = "tagged_vlans", skip_serializing_if = "Option::is_none")]
    pub tagged_vlans: Option<Vec<i32>>,
    /// Treat as if a cable is connected
    #[serde(rename = "mark_connected", skip_serializing_if = "Option::is_none")]
    pub mark_connected: Option<bool>,
    #[serde(rename = "cable", skip_serializing_if = "Option::is_none")]
    pub cable: Option<Box<crate::models::NestedCable>>,
    #[serde(rename = "cable_end", skip_serializing_if = "Option::is_none")]
    pub cable_end: Option<String>,
    #[serde(rename = "wireless_link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub wireless_link: Option<Option<i32>>,
    ///  Return the appropriate serializer for the link termination model. 
    #[serde(rename = "link_peers", skip_serializing_if = "Option::is_none")]
    pub link_peers: Option<Vec<String>>,
    #[serde(rename = "link_peers_type", skip_serializing_if = "Option::is_none")]
    pub link_peers_type: Option<String>,
    #[serde(rename = "wireless_lans", skip_serializing_if = "Option::is_none")]
    pub wireless_lans: Option<Vec<i32>>,
    #[serde(rename = "vrf", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vrf: Option<Option<i32>>,
    #[serde(rename = "l2vpn_termination", skip_serializing_if = "Option::is_none")]
    pub l2vpn_termination: Option<String>,
    ///  Return the appropriate serializer for the type of connected object. 
    #[serde(rename = "connected_endpoints", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints: Option<Vec<String>>,
    #[serde(rename = "connected_endpoints_type", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints_type: Option<String>,
    #[serde(rename = "connected_endpoints_reachable", skip_serializing_if = "Option::is_none")]
    pub connected_endpoints_reachable: Option<bool>,
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<crate::models::NestedTag>>,
    #[serde(rename = "custom_fields", skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "last_updated", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<Option<String>>,
    #[serde(rename = "count_ipaddresses", skip_serializing_if = "Option::is_none")]
    pub count_ipaddresses: Option<i32>,
    #[serde(rename = "count_fhrp_groups", skip_serializing_if = "Option::is_none")]
    pub count_fhrp_groups: Option<i32>,
    #[serde(rename = "_occupied", skip_serializing_if = "Option::is_none")]
    pub _occupied: Option<bool>,
}

impl WritableInterface {
    pub fn new(device: i32, name: String, r#type: Type) -> WritableInterface {
        WritableInterface {
            id: None,
            url: None,
            display: None,
            device,
            module: None,
            name,
            label: None,
            r#type,
            enabled: None,
            parent: None,
            bridge: None,
            lag: None,
            mtu: None,
            mac_address: None,
            speed: None,
            duplex: None,
            wwn: None,
            mgmt_only: None,
            description: None,
            mode: None,
            rf_role: None,
            rf_channel: None,
            poe_mode: None,
            poe_type: None,
            rf_channel_frequency: None,
            rf_channel_width: None,
            tx_power: None,
            untagged_vlan: None,
            tagged_vlans: None,
            mark_connected: None,
            cable: None,
            cable_end: None,
            wireless_link: None,
            link_peers: None,
            link_peers_type: None,
            wireless_lans: None,
            vrf: None,
            l2vpn_termination: None,
            connected_endpoints: None,
            connected_endpoints_type: None,
            connected_endpoints_reachable: None,
            tags: None,
            custom_fields: None,
            created: None,
            last_updated: None,
            count_ipaddresses: None,
            count_fhrp_groups: None,
            _occupied: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "virtual")]
    Virtual,
    #[serde(rename = "bridge")]
    Bridge,
    #[serde(rename = "lag")]
    Lag,
    #[serde(rename = "100base-tx")]
    Variant100baseTx,
    #[serde(rename = "1000base-t")]
    Variant1000baseT,
    #[serde(rename = "2.5gbase-t")]
    Variant2Period5gbaseT,
    #[serde(rename = "5gbase-t")]
    Variant5gbaseT,
    #[serde(rename = "10gbase-t")]
    Variant10gbaseT,
    #[serde(rename = "10gbase-cx4")]
    Variant10gbaseCx4,
    #[serde(rename = "1000base-x-gbic")]
    Variant1000baseXGbic,
    #[serde(rename = "1000base-x-sfp")]
    Variant1000baseXSfp,
    #[serde(rename = "10gbase-x-sfpp")]
    Variant10gbaseXSfpp,
    #[serde(rename = "10gbase-x-xfp")]
    Variant10gbaseXXfp,
    #[serde(rename = "10gbase-x-xenpak")]
    Variant10gbaseXXenpak,
    #[serde(rename = "10gbase-x-x2")]
    Variant10gbaseXx2,
    #[serde(rename = "25gbase-x-sfp28")]
    Variant25gbaseXSfp28,
    #[serde(rename = "50gbase-x-sfp56")]
    Variant50gbaseXSfp56,
    #[serde(rename = "40gbase-x-qsfpp")]
    Variant40gbaseXQsfpp,
    #[serde(rename = "50gbase-x-sfp28")]
    Variant50gbaseXSfp28,
    #[serde(rename = "100gbase-x-cfp")]
    Variant100gbaseXCfp,
    #[serde(rename = "100gbase-x-cfp2")]
    Variant100gbaseXCfp2,
    #[serde(rename = "200gbase-x-cfp2")]
    Variant200gbaseXCfp2,
    #[serde(rename = "100gbase-x-cfp4")]
    Variant100gbaseXCfp4,
    #[serde(rename = "100gbase-x-cpak")]
    Variant100gbaseXCpak,
    #[serde(rename = "100gbase-x-qsfp28")]
    Variant100gbaseXQsfp28,
    #[serde(rename = "200gbase-x-qsfp56")]
    Variant200gbaseXQsfp56,
    #[serde(rename = "400gbase-x-qsfpdd")]
    Variant400gbaseXQsfpdd,
    #[serde(rename = "400gbase-x-osfp")]
    Variant400gbaseXOsfp,
    #[serde(rename = "ieee802.11a")]
    Ieee802Period11a,
    #[serde(rename = "ieee802.11g")]
    Ieee802Period11g,
    #[serde(rename = "ieee802.11n")]
    Ieee802Period11n,
    #[serde(rename = "ieee802.11ac")]
    Ieee802Period11ac,
    #[serde(rename = "ieee802.11ad")]
    Ieee802Period11ad,
    #[serde(rename = "ieee802.11ax")]
    Ieee802Period11ax,
    #[serde(rename = "ieee802.11ay")]
    Ieee802Period11ay,
    #[serde(rename = "ieee802.15.1")]
    Ieee802Period15Period1,
    #[serde(rename = "other-wireless")]
    OtherWireless,
    #[serde(rename = "gsm")]
    Gsm,
    #[serde(rename = "cdma")]
    Cdma,
    #[serde(rename = "lte")]
    Lte,
    #[serde(rename = "sonet-oc3")]
    SonetOc3,
    #[serde(rename = "sonet-oc12")]
    SonetOc12,
    #[serde(rename = "sonet-oc48")]
    SonetOc48,
    #[serde(rename = "sonet-oc192")]
    SonetOc192,
    #[serde(rename = "sonet-oc768")]
    SonetOc768,
    #[serde(rename = "sonet-oc1920")]
    SonetOc1920,
    #[serde(rename = "sonet-oc3840")]
    SonetOc3840,
    #[serde(rename = "1gfc-sfp")]
    Variant1gfcSfp,
    #[serde(rename = "2gfc-sfp")]
    Variant2gfcSfp,
    #[serde(rename = "4gfc-sfp")]
    Variant4gfcSfp,
    #[serde(rename = "8gfc-sfpp")]
    Variant8gfcSfpp,
    #[serde(rename = "16gfc-sfpp")]
    Variant16gfcSfpp,
    #[serde(rename = "32gfc-sfp28")]
    Variant32gfcSfp28,
    #[serde(rename = "64gfc-qsfpp")]
    Variant64gfcQsfpp,
    #[serde(rename = "128gfc-qsfp28")]
    Variant128gfcQsfp28,
    #[serde(rename = "infiniband-sdr")]
    InfinibandSdr,
    #[serde(rename = "infiniband-ddr")]
    InfinibandDdr,
    #[serde(rename = "infiniband-qdr")]
    InfinibandQdr,
    #[serde(rename = "infiniband-fdr10")]
    InfinibandFdr10,
    #[serde(rename = "infiniband-fdr")]
    InfinibandFdr,
    #[serde(rename = "infiniband-edr")]
    InfinibandEdr,
    #[serde(rename = "infiniband-hdr")]
    InfinibandHdr,
    #[serde(rename = "infiniband-ndr")]
    InfinibandNdr,
    #[serde(rename = "infiniband-xdr")]
    InfinibandXdr,
    #[serde(rename = "t1")]
    T1,
    #[serde(rename = "e1")]
    E1,
    #[serde(rename = "t3")]
    T3,
    #[serde(rename = "e3")]
    E3,
    #[serde(rename = "xdsl")]
    Xdsl,
    #[serde(rename = "docsis")]
    Docsis,
    #[serde(rename = "gpon")]
    Gpon,
    #[serde(rename = "xg-pon")]
    XgPon,
    #[serde(rename = "xgs-pon")]
    XgsPon,
    #[serde(rename = "ng-pon2")]
    NgPon2,
    #[serde(rename = "epon")]
    Epon,
    #[serde(rename = "10g-epon")]
    Variant10gEpon,
    #[serde(rename = "cisco-stackwise")]
    CiscoStackwise,
    #[serde(rename = "cisco-stackwise-plus")]
    CiscoStackwisePlus,
    #[serde(rename = "cisco-flexstack")]
    CiscoFlexstack,
    #[serde(rename = "cisco-flexstack-plus")]
    CiscoFlexstackPlus,
    #[serde(rename = "cisco-stackwise-80")]
    CiscoStackwise80,
    #[serde(rename = "cisco-stackwise-160")]
    CiscoStackwise160,
    #[serde(rename = "cisco-stackwise-320")]
    CiscoStackwise320,
    #[serde(rename = "cisco-stackwise-480")]
    CiscoStackwise480,
    #[serde(rename = "juniper-vcp")]
    JuniperVcp,
    #[serde(rename = "extreme-summitstack")]
    ExtremeSummitstack,
    #[serde(rename = "extreme-summitstack-128")]
    ExtremeSummitstack128,
    #[serde(rename = "extreme-summitstack-256")]
    ExtremeSummitstack256,
    #[serde(rename = "extreme-summitstack-512")]
    ExtremeSummitstack512,
    #[serde(rename = "other")]
    Other,
}

impl Default for Type {
    fn default() -> Type {
        Self::Virtual
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Duplex {
    #[serde(rename = "half")]
    Half,
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "auto")]
    Auto,
}

impl Default for Duplex {
    fn default() -> Duplex {
        Self::Half
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "access")]
    Access,
    #[serde(rename = "tagged")]
    Tagged,
    #[serde(rename = "tagged-all")]
    TaggedAll,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::Access
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RfRole {
    #[serde(rename = "ap")]
    Ap,
    #[serde(rename = "station")]
    Station,
}

impl Default for RfRole {
    fn default() -> RfRole {
        Self::Ap
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RfChannel {
    #[serde(rename = "2.4g-1-2412-22")]
    Variant2Period4g1241222,
    #[serde(rename = "2.4g-2-2417-22")]
    Variant2Period4g2241722,
    #[serde(rename = "2.4g-3-2422-22")]
    Variant2Period4g3242222,
    #[serde(rename = "2.4g-4-2427-22")]
    Variant2Period4g4242722,
    #[serde(rename = "2.4g-5-2432-22")]
    Variant2Period4g5243222,
    #[serde(rename = "2.4g-6-2437-22")]
    Variant2Period4g6243722,
    #[serde(rename = "2.4g-7-2442-22")]
    Variant2Period4g7244222,
    #[serde(rename = "2.4g-8-2447-22")]
    Variant2Period4g8244722,
    #[serde(rename = "2.4g-9-2452-22")]
    Variant2Period4g9245222,
    #[serde(rename = "2.4g-10-2457-22")]
    Variant2Period4g10245722,
    #[serde(rename = "2.4g-11-2462-22")]
    Variant2Period4g11246222,
    #[serde(rename = "2.4g-12-2467-22")]
    Variant2Period4g12246722,
    #[serde(rename = "2.4g-13-2472-22")]
    Variant2Period4g13247222,
    #[serde(rename = "5g-32-5160-20")]
    Variant5g32516020,
    #[serde(rename = "5g-34-5170-40")]
    Variant5g34517040,
    #[serde(rename = "5g-36-5180-20")]
    Variant5g36518020,
    #[serde(rename = "5g-38-5190-40")]
    Variant5g38519040,
    #[serde(rename = "5g-40-5200-20")]
    Variant5g40520020,
    #[serde(rename = "5g-42-5210-80")]
    Variant5g42521080,
    #[serde(rename = "5g-44-5220-20")]
    Variant5g44522020,
    #[serde(rename = "5g-46-5230-40")]
    Variant5g46523040,
    #[serde(rename = "5g-48-5240-20")]
    Variant5g48524020,
    #[serde(rename = "5g-50-5250-160")]
    Variant5g505250160,
    #[serde(rename = "5g-52-5260-20")]
    Variant5g52526020,
    #[serde(rename = "5g-54-5270-40")]
    Variant5g54527040,
    #[serde(rename = "5g-56-5280-20")]
    Variant5g56528020,
    #[serde(rename = "5g-58-5290-80")]
    Variant5g58529080,
    #[serde(rename = "5g-60-5300-20")]
    Variant5g60530020,
    #[serde(rename = "5g-62-5310-40")]
    Variant5g62531040,
    #[serde(rename = "5g-64-5320-20")]
    Variant5g64532020,
    #[serde(rename = "5g-100-5500-20")]
    Variant5g100550020,
    #[serde(rename = "5g-102-5510-40")]
    Variant5g102551040,
    #[serde(rename = "5g-104-5520-20")]
    Variant5g104552020,
    #[serde(rename = "5g-106-5530-80")]
    Variant5g106553080,
    #[serde(rename = "5g-108-5540-20")]
    Variant5g108554020,
    #[serde(rename = "5g-110-5550-40")]
    Variant5g110555040,
    #[serde(rename = "5g-112-5560-20")]
    Variant5g112556020,
    #[serde(rename = "5g-114-5570-160")]
    Variant5g1145570160,
    #[serde(rename = "5g-116-5580-20")]
    Variant5g116558020,
    #[serde(rename = "5g-118-5590-40")]
    Variant5g118559040,
    #[serde(rename = "5g-120-5600-20")]
    Variant5g120560020,
    #[serde(rename = "5g-122-5610-80")]
    Variant5g122561080,
    #[serde(rename = "5g-124-5620-20")]
    Variant5g124562020,
    #[serde(rename = "5g-126-5630-40")]
    Variant5g126563040,
    #[serde(rename = "5g-128-5640-20")]
    Variant5g128564020,
    #[serde(rename = "5g-132-5660-20")]
    Variant5g132566020,
    #[serde(rename = "5g-134-5670-40")]
    Variant5g134567040,
    #[serde(rename = "5g-136-5680-20")]
    Variant5g136568020,
    #[serde(rename = "5g-138-5690-80")]
    Variant5g138569080,
    #[serde(rename = "5g-140-5700-20")]
    Variant5g140570020,
    #[serde(rename = "5g-142-5710-40")]
    Variant5g142571040,
    #[serde(rename = "5g-144-5720-20")]
    Variant5g144572020,
    #[serde(rename = "5g-149-5745-20")]
    Variant5g149574520,
    #[serde(rename = "5g-151-5755-40")]
    Variant5g151575540,
    #[serde(rename = "5g-153-5765-20")]
    Variant5g153576520,
    #[serde(rename = "5g-155-5775-80")]
    Variant5g155577580,
    #[serde(rename = "5g-157-5785-20")]
    Variant5g157578520,
    #[serde(rename = "5g-159-5795-40")]
    Variant5g159579540,
    #[serde(rename = "5g-161-5805-20")]
    Variant5g161580520,
    #[serde(rename = "5g-163-5815-160")]
    Variant5g1635815160,
    #[serde(rename = "5g-165-5825-20")]
    Variant5g165582520,
    #[serde(rename = "5g-167-5835-40")]
    Variant5g167583540,
    #[serde(rename = "5g-169-5845-20")]
    Variant5g169584520,
    #[serde(rename = "5g-171-5855-80")]
    Variant5g171585580,
    #[serde(rename = "5g-173-5865-20")]
    Variant5g173586520,
    #[serde(rename = "5g-175-5875-40")]
    Variant5g175587540,
    #[serde(rename = "5g-177-5885-20")]
    Variant5g177588520,
    #[serde(rename = "6g-1-5955-20")]
    Variant6g1595520,
    #[serde(rename = "6g-3-5965-40")]
    Variant6g3596540,
    #[serde(rename = "6g-5-5975-20")]
    Variant6g5597520,
    #[serde(rename = "6g-7-5985-80")]
    Variant6g7598580,
    #[serde(rename = "6g-9-5995-20")]
    Variant6g9599520,
    #[serde(rename = "6g-11-6005-40")]
    Variant6g11600540,
    #[serde(rename = "6g-13-6015-20")]
    Variant6g13601520,
    #[serde(rename = "6g-15-6025-160")]
    Variant6g156025160,
    #[serde(rename = "6g-17-6035-20")]
    Variant6g17603520,
    #[serde(rename = "6g-19-6045-40")]
    Variant6g19604540,
    #[serde(rename = "6g-21-6055-20")]
    Variant6g21605520,
    #[serde(rename = "6g-23-6065-80")]
    Variant6g23606580,
    #[serde(rename = "6g-25-6075-20")]
    Variant6g25607520,
    #[serde(rename = "6g-27-6085-40")]
    Variant6g27608540,
    #[serde(rename = "6g-29-6095-20")]
    Variant6g29609520,
    #[serde(rename = "6g-31-6105-320")]
    Variant6g316105320,
    #[serde(rename = "6g-33-6115-20")]
    Variant6g33611520,
    #[serde(rename = "6g-35-6125-40")]
    Variant6g35612540,
    #[serde(rename = "6g-37-6135-20")]
    Variant6g37613520,
    #[serde(rename = "6g-39-6145-80")]
    Variant6g39614580,
    #[serde(rename = "6g-41-6155-20")]
    Variant6g41615520,
    #[serde(rename = "6g-43-6165-40")]
    Variant6g43616540,
    #[serde(rename = "6g-45-6175-20")]
    Variant6g45617520,
    #[serde(rename = "6g-47-6185-160")]
    Variant6g476185160,
    #[serde(rename = "6g-49-6195-20")]
    Variant6g49619520,
    #[serde(rename = "6g-51-6205-40")]
    Variant6g51620540,
    #[serde(rename = "6g-53-6215-20")]
    Variant6g53621520,
    #[serde(rename = "6g-55-6225-80")]
    Variant6g55622580,
    #[serde(rename = "6g-57-6235-20")]
    Variant6g57623520,
    #[serde(rename = "6g-59-6245-40")]
    Variant6g59624540,
    #[serde(rename = "6g-61-6255-20")]
    Variant6g61625520,
    #[serde(rename = "6g-65-6275-20")]
    Variant6g65627520,
    #[serde(rename = "6g-67-6285-40")]
    Variant6g67628540,
    #[serde(rename = "6g-69-6295-20")]
    Variant6g69629520,
    #[serde(rename = "6g-71-6305-80")]
    Variant6g71630580,
    #[serde(rename = "6g-73-6315-20")]
    Variant6g73631520,
    #[serde(rename = "6g-75-6325-40")]
    Variant6g75632540,
    #[serde(rename = "6g-77-6335-20")]
    Variant6g77633520,
    #[serde(rename = "6g-79-6345-160")]
    Variant6g796345160,
    #[serde(rename = "6g-81-6355-20")]
    Variant6g81635520,
    #[serde(rename = "6g-83-6365-40")]
    Variant6g83636540,
    #[serde(rename = "6g-85-6375-20")]
    Variant6g85637520,
    #[serde(rename = "6g-87-6385-80")]
    Variant6g87638580,
    #[serde(rename = "6g-89-6395-20")]
    Variant6g89639520,
    #[serde(rename = "6g-91-6405-40")]
    Variant6g91640540,
    #[serde(rename = "6g-93-6415-20")]
    Variant6g93641520,
    #[serde(rename = "6g-95-6425-320")]
    Variant6g956425320,
    #[serde(rename = "6g-97-6435-20")]
    Variant6g97643520,
    #[serde(rename = "6g-99-6445-40")]
    Variant6g99644540,
    #[serde(rename = "6g-101-6455-20")]
    Variant6g101645520,
    #[serde(rename = "6g-103-6465-80")]
    Variant6g103646580,
    #[serde(rename = "6g-105-6475-20")]
    Variant6g105647520,
    #[serde(rename = "6g-107-6485-40")]
    Variant6g107648540,
    #[serde(rename = "6g-109-6495-20")]
    Variant6g109649520,
    #[serde(rename = "6g-111-6505-160")]
    Variant6g1116505160,
    #[serde(rename = "6g-113-6515-20")]
    Variant6g113651520,
    #[serde(rename = "6g-115-6525-40")]
    Variant6g115652540,
    #[serde(rename = "6g-117-6535-20")]
    Variant6g117653520,
    #[serde(rename = "6g-119-6545-80")]
    Variant6g119654580,
    #[serde(rename = "6g-121-6555-20")]
    Variant6g121655520,
    #[serde(rename = "6g-123-6565-40")]
    Variant6g123656540,
    #[serde(rename = "6g-125-6575-20")]
    Variant6g125657520,
    #[serde(rename = "6g-129-6595-20")]
    Variant6g129659520,
    #[serde(rename = "6g-131-6605-40")]
    Variant6g131660540,
    #[serde(rename = "6g-133-6615-20")]
    Variant6g133661520,
    #[serde(rename = "6g-135-6625-80")]
    Variant6g135662580,
    #[serde(rename = "6g-137-6635-20")]
    Variant6g137663520,
    #[serde(rename = "6g-139-6645-40")]
    Variant6g139664540,
    #[serde(rename = "6g-141-6655-20")]
    Variant6g141665520,
    #[serde(rename = "6g-143-6665-160")]
    Variant6g1436665160,
    #[serde(rename = "6g-145-6675-20")]
    Variant6g145667520,
    #[serde(rename = "6g-147-6685-40")]
    Variant6g147668540,
    #[serde(rename = "6g-149-6695-20")]
    Variant6g149669520,
    #[serde(rename = "6g-151-6705-80")]
    Variant6g151670580,
    #[serde(rename = "6g-153-6715-20")]
    Variant6g153671520,
    #[serde(rename = "6g-155-6725-40")]
    Variant6g155672540,
    #[serde(rename = "6g-157-6735-20")]
    Variant6g157673520,
    #[serde(rename = "6g-159-6745-320")]
    Variant6g1596745320,
    #[serde(rename = "6g-161-6755-20")]
    Variant6g161675520,
    #[serde(rename = "6g-163-6765-40")]
    Variant6g163676540,
    #[serde(rename = "6g-165-6775-20")]
    Variant6g165677520,
    #[serde(rename = "6g-167-6785-80")]
    Variant6g167678580,
    #[serde(rename = "6g-169-6795-20")]
    Variant6g169679520,
    #[serde(rename = "6g-171-6805-40")]
    Variant6g171680540,
    #[serde(rename = "6g-173-6815-20")]
    Variant6g173681520,
    #[serde(rename = "6g-175-6825-160")]
    Variant6g1756825160,
    #[serde(rename = "6g-177-6835-20")]
    Variant6g177683520,
    #[serde(rename = "6g-179-6845-40")]
    Variant6g179684540,
    #[serde(rename = "6g-181-6855-20")]
    Variant6g181685520,
    #[serde(rename = "6g-183-6865-80")]
    Variant6g183686580,
    #[serde(rename = "6g-185-6875-20")]
    Variant6g185687520,
    #[serde(rename = "6g-187-6885-40")]
    Variant6g187688540,
    #[serde(rename = "6g-189-6895-20")]
    Variant6g189689520,
    #[serde(rename = "6g-193-6915-20")]
    Variant6g193691520,
    #[serde(rename = "6g-195-6925-40")]
    Variant6g195692540,
    #[serde(rename = "6g-197-6935-20")]
    Variant6g197693520,
    #[serde(rename = "6g-199-6945-80")]
    Variant6g199694580,
    #[serde(rename = "6g-201-6955-20")]
    Variant6g201695520,
    #[serde(rename = "6g-203-6965-40")]
    Variant6g203696540,
    #[serde(rename = "6g-205-6975-20")]
    Variant6g205697520,
    #[serde(rename = "6g-207-6985-160")]
    Variant6g2076985160,
    #[serde(rename = "6g-209-6995-20")]
    Variant6g209699520,
    #[serde(rename = "6g-211-7005-40")]
    Variant6g211700540,
    #[serde(rename = "6g-213-7015-20")]
    Variant6g213701520,
    #[serde(rename = "6g-215-7025-80")]
    Variant6g215702580,
    #[serde(rename = "6g-217-7035-20")]
    Variant6g217703520,
    #[serde(rename = "6g-219-7045-40")]
    Variant6g219704540,
    #[serde(rename = "6g-221-7055-20")]
    Variant6g221705520,
    #[serde(rename = "6g-225-7075-20")]
    Variant6g225707520,
    #[serde(rename = "6g-227-7085-40")]
    Variant6g227708540,
    #[serde(rename = "6g-229-7095-20")]
    Variant6g229709520,
    #[serde(rename = "6g-233-7115-20")]
    Variant6g233711520,
    #[serde(rename = "60g-1-58320-2160")]
    Variant60g1583202160,
    #[serde(rename = "60g-2-60480-2160")]
    Variant60g2604802160,
    #[serde(rename = "60g-3-62640-2160")]
    Variant60g3626402160,
    #[serde(rename = "60g-4-64800-2160")]
    Variant60g4648002160,
    #[serde(rename = "60g-5-66960-2160")]
    Variant60g5669602160,
    #[serde(rename = "60g-6-69120-2160")]
    Variant60g6691202160,
    #[serde(rename = "60g-9-59400-4320")]
    Variant60g9594004320,
    #[serde(rename = "60g-10-61560-4320")]
    Variant60g10615604320,
    #[serde(rename = "60g-11-63720-4320")]
    Variant60g11637204320,
    #[serde(rename = "60g-12-65880-4320")]
    Variant60g12658804320,
    #[serde(rename = "60g-13-68040-4320")]
    Variant60g13680404320,
    #[serde(rename = "60g-17-60480-6480")]
    Variant60g17604806480,
    #[serde(rename = "60g-18-62640-6480")]
    Variant60g18626406480,
    #[serde(rename = "60g-19-64800-6480")]
    Variant60g19648006480,
    #[serde(rename = "60g-20-66960-6480")]
    Variant60g20669606480,
    #[serde(rename = "60g-25-61560-6480")]
    Variant60g25615606480,
    #[serde(rename = "60g-26-63720-6480")]
    Variant60g26637206480,
    #[serde(rename = "60g-27-65880-6480")]
    Variant60g27658806480,
}

impl Default for RfChannel {
    fn default() -> RfChannel {
        Self::Variant2Period4g1241222
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoeMode {
    #[serde(rename = "pd")]
    Pd,
    #[serde(rename = "pse")]
    Pse,
}

impl Default for PoeMode {
    fn default() -> PoeMode {
        Self::Pd
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PoeType {
    #[serde(rename = "type1-ieee802.3af")]
    Type1Ieee802Period3af,
    #[serde(rename = "type2-ieee802.3at")]
    Type2Ieee802Period3at,
    #[serde(rename = "type3-ieee802.3bt")]
    Type3Ieee802Period3bt,
    #[serde(rename = "type4-ieee802.3bt")]
    Type4Ieee802Period3bt,
    #[serde(rename = "passive-24v-2pair")]
    Passive24v2pair,
    #[serde(rename = "passive-24v-4pair")]
    Passive24v4pair,
    #[serde(rename = "passive-48v-2pair")]
    Passive48v2pair,
    #[serde(rename = "passive-48v-4pair")]
    Passive48v4pair,
}

impl Default for PoeType {
    fn default() -> PoeType {
        Self::Type1Ieee802Period3af
    }
}
