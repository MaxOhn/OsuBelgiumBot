mod builders;
mod datetime;
pub mod discord;
mod ext;
pub mod matcher;
pub mod numbers;

pub use builders::author::Author;
pub use builders::embed::EmbedBuilder;
pub use builders::footer::Footer;
pub use builders::message::MessageBuilder;
pub use ext::ApplicationCommandExt;
use twilight_model::id::{ChannelId, GuildId, RoleId};

// Colors
pub const DARK_GREEN: u32 = 0x1F8B4C;
pub const RED: u32 = 0xE74C3C;

// Server ID
pub const SERVER_ID: GuildId = GuildId(277469642908237826);

// Channel IDs
pub const GENERAL_CHANNEL: ChannelId = ChannelId(277469642908237826);
pub const YOUTUBE_REQUEST_CHANNEL: ChannelId = ChannelId(732257710283685948);
pub const OSUVS_CHANNEL: ChannelId = ChannelId(748683685204721729);
pub const APPROVE_CHANNEL: ChannelId = ChannelId(438410203977744394);

// Role IDs
pub const OSU_ROLE_ID: RoleId = RoleId(277473888173162497);
pub const UNCHECKED_ROLE_ID: RoleId = RoleId(326390404620746752);
pub const TOP_ROLE_ID: RoleId = RoleId(438450781142908929);
pub const VC_ROLE_ID: RoleId = RoleId(673633138207096833);

// Message field sizes
pub const DESCRIPTION_SIZE: usize = 2048;
pub const FIELD_VALUE_SIZE: usize = 1024;

// osu!
pub const OSU_BASE: &str = "https://osu.ppy.sh/";
pub const MAP_THUMB_URL: &str = "https://b.ppy.sh/thumb/";
pub const AVATAR_URL: &str = "https://a.ppy.sh/";
pub const HUISMETBENEN: &str = "https://api.huismetbenen.nl/";
pub const OSEKAI_MEDAL_API: &str = "https://osekai.net/medals/apiv2/";
pub const OSU_DAILY_API: &str = "https://osudaily.net/api/";

// twitch
pub const TWITCH_BASE: &str = "https://www.twitch.tv/";
pub const TWITCH_OAUTH: &str = "https://id.twitch.tv/oauth2/token";
pub const TWITCH_STREAM_ENDPOINT: &str = "https://api.twitch.tv/helix/streams";
pub const TWITCH_USERS_ENDPOINT: &str = "https://api.twitch.tv/helix/users";
pub const TWITCH_VIDEOS_ENDPOINT: &str = "https://api.twitch.tv/helix/videos";

// discord
pub const DISCORD_CDN: &str = "https://cdn.discordapp.com/";

// Error messages
pub const GENERAL_ISSUE: &str = "Something went wrong, blame joshi";
pub const OSU_API_ISSUE: &str = "Some issue with the osu api, blame joshi";
pub const OSU_WEB_ISSUE: &str = "Some issue with the osu website, DDoS protection? blame peppy";
pub const OSEKAI_ISSUE: &str = "Some issue with the osekai api, blame joshi";
pub const HUISMETBENEN_ISSUE: &str = "Some issue with the huismetbenen api, blame joshi";
pub const OSU_DAILY_ISSUE: &str = "Some issue with the osudaily api, blame joshi";
pub const OSUSTATS_API_ISSUE: &str = "Some issue with the osustats api, blame joshi";
pub const TWITCH_API_ISSUE: &str = "Some issue with the twitch api, blame joshi";

// Misc
pub const OWNER_USER_ID: u64 = 219905108316520448;
pub const SYMBOLS: [&str; 6] = ["♔", "♕", "♖", "♗", "♘", "♙"];
pub const DATE_FORMAT: &str = "%F %T";
pub const INVITE_LINK: &str = "https://discord.com/api/oauth2/authorize?client_id=297073686916366336&permissions=36776045632&scope=bot%20applications.commands";
pub const BATHBOT_WORKSHOP: &str = "https://discord.gg/n9fFstG";
pub const BATHBOT_WORKSHOP_ID: u64 = 741040473476694159;
