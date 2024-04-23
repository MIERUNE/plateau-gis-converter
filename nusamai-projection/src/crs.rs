pub type EpsgCode = u16;

pub const EPSG_WGS84_GEOGRAPHIC_2D: EpsgCode = 4326;
pub const EPSG_WGS84_GEOGRAPHIC_3D: EpsgCode = 4979;
pub const EPSG_WGS84_GEOCENTRIC: EpsgCode = 4978;

// Web Mercator
pub const EPSG_WEB_MERCATOR: EpsgCode = 3857;

/// JGD2011
pub const EPSG_JGD2011_GEOGRAPHIC_2D: EpsgCode = 6668;

/// JGD2011 + JGD2011 (vertical) height
pub const EPSG_JGD2011_GEOGRAPHIC_3D: EpsgCode = 6697;

// JGD2011 / Japan Plane Rectangular CS + JGD2011 (vertical) height
// Note: Only I - XIII are defined (XIV - XIX does not exist)
pub const EPSG_JGD2011_JPRECT_I_JGD2011_HEIGHT: EpsgCode = 10162;
pub const EPSG_JGD2011_JPRECT_II_JGD2011_HEIGHT: EpsgCode = 10163;
pub const EPSG_JGD2011_JPRECT_III_JGD2011_HEIGHT: EpsgCode = 10164;
pub const EPSG_JGD2011_JPRECT_IV_JGD2011_HEIGHT: EpsgCode = 10165;
pub const EPSG_JGD2011_JPRECT_V_JGD2011_HEIGHT: EpsgCode = 10166;
pub const EPSG_JGD2011_JPRECT_VI_JGD2011_HEIGHT: EpsgCode = 10167;
pub const EPSG_JGD2011_JPRECT_VII_JGD2011_HEIGHT: EpsgCode = 10168;
pub const EPSG_JGD2011_JPRECT_VIII_JGD2011_HEIGHT: EpsgCode = 10169;
pub const EPSG_JGD2011_JPRECT_IX_JGD2011_HEIGHT: EpsgCode = 10170;
pub const EPSG_JGD2011_JPRECT_X_JGD2011_HEIGHT: EpsgCode = 10171;
pub const EPSG_JGD2011_JPRECT_XI_JGD2011_HEIGHT: EpsgCode = 10172;
pub const EPSG_JGD2011_JPRECT_XII_JGD2011_HEIGHT: EpsgCode = 10173;
pub const EPSG_JGD2011_JPRECT_XIII_JGD2011_HEIGHT: EpsgCode = 10174;

// JGD2011 / Japan Plane Rectangular CS
pub const EPSG_JGD2011_JPRECT_I: EpsgCode = 6669;
pub const EPSG_JGD2011_JPRECT_II: EpsgCode = 6670;
pub const EPSG_JGD2011_JPRECT_III: EpsgCode = 6671;
pub const EPSG_JGD2011_JPRECT_IV: EpsgCode = 6672;
pub const EPSG_JGD2011_JPRECT_V: EpsgCode = 6673;
pub const EPSG_JGD2011_JPRECT_VI: EpsgCode = 6674;
pub const EPSG_JGD2011_JPRECT_VII: EpsgCode = 6675;
pub const EPSG_JGD2011_JPRECT_VIII: EpsgCode = 6676;
pub const EPSG_JGD2011_JPRECT_IX: EpsgCode = 6677;
pub const EPSG_JGD2011_JPRECT_X: EpsgCode = 6678;
pub const EPSG_JGD2011_JPRECT_XI: EpsgCode = 6679;
pub const EPSG_JGD2011_JPRECT_XII: EpsgCode = 6680;
pub const EPSG_JGD2011_JPRECT_XIII: EpsgCode = 6681;
pub const EPSG_JGD2011_JPRECT_XIV: EpsgCode = 6682;
pub const EPSG_JGD2011_JPRECT_XV: EpsgCode = 6683;
pub const EPSG_JGD2011_JPRECT_XVI: EpsgCode = 6684;
pub const EPSG_JGD2011_JPRECT_XVII: EpsgCode = 6685;
pub const EPSG_JGD2011_JPRECT_XVIII: EpsgCode = 6686;
pub const EPSG_JGD2011_JPRECT_XIX: EpsgCode = 6687;

// JGD2000 / Japan Plane Rectangular CS
pub const EPSG_JGD2000_JPRECT_I: EpsgCode = 2443;
pub const EPSG_JGD2000_JPRECT_II: EpsgCode = 2444;
pub const EPSG_JGD2000_JPRECT_III: EpsgCode = 2445;
pub const EPSG_JGD2000_JPRECT_IV: EpsgCode = 2446;
pub const EPSG_JGD2000_JPRECT_V: EpsgCode = 2447;
pub const EPSG_JGD2000_JPRECT_VI: EpsgCode = 2448;
pub const EPSG_JGD2000_JPRECT_VII: EpsgCode = 2449;
pub const EPSG_JGD2000_JPRECT_VIII: EpsgCode = 2450;
pub const EPSG_JGD2000_JPRECT_IX: EpsgCode = 2451;
pub const EPSG_JGD2000_JPRECT_X: EpsgCode = 2452;
pub const EPSG_JGD2000_JPRECT_XI: EpsgCode = 2453;
pub const EPSG_JGD2000_JPRECT_XII: EpsgCode = 2454;
pub const EPSG_JGD2000_JPRECT_XIII: EpsgCode = 2455;
pub const EPSG_JGD2000_JPRECT_XIV: EpsgCode = 2456;
pub const EPSG_JGD2000_JPRECT_XV: EpsgCode = 2457;
pub const EPSG_JGD2000_JPRECT_XVI: EpsgCode = 2458;
pub const EPSG_JGD2000_JPRECT_XVII: EpsgCode = 2459;
pub const EPSG_JGD2000_JPRECT_XVIII: EpsgCode = 2460;
pub const EPSG_JGD2000_JPRECT_XIX: EpsgCode = 2461;

// Tokyo / Japan Plane Rectangular CS
pub const EPSG_TOKYO_JPRECT_I: EpsgCode = 30161;
pub const EPSG_TOKYO_JPRECT_II: EpsgCode = 30162;
pub const EPSG_TOKYO_JPRECT_III: EpsgCode = 30163;
pub const EPSG_TOKYO_JPRECT_IV: EpsgCode = 30164;
pub const EPSG_TOKYO_JPRECT_V: EpsgCode = 30165;
pub const EPSG_TOKYO_JPRECT_VI: EpsgCode = 30166;
pub const EPSG_TOKYO_JPRECT_VII: EpsgCode = 30167;
pub const EPSG_TOKYO_JPRECT_VIII: EpsgCode = 30168;
pub const EPSG_TOKYO_JPRECT_IX: EpsgCode = 30169;
pub const EPSG_TOKYO_JPRECT_X: EpsgCode = 30170;
pub const EPSG_TOKYO_JPRECT_XI: EpsgCode = 30171;
pub const EPSG_TOKYO_JPRECT_XII: EpsgCode = 30172;
pub const EPSG_TOKYO_JPRECT_XIII: EpsgCode = 30173;
pub const EPSG_TOKYO_JPRECT_XIV: EpsgCode = 30174;
pub const EPSG_TOKYO_JPRECT_XV: EpsgCode = 30175;
pub const EPSG_TOKYO_JPRECT_XVI: EpsgCode = 30176;
pub const EPSG_TOKYO_JPRECT_XVII: EpsgCode = 30177;
pub const EPSG_TOKYO_JPRECT_XVIII: EpsgCode = 30178;
pub const EPSG_TOKYO_JPRECT_XIX: EpsgCode = 30179;
