use std::string::String;
use sha2::{Sha256, Digest};

pub fn generate_prediction(index: u128) -> (String, String) {
    // Special case for index 0 - GENESIS card
    if index == 0 {
        return (
            "Every journey begins with a single step".to_string(),
            "千里之行，始于足下".to_string()
        );
    }
    
    let future = [
        "Today", "Tomorrow", "This Morning", "This Afternoon", "This Evening", "Tonight", 
        "Next Monday", "Next Tuesday", "Next Wednesday", "Next Thursday", "Next Friday", 
        "Next Saturday", "Next Sunday", "This Weekend", "Next Week", "In Two Days", 
        "By Noon", "At Night", "Before Bed", "After Breakfast", "During Lunch", 
        "After Work", "Before Dinner", "At Sunrise", "At Sunset", "In the Morning", 
        "In the Evening", "This Night", "Next Morning", "Next Evening", "On Monday", 
        "On Tuesday", "On Wednesday", "On Thursday", "On Friday", "On Saturday", 
        "On Sunday", "This Week", "Next Weekend", "In Three Days", "By Evening", 
        "After Lunch", "Before Noon", "At Midnight", "At Dawn", "At Dusk", 
        "Before Sunrise", "After Sunset", "During Breakfast", "During Dinner", 
        "After Midnight", "Before Midnight", "In Four Days", "This Month", 
        "Next Month", "By Tomorrow", "By Tonight", "This Day", "Next Night"
    ];
    let future_cn = [
        "今天", "明天", "今天早上", "今天下午", "今天晚上", "今晚", 
        "下周一", "下周二", "下周三", "下周四", "下周五", 
        "下周六", "下周日", "本周末", "下周", "两天后", 
        "中午前", "夜里", "睡前", "早餐后", "午餐时", 
        "下班后", "晚餐前", "日出时", "日落时", "早上", 
        "晚上", "今夜", "明早", "明晚", "周一", 
        "周二", "周三", "周四", "周五", "周六", 
        "周日", "本周", "下个周末", "三天后", "傍晚前", 
        "午饭后", "中午前", "午夜", "黎明", "黄昏", 
        "日出前", "日落后", "早餐时", "晚餐时", 
        "午夜后", "午夜前", "四天后", "本月", 
        "下月", "明天前", "今晚前", "今天", "明夜"
    ];
    let subject = [
        "You", "Your Friend", "Your Boss", "Your Colleague", "Your Neighbor", "Your Partner", 
        "Your Family", "Your Team", "Your Rival", "A Stranger", "Your Mentor", 
        "Your Teacher", "Your Coworker", "Your Sibling", "Your Parent", "Your Child", 
        "Your Best Friend", "Your Enemy", "Your Classmate", "Your Manager", 
        "Your Client", "Your Roommate", "Your Spouse", "Your Crush", "Your Ex", 
        "Your Doctor", "Your Accountant", "Your Coach", 
        "Your Barber", "Your Driver", "Your Waiter", "Your Barista", "Your Therapist", 
        "Your Neighbor's Dog", "Your Cat", "Your Dog", "Your Pet", "Your Landlord", 
        "Your Tenant", "Your Mailman", "Your Grocer", "Your Pharmacist", "Your Banker", 
        "Your Mechanic", "Your Plumber", "Your Electrician", "Your Dentist", 
        "Your Hairdresser", "Your Tailor", "Your Cleaner", "Your Gardener", 
        "Your Babysitter", "Your Nanny", "Your Tutor", "Your Advisor", 
        "Your Accountant", "Your Chef", "Your Assistant"
    ];
    let subject_cn = [
        "你", "你的朋友", "你的老板", "你的同事", "你的邻居", "你的伴侣", 
        "你的家人", "你的团队", "你的对手", "陌生人", "你的导师", 
        "你的老师", "你的同事", "你的兄弟姐妹", "你的父母", "你的孩子", 
        "你最好的朋友", "你的敌人", "你的同学", "你的经理", 
        "你的客户", "你的室友", "你的配偶", "你的暗恋对象", "你的前任", 
        "你的医生", "你的会计", "你的教练", 
        "你的理发师", "你的司机", "你的服务员", "你的咖啡师", "你的治疗师", 
        "你邻居的狗", "你的猫", "你的狗", "你的宠物", "你的房东", 
        "你的租客", "你的邮递员", "你的杂货商", "你的药剂师", "你的银行家", 
        "你的技工", "你的水管工", "你的电工", "你的牙医", 
        "你的美发师", "你的裁缝", "你的清洁工", "你的园丁", 
        "你的保姆", "你的保育员", "你的家教", "你的顾问", 
        "你的会计", "你的厨师", "你的助理"
    ];
    let verb = [
        "Will Triumph", "Will Flop", "Will Dominate", "Will Stumble", "Will Discover", "Will Reveal", 
        "Will Cackle", "Will Weep", "Will Hustle", "Will Chill", "Will Wander", "Will Linger", 
        "Will Clash", "Will Spark", "Will Soothe", "Will Snub", "Will Rally", "Will Grumble", 
        "Will Scheme", "Will Ditch", "Will Invent", "Will Wreck", "Will Mend", "Will Shatter", 
        "Will Snag", "Will Trade", "Will Holler", "Will Ping", "Will Ghost", "Will Recall", 
        "Will Blank", "Will Push", "Will Bail", "Will Launch", "Will Wrap", "Will Stall", 
        "Will Sprint", "Will Unwind", "Will Freak", "Will Sizzle", "Will Feast", "Will Crash", 
        "Will Groove", "Will Croon", "Will Brawl", "Will Grovel", "Will Absolve", "Will Cram", 
        "Will Rage", "Will Stash", "Will Splurge", "Will Rake", "Will Misplace", "Will Uncover", 
        "Will Charm", "Will Fumble", "Will Shine", "Will Flirt", "Will Roast", "Will Swagger"
    ];
    let verb_cn = [
        "将成功", "会失败", "将主宰", "会绊倒", "会发现", "会揭示", 
        "会大笑", "会哭泣", "会奋斗", "会放松", "会徘徊", "会逗留", 
        "会冲突", "会激发", "会安慰", "会冷落", "会集结", "会抱怨", 
        "会策划", "会抛弃", "会发明", "会破坏", "会修复", "会粉碎", 
        "会抓住", "会交易", "会呼喊", "会联系", "会消失", "会回忆", 
        "会空白", "会推动", "会退出", "会启动", "会结束", "会拖延", 
        "会冲刺", "会放松", "会惊慌", "会滋滋作响", "会盛宴", "会崩溃", 
        "会律动", "会歌唱", "会争吵", "会卑躬屈膝", "会赦免", "会死记", 
        "会愤怒", "会藏匿", "会挥霍", "会耙拉", "会遗失", "会发现", 
        "会吸引", "会失误", "会闪耀", "会调情", "会烤制", "会炫耀"
    ];



    let mut hasher = Sha256::new();
    hasher.update(index.to_le_bytes());
    let hash = hasher.finalize();

    // Используем разные части хеша для определения типа предсказания
    let prediction_type = hash[24] % 2; // 0 или 1 для двух типов

    match prediction_type {
        0 => {
            // Персональные предсказания (существующие)
            let f_idx = (u64::from_le_bytes(hash[0..8].try_into().unwrap()) % future.len() as u64) as usize;
            let s_idx = (u64::from_le_bytes(hash[8..16].try_into().unwrap()) % subject.len() as u64) as usize;
            let v_idx = (u64::from_le_bytes(hash[16..24].try_into().unwrap()) % verb.len() as u64) as usize;

            let f = &future[f_idx];
            let s = &subject[s_idx];
            let v = &verb[v_idx];
            let f_cn = &future_cn[f_idx];
            let s_cn = &subject_cn[s_idx];
            let v_cn = &verb_cn[v_idx];
            (
                format!("{} • {} • {}", f, s, v),
                format!("{} • {} • {}", f_cn, s_cn, v_cn)
            )
        },
        _ => {
            // Общие пожелания (новые) - разбитые на компоненты
            let general_beginnings = [
                "Good fortune", "The stars", "Luck smiles", "Your wish", 
                "The universe", "Your dreams", "A surprise", "Change brings", 
                "Your patience", "A breakthrough", "The path", "Your efforts", 
                "An angel", "Magic surrounds", "Your heart", "A golden chance", 
                "The tides", "Your strength", "A blessing", "The road", 
                "Your intuition", "A pure wish", "The cosmos", "Your energy", 
                "A door opens", "Your power", "Your courage", "Perfect clarity", 
                "Your seeds", "Your true self", "Good luck", "Your inner light", 
                "Your gifts", "Support surrounds", "Wisdom finds", "Your spark", 
                "A bridge", "Harmony", "Your resilience", "A gentle touch", 
                "Your love", "Your potential", "A sacred time", "Peace", 
                "Your journey", "Celebration", "Truth reveals", "Your spirit", 
                "Connection", "Abundance flows", "Your voice", "Your story", 
                "Healing begins", "Your light"
            ];
            let general_beginnings_cn = [
                "好运", "星辰", "幸运微笑", "你的愿望", 
                "宇宙", "你的梦想", "惊喜", "变化带来", 
                "你的耐心", "突破", "道路", "你的努力", 
                "天使", "魔法围绕", "你的心", "黄金机会", 
                "潮流", "你的力量", "祝福", "道路", 
                "你的直觉", "纯净愿望", "宇宙", "你的能量", 
                "门打开", "你的力量", "你的勇气", "完美清晰", 
                "你的种子", "真实的你", "好运", "内在光芒", 
                "你的天赋", "支持围绕", "智慧找到", "你的火花", 
                "桥梁", "和谐", "你的韧性", "温柔触碰", 
                "你的爱", "你的潜力", "神圣时刻", "和平", 
                "你的旅程", "庆祝", "真相揭示", "你的精神", 
                "连接", "丰盛流动", "你的声音", "你的故事", 
                "治愈开始", "你的光芒"
            ];
            
            let general_middles = [
                "awaits you", "align for you", "upon you", "comes true", 
                "helps you", "are near", "finds you", "good news", 
                "is rewarded", "is close", "is bright", "bears fruit", 
                "watches you", "surrounds you", "is within reach", "appears", 
                "turns for you", "shines through", "approaches", "leads to treasure", 
                "guides you", "manifests", "rewards you", "attracts miracles", 
                "unexpectedly", "transforms", "is celebrated", "arrives", 
                "blooms", "is recognized", "washes over you", "illuminates", 
                "is discovered", "embraces you", "finds you", "ignites", 
                "appears", "is here", "becomes your power", "reminds you", 
                "returns multiplied", "unfolds", "begins", "is yours", 
                "takes a turn", "awaits", "reveals itself", "soars", 
                "deepens", "flows freely", "is heard", "reaches its peak", 
                "is happening", "touches lives"
            ];
            let general_middles_cn = [
                "等待你", "为你排列", "对你", "成真", 
                "帮助你", "就在附近", "找到你", "好消息", 
                "得到回报", "即将到来", "明亮", "结果", 
                "注视你", "围绕你", "触手可及", "出现", 
                "为你转向", "闪耀", "接近", "通向宝藏", 
                "引导你", "实现", "奖励你", "吸引奇迹", 
                "意想不到", "转变", "被赞美", "到来", 
                "绽放", "被认可", "冲刷你", "照亮", 
                "被发现", "拥抱你", "找到你", "点燃", 
                "出现", "在这里", "成为你的力量", "提醒你", 
                "成倍回报", "展开", "开始", "是你的", 
                "转向", "等待", "揭示自己", "翱翔", 
                "加深", "自由流动", "被听到", "达到顶峰", 
                "正在发生", "触动生命"
            ];
            
            let general_endings = [
                "with perfect timing", "beyond dreams", "in harmony", "with ease", 
                "unexpectedly", "when least expected", "with joy", "miraculously", 
                "beyond measure", "profoundly", "beautifully", "with fulfillment", 
                "with protection", "infinitely", "with satisfaction", "with radiance", 
                "powerfully", "brilliantly", "with blessings", "with treasures", 
                "accurately", "with pure intent", "justly", "magnetically", 
                "in alignment", "transformatively", "universally", "clearly", 
                "beautifully", "genuinely", "abundantly", "warmly", 
                "uniquely", "unconditionally", "wisely", "creatively", 
                "where needed", "in balance", "unbreakably", "gently", 
                "exponentially", "naturally", "sacredly", "peacefully", 
                "beautifully", "honorably", "profoundly", "spiritually", 
                "deeply", "limitlessly", "authentically", "perfectly", 
                "gently", "transformatively"
            ];
            let general_endings_cn = [
                "在完美时机", "超越梦想", "和谐中", "轻松地", 
                "意想不到", "最不期待时", "带着喜悦", "奇迹般", 
                "超越衡量", "深刻地", "美丽地", "满足地", 
                "受保护", "无限地", "满足地", "光芒四射", 
                "强大地", "光辉地", "受祝福", "带着宝藏", 
                "准确地", "纯净意图", "公正地", "磁性般", 
                "完美排列", "转变地", "普遍地", "清晰地", 
                "美丽地", "真实地", "丰盛地", "温暖地", 
                "独特地", "无条件地", "智慧地", "创意地", 
                "需要时", "平衡中", "不可打破", "温柔地", 
                "指数级", "自然地", "神圣地", "和平地", 
                "美丽地", "光荣地", "深刻地", "精神地", 
                "深深地", "无限地", "真实地", "完美地", 
                "温柔地", "转变地"
            ];
            
            let beg_idx = (u64::from_le_bytes(hash[0..8].try_into().unwrap()) % general_beginnings.len() as u64) as usize;
            let mid_idx = (u64::from_le_bytes(hash[8..16].try_into().unwrap()) % general_middles.len() as u64) as usize;
            let end_idx = (u64::from_le_bytes(hash[16..24].try_into().unwrap()) % general_endings.len() as u64) as usize;
            
            (
                format!("{} {} {}", general_beginnings[beg_idx], general_middles[mid_idx], general_endings[end_idx]),
                format!("{} {} {}", general_beginnings_cn[beg_idx], general_middles_cn[mid_idx], general_endings_cn[end_idx])
            )
        }
    }
}