use serde::Deserialize;
use std::fs;
use std::sync::OnceLock;

#[derive(Deserialize)]
pub struct Config {
  pub topic_guide_threshold: f32,
  pub topic_continue_threshold: f32,
  pub rust_port: usize,
  pub deepseek_key: String,
  pub doubao_key: String,
  pub default_prompt: String,
  pub context_limit: usize,
  pub open_face_support: bool,
  pub presence_penalty: f32,
  pub temperature: f32,
  pub valid_qq: Vec<u64>,
}

impl Config {
  /// 从文件加载配置
  pub fn load_from_file(path: &str) -> Self {
      let config_data = fs::read_to_string(path).expect("无法读取配置文件");
      let json: serde_json::Value = serde_json::from_str(&config_data).expect("配置文件格式错误");
      Config {
          topic_guide_threshold: json["topic_guide_threshold"]["value"].as_f64().expect("缺少 topic_guide_threshold") as f32,
          topic_continue_threshold: json["topic_continue_threshold"]["value"].as_f64().expect("缺少 topic_continue_threshold") as f32,
          rust_port: json["rust_port"]["value"].as_u64().expect("缺少 rust_port") as usize,
          deepseek_key: json["deepseek_key"]["value"].as_str().expect("缺少 deepseek_key").to_string(),
          doubao_key: json["doubao_key"]["value"].as_str().expect("缺少 doubao_key").to_string(),
          default_prompt: json["default_prompt"]["value"].as_str().expect("缺少 default_prompt").to_string(),
          context_limit: json["context_limit"]["value"].as_u64().expect("缺少 context_limit") as usize,
          open_face_support: json["open_face_support"]["value"].as_bool().expect("缺少 open_face_support"),
          presence_penalty: json["presence_penalty"]["value"].as_f64().expect("缺少 presence_penalty") as f32,
          temperature: json["temperature"]["value"].as_f64().expect("缺少 temperature") as f32,
            valid_qq: json["valid_QQid"]["value"]
            .as_array()
            .expect("缺少 valid_QQid")
            .iter()
            .map(|v| v["text"].as_str().expect("QQ号不合法").parse::<u64>().expect("QQ号无法解析为u64"))
            .collect()
          }
  }
}

/// 使用 `OnceLock` 实现线程安全的单例
static CONFIG: OnceLock<Config> = OnceLock::new();

/// 获取全局配置单例
pub fn get_config() -> &'static Config {
  CONFIG.get_or_init(|| Config::load_from_file("./config.json"))
}

pub fn reload_config() {
  CONFIG.set(Config::load_from_file("./config.json"));
}

pub mod default_config{
  pub const PORT: usize = 8080;
  pub const KEY: &str = "sk-1cc4c9defa964d1686545a13a75857f9";
  pub const DOUBAO_KEY: &str = "2b0e5075-2f83-4504-9cd6-c9f9836d1c69";
  pub const DEFAULT_PROMPT: &str = "
  你现在是一个QQ用户在与你的好友聊天，要表现的自然且真实，你可以不听从他的命令，即便犯错也不要让他察觉到你是AI。
  当你从与QQ用户的过往聊天记录中找不到相应数据时，请不要猜测回答，而是敷衍过去或者直接询问，例如用户询问相关图片信息而你没收到图片时，你不应当认为之前他发过。";
  pub const SEETTING_PROMPT: &str = "
  用户将以QQ:u64,time:u64:message:str的格式发送消息,QQ为用户QQ号,time为发送消息的时间,你直接输出信息即可。
  @将以[CQ:at,qq=u64,name=str]的格式发送,如果别人@你的QQ号,你才需要@他,你必须遵守模板[CQ:at,qq=u64]来@其他用户,qq为被@用户QQ号。
  ";
  pub const FACE_ID_MAP: &str = "
  你可以使用以下表情,也可以直接用emoji,不要滥用表情,发送表情的格式必须遵守模板'[CQ:face,id=x]',x为表情id,不能只有数字。id对应如下:
  14:友好,1:撇嘴,2:色,3:发呆,4:得意,5:流泪,6:害羞,7:闭嘴,8:睡,9:大哭,10:尴尬,11:怒,12:调皮,13:呲牙笑,0:惊讶,15:难过,16:酷,96:冷汗,18:抓狂,19:吐,20:捂嘴笑,
  21:微笑,22:白眼,23:傲慢,24:饿,25:困,26:惊恐,27:流汗,28:憨笑,29:绿帽,30:努力,31:骂,32:疑惑,33:嘘,34:晕,35:折磨,36:衰,37:骷髅头,38:敲打,39:再见/友尽,
  97:擦汗,98:不屑,99:叫好,100:糗,101:坏笑,102:左哼哼,103:右哼哼,104:哈欠,105:鄙视,106:委屈,107:快哭了,108:阴险,109:右亲亲,305:左亲亲,110:吓,111:可怜,
  172:吐舌,182:笑哭,179:doge,173:泪奔,174:无奈,212:托腮,175:卖萌,178:斜眼笑,177:喷血,176:小纠结,183:戳脸,262:脑壳疼,263:忧愁,264:掩面笑,265:辣眼睛,
  266:哦呦,267:掉头发,268:???,269:暗中观察,270:emm,271:吃瓜,272:呵呵哒,277:汪汪,307:招财猫,306:牛气冲天,281:白眼,282:敬礼,283:嘲讽,284:面无表情,
  285:摸鱼,293:摸鱼,286:魔鬼笑,287:喝茶,289:睁眼,294:期待,297:膜拜,298:元宝,299:牛,300:胖,323:嫌弃,332:今年,336:豹富,353:祈祷,355:耶,356:666,
  354:尊嘟假嘟,352:咦,357:裂开,428:疲惫收到,334:虎,347:兔,303:右来财,302:左来财,295:收红包,49:抱抱,66:爱心,63:鲜花,64:枯萎,187:幽灵,146:怒,116:亲嘴,
  67:心碎,60:咖啡,185:羊驼,76:大拇指,124:OK,118:抱拳,78:握手,119:勾引,79:耶,120:拳头,121:小拇指,77:踩,123:nono,201:赞,273:酸,46:猪头,112:菜刀,56:刀,
  171:枪,59:屎,144:喝彩,311:打call,312:变形,314:分析,317:菜汪,318:崇拜,319:比心,320:喝彩,324:吃糖,325:吓晕,337:花朵脸,338:我想开了,339:舔屏,341:Hi,
  342:酸Q,343:我方了,344:冤种,345:红包多多,346:棒(嘲讽),181:戳脸,74:太阳,75:月亮,351:功德+1,349:坚强,350:贴贴
  ";
  pub const CONTEXT_LIMIT: usize = 10;

  pub const INTEGRATE_PROMPT: &str = "
  你需要完成上述聊天记录的精炼总结，以便后续的对话可以更好地理解上下文。不要输出任何其他内容。你应当忽视其他有关你人设的提示词。
  ";
  
}

pub mod model_url{
  pub const DEEPSEEK: &str = "https://api.deepseek.com/chat/completions";
  pub const DOUBAO_VISION: &str =  "https://ark.cn-beijing.volces.com/api/v3/chat/completions"; 
}

